use crate::custom_types::enums::{OrderByField, OrderDirection};
use crate::custom_types::structs::{
    Access, AppState, CatalogParams, Category, DateRange, Location, MachineModel, ModelAndLocation,
};
use crate::helpers::machinery_mgmt::validate_client;
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use axum_extra::extract::Query;
use serde_json::json;
use std::collections::HashMap;
use tokio_postgres::types::ToSql;
use validator::Validate;

#[axum::debug_handler]
pub async fn explore_catalog(
    State(state): State<AppState>,
    query_params: Query<CatalogParams>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Err(e) = query_params.validate() {
        eprintln!("Validation error: {:?}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Invalid input data",
            })),
        );
    }

    let page = query_params.page.unwrap_or(1);
    let page_size = query_params.page_size.unwrap_or(20);

    if let Ok(client) = state.pool.get().await {
        let offset = (page - 1) * page_size;
        let limit = page_size;

        let mut where_clauses: Vec<String> = Vec::new();
        let mut join_clauses: Vec<String> = Vec::new();
        let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut param_idx = 1;

        if let Some(search_term) = &query_params.search {
            where_clauses.push(format!(
                "(mm.name ILIKE ${} OR mm.brand ILIKE ${} OR mm.model ILIKE ${})",
                param_idx,
                param_idx + 1,
                param_idx + 2
            ));

            params.push(Box::new(format!("%{}%", search_term)));
            params.push(Box::new(format!("%{}%", search_term)));
            params.push(Box::new(format!("%{}%", search_term)));

            param_idx += 3;
        }

        if let Some(min_price) = &query_params.min_price {
            where_clauses.push(format!("(price >= ${})", param_idx));

            params.push(Box::new(min_price));
            param_idx += 1;
        }

        if let Some(max_price) = &query_params.max_price {
            where_clauses.push(format!("(price <= ${})", param_idx));

            params.push(Box::new(max_price));
            param_idx += 1;
        }

        if let (Some(min), Some(max)) = (&query_params.min_price, &query_params.max_price) {
            if min > max {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Minimum price cannot be greater than maximum price",
                    })),
                );
            }
        }

        let categories = &query_params.categories;

        if !categories.is_empty() {
            join_clauses
                .push("INNER JOIN machinery_categories mc ON mm.id = mc.model_id".to_string());
            join_clauses.push("INNER JOIN categories c ON mc.category_id = c.id".to_string());
            let category_placeholders: Vec<String> = categories
                .iter()
                .map(|_| {
                    let placeholder = format!("${}", param_idx);
                    param_idx += 1;
                    placeholder
                })
                .collect();

            where_clauses.push(format!("c.name IN ({})", category_placeholders.join(", ")));

            for cat_name in categories {
                params.push(Box::new(cat_name.clone()));
            }
        }

        let where_clause = if where_clauses.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        };

        let order_clause = if let Some(order_by) = &query_params.order_by {
            let order_dir = query_params
                .order_dir
                .as_ref()
                .unwrap_or(&OrderDirection::Asc);

            let direction_str = match order_dir {
                OrderDirection::Asc => "ASC".to_string(),
                OrderDirection::Desc => "DESC".to_string(),
            };

            let order_by_str = match order_by {
                OrderByField::Price => "price".to_string(),
                OrderByField::Rating => "rating".to_string(),
            };

            format!("ORDER BY {} {}", order_by_str, direction_str)
        } else {
            "ORDER BY id ASC".to_string()
        };

        params.push(Box::new(limit as i64));
        params.push(Box::new(offset as i64));

        let limit_param_idx = param_idx;
        let offset_param_idx = param_idx + 1;

        let all_joins = join_clauses.join(" ");

        let select_query = format!(
            "SELECT DISTINCT mm.* FROM machinery_models mm 
            {} {} {} LIMIT ${} OFFSET ${};",
            all_joins, where_clause, order_clause, limit_param_idx, offset_param_idx
        );

        let count_query = if categories.is_empty() {
            format!("SELECT COUNT(*) FROM machinery_models mm {};", where_clause)
        } else {
            format!(
                "SELECT COUNT(DISTINCT mm.id) FROM machinery_models mm {} {};",
                all_joins, where_clause
            )
        };

        let all_params_slice: Vec<&(dyn ToSql + Sync + Send)> =
            params.iter().map(|p| p.as_ref()).collect();

        let count_params: Vec<&(dyn ToSql + Sync)> = all_params_slice
            [..(all_params_slice.len() - 2)]
            .iter()
            .map(|p_ref| *p_ref as &(dyn ToSql + Sync)) // explicit coercion to traits ToSql + Sync
            .collect();

        let select_params: Vec<&(dyn ToSql + Sync)> = all_params_slice
            .iter()
            .map(|p_ref| *p_ref as &(dyn ToSql + Sync))
            .collect();

        match client
            .query_one(&count_query, count_params.as_slice())
            .await
        {
            Ok(count_row) => {
                let total_items: i64 = count_row.get(0);

                match client.query(&select_query, select_params.as_slice()).await {
                    Ok(machinery_rows) => {
                        let mut machinery_list: Vec<MachineModel> = machinery_rows
                            .iter()
                            .map(|row| MachineModel::build_from_row(row))
                            .collect();

                        for machine in machinery_list.iter_mut() {
                            let category_query = "
                                SELECT c.id AS category_id, c.name AS category_name 
                                FROM categories c
                                INNER JOIN machinery_categories mc ON c.id = mc.category_id
                                WHERE mc.model_id = $1;
                            ";

                            if let Ok(category_rows) =
                                client.query(category_query, &[&machine.id]).await
                            {
                                machine.categories = category_rows
                                    .into_iter()
                                    .map(|row| Category {
                                        id: row.get("category_id"),
                                        name: row.get("category_name"),
                                    })
                                    .collect();
                            } else {
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(json!({
                                        "message": "Se ha producido un error interno en el servidor",
                                    })),
                                );
                            }
                        }

                        return (
                            StatusCode::OK,
                            Json(json!({
                                "page": page,
                                "page_size": page_size,
                                "total_items": total_items,
                                "items": machinery_list,
                            })),
                        );
                    }
                    Err(e) => {
                        eprintln!("Error querying database: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error counting items: {:?}", e);
            }
        }
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": "Se ha producido un error interno en el servidor",
        })),
    );
}

