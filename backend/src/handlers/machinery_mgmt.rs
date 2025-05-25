use crate::custom_types::structs::{AppState, CatalogParams, Machine};
use axum::{extract::Query, extract::State, http::StatusCode, Json};
use deadpool_postgres::{Pool, Status};
use serde_json::json;
use validator::Validate;

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

    let _order_by = query_params.order_by.as_deref();
    let _order_direction = query_params.order_dir.as_deref();
    let _categories = query_params.categories.as_deref();
    let _min_price = query_params.min_price;
    let _max_price = query_params.max_price;

    if let Ok(client) = state.pool.get().await {
        let offset = (page - 1) * page_size;
        let limit = page_size;

        if let Ok(total_rows) = client
            .query_one("SELECT COUNT(*) FROM machines;", &[])
            .await
        {
            let total_items: i64 = total_rows.get(0);

            match client
                .query(
                    "SELECT * FROM machines LIMIT $1 OFFSET $2;",
                    &[&(limit as i64), &(offset as i64)],
                )
                .await
            {
                Ok(rows) => {
                    let machinery_list: Vec<Machine> = rows
                        .into_iter()
                        .map(|row| Machine::build_from_row(&row))
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
