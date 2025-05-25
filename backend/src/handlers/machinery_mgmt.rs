use crate::custom_types::enums::{OrderByField, OrderDirection};
use crate::custom_types::structs::{AppState, CatalogParams, Location, MachineModel};
use axum::{extract::Path, extract::Query, extract::State, http::StatusCode, Json};
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

    let _categories = query_params.categories.as_deref(); // Will be implemented later

    if let Ok(client) = state.pool.get().await {
        let offset = (page - 1) * page_size;
        let limit = page_size;

        let mut where_clauses: Vec<String> = Vec::new();
        let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut param_idx = 1;

        if let Some(search_term) = &query_params.search {
            where_clauses.push(format!(
                "(name ILIKE ${} OR brand ILIKE ${} OR model ILIKE ${})",
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

        let select_query = format!(
            "SELECT * FROM machinery_models {} {} LIMIT ${} OFFSET ${};",
            where_clause, order_clause, limit_param_idx, offset_param_idx
        );
        let count_query = format!("SELECT COUNT(*) FROM machinery_models {};", where_clause);

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

        if let Ok(count_row) = client
            .query_one(&count_query, count_params.as_slice())
            .await
        {
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