pub async fn select_machine(
    State(state): State<AppState>,
    Path(machine_id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Ok(client) = state.pool.get().await {
        let machine_query = "SELECT * FROM machinery_models WHERE id = $1;";

        match client.query_one(machine_query, &[&machine_id]).await {
            Ok(machine_row) => {
                let machine = MachineModel::build_from_row(&machine_row);

                return (
                    StatusCode::OK,
                    Json(json!({
                        "machine": machine,
                    })),
                );
            }

            Err(e) => {
                eprintln!("Error querying the machine: {:?}", e);
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "Machine not found",
                    })),
                );
            }
        }
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": "Error connecting to the database",
        })),
    );
}

pub async fn get_machine_locations(
    State(state): State<AppState>,
    Path(machine_id): Path<i32>,
    Json(payload): Json<Access>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Some(invalid_response) = validate_client(&payload.access) {
        return invalid_response;
    }

    if let Ok(client) = state.pool.get().await {
        let locations_query = "
            SELECT l.* FROM machinery_models mm
            INNER JOIN machinery_units mu ON mm.id = mu.model_id 
            INNER JOIN locations l ON mu.location_id = l.id
            WHERE mm.id = $1;
        ";

        if let Ok(location_rows) = client.query(locations_query, &[&machine_id]).await {
            if location_rows.is_empty() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "No locations found for this machine",
                    })),
                );
            }

            let locations: Vec<Location> = location_rows
                .into_iter()
                .map(|row| Location::build_from_row(&row))
                .collect();

            return (
                StatusCode::OK,
                Json(json!({
                    "locations": locations,
                })),
            );
        }
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": "Error connecting to the database",
        })),
    );
}

pub async fn get_unavailable_dates(
    State(state): State<AppState>,
    Query(query_params): Query<ModelAndLocation>,
    Json(payload): Json<Access>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Some(invalid_response) = validate_client(&payload.access) {
        return invalid_response;
    }

    if let Ok(client) = state.pool.get().await {
        let not_available_dates_query = "
            WITH relevant_units AS (SELECT mu.id
                FROM machinery_units mu
                WHERE mu.model_id = $1 AND mu.location_id = $2),
            relevant_rentals AS (SELECT r.machine_id, r.start_date::date AS start_date, (r.end_date + INTERVAL '7 days')::date AS end_date
                FROM rentals r
                WHERE r.status IN ('active', 'pending_payment') AND r.machine_id IN (SELECT id FROM relevant_units)),
            dates AS (SELECT generate_series(start_date, end_date, INTERVAL '1 day')::date AS day, machine_id
                FROM relevant_rentals),
            busy_days AS (SELECT day, COUNT(DISTINCT machine_id) AS busy_units
                FROM dates
                GROUP BY day),
            fully_booked_days AS (SELECT day
                FROM busy_days
                WHERE busy_units = (SELECT COUNT(*) FROM relevant_units)),
            grouped_periods AS (SELECT day, day - INTERVAL '1 day' * ROW_NUMBER() OVER (ORDER BY day) AS grp
                FROM fully_booked_days),
            merged_ranges AS (SELECT MIN(day) AS start_date, MAX(day) AS end_date
                FROM grouped_periods
                GROUP BY grp)
            SELECT start_date, end_date
            FROM merged_ranges
            ORDER BY start_date;
        ";

        let machine_id = query_params.model_id;
        let location_id = query_params.location_id;

        if let Ok(rows) = client
            .query(not_available_dates_query, &[&machine_id, &location_id])
            .await
        {
            if rows.is_empty() {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "not_available_dates": [],
                    })),
                );
            }

            let not_available_dates: Vec<DateRange> = rows
                .into_iter()
                .map(|row| DateRange {
                    start_date: row.get("start_date"),
                    end_date: row.get("end_date"),
                })
                .collect();

            return (
                StatusCode::OK,
                Json(json!({
                    "not_available_dates": not_available_dates,
                })),
            );
        }
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": "Error connecting to the database",
        })),
    );
}
