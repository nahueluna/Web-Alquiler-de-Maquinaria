use crate::custom_types::enums::{OrderByField, OrderDirection};
use crate::custom_types::structs::{AppState, CatalogParams, Location, MachineModel};
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use axum_extra::extract::Query;
use serde_json::json;
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
            "SELECT DISTINCT mm.* FROM machinery_models mm {} {} {} LIMIT ${} OFFSET ${};",
            all_joins, where_clause, order_clause, limit_param_idx, offset_param_idx
        );

        let count_query = if join_clauses.is_empty() {
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
                        let machinery_list: Vec<MachineModel> = machinery_rows
                            .into_iter()
                            .map(|row| MachineModel::build_from_row(&row))
                            .collect();

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
            "message": "Error connecting to the database",
        })),
    );
}

pub async fn select_machine(
    State(state): State<AppState>,
    Path(machine_id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Ok(client) = state.pool.get().await {
        let machine_query = "SELECT * FROM machinery_models WHERE id = $1;";

        let location_query =
            "SELECT locations.id AS location_id, latitude, longitude, street, number, city 
        FROM machinery_models INNER JOIN machinery_units 
            ON machinery_models.id = machinery_units.model_id INNER JOIN locations 
            ON machinery_units.location_id = locations.id 
        WHERE machinery_models.id = $1;";

        match client.query_one(machine_query, &[&machine_id]).await {
            Ok(machine_row) => match client.query(location_query, &[&machine_id]).await {
                Ok(location_rows) => {
                    let machine = MachineModel::build_from_row(&machine_row);
                    let locations: Vec<Location> = location_rows
                        .into_iter()
                        .map(|r| Location::build_from_row(&r))
                        .collect();

                    return (
                        StatusCode::OK,
                        Json(json!({
                            "machine": machine,
                            "locations": locations,
                        })),
                    );
                }
                Err(e) => eprintln!("Error querying locations: {:?}", e),
            },

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
