use crate::constants::INTERNAL_PAYMENT_ID_PREFIX;
use crate::constants::LATE_RETURN_FINE;
use crate::custom_types::enums::*;
use crate::custom_types::structs::*;
use crate::helpers::{auth::*, machinery_mgmt::*};
use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::Query;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Duration;
use chrono::{Local, NaiveDate, NaiveDateTime};
use image::ImageFormat;
use serde_json::json;
use std::{env, fs::File, io::BufWriter, path::PathBuf, collections::HashSet};
use tokio_postgres::{error::SqlState, types::ToSql};
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
                "message": "Ingreso de información inválida",
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
            let trimmed_search_term = search_term.trim();
            let cleaned_search_term = clean_strings(trimmed_search_term);

            where_clauses.push(format!(
                "(mm.name ILIKE ${} OR mm.brand ILIKE ${} OR mm.model ILIKE ${})",
                param_idx,
                param_idx + 1,
                param_idx + 2
            ));

            params.push(Box::new(format!("%{}%", cleaned_search_term)));
            params.push(Box::new(format!("%{}%", cleaned_search_term)));
            params.push(Box::new(format!("%{}%", cleaned_search_term)));

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
                        "message": "El precio mínimo no puede ser mayor que el máximo",
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
            format!("AND {}", where_clauses.join(" AND "))
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

        let exists_clause = "EXISTS
        (SELECT * FROM machinery_units mu2 
        WHERE mu2.model_id = mm.id)";

        let select_query = format!(
            "SELECT DISTINCT mm.* FROM machinery_models mm 
            {} WHERE {} {} {} LIMIT ${} OFFSET ${};",
            all_joins, exists_clause, where_clause, order_clause, limit_param_idx, offset_param_idx
        );

        let count_query = if categories.is_empty() {
            format!(
                "SELECT COUNT(*) FROM machinery_models mm WHERE {} {};",
                exists_clause, where_clause
            )
        } else {
            format!(
                "SELECT COUNT(DISTINCT mm.id) FROM machinery_models mm {} WHERE {} {};",
                all_joins, exists_clause, where_clause
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
                        let result_machinery_list: Result<
                            Vec<MachineModel>,
                            (StatusCode, Json<serde_json::Value>),
                        > = machinery_rows
                            .iter()
                            .map(|row| MachineModel::build_from_row(row))
                            .collect();

                        let mut machinery_list = match result_machinery_list {
                            Ok(list) => list,
                            Err((status, json)) => {
                                return (status, json);
                            }
                        };

                        let all_categories: Vec<String>;

                        if let Ok(all_categories_rows) = client
                            .query("SELECT name FROM categories ORDER BY name ASC;", &[])
                            .await
                        {
                            all_categories = all_categories_rows
                                .iter()
                                .map(|row| row.get("name"))
                                .collect();
                        } else {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({
                                    "message": "Se ha producido un error interno al intentar obtener las categorías",
                                })),
                            );
                        }

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
                                "all_categories": all_categories,
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
        let nginx_url = match env::var("NGINX_URL") {
            Ok(url) => url,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "NGINX_URL environment variable is not set. Cannot get machine image.",
                    })),
                );
            }
        };

        let machine_query = "SELECT * FROM machinery_models WHERE id = $1;";

        match client.query_one(machine_query, &[&machine_id]).await {
            Ok(machine_row) => {
                let result_machine = MachineModel::build_from_row(&machine_row);

                let mut machine = match result_machine {
                    Ok(m) => m,
                    Err((status, json)) => {
                        return (status, json);
                    }
                };

                let extra_images_query = "
                    SELECT name FROM model_extra_images WHERE id = $1;";

                if let Ok(extra_images_rows) =
                    client.query(extra_images_query, &[&machine.id]).await
                {
                    machine.extra_images = extra_images_rows
                        .into_iter()
                        .map(|row| {
                            format!(
                                "{}/media/machines/{}.webp",
                                nginx_url,
                                row.get::<_, String>("name")
                            )
                        })
                        .collect();
                } else {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "message": "Se ha producido un error interno al intentar obtener las imágenes adicionales",
                        })),
                    );
                }

                let category_query = "
                    SELECT c.id AS category_id, c.name AS category_name 
                    FROM categories c
                    INNER JOIN machinery_categories mc ON c.id = mc.category_id
                    WHERE mc.model_id = $1;
                ";

                if let Ok(category_rows) = client.query(category_query, &[&machine.id]).await {
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
                        "message": "No se ha encontrado la máquina solicitada",
                    })),
                );
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

pub async fn get_machine_locations(
    State(state): State<AppState>,
    Path(machine_id): Path<i32>,
    Json(payload): Json<Access>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    if (claims.role != 1) && (claims.role != 2) {
        return (
            StatusCode::FORBIDDEN,
            Json(
                json!({"message": "Solo empleados y clientes pueden acceder a esta funcionalidad"}),
            ),
        );
    }

    if let Ok(client) = state.pool.get().await {
        let locations_query = "
            SELECT DISTINCT l.* FROM machinery_models mm
            INNER JOIN machinery_units mu ON mm.id = mu.model_id 
            INNER JOIN locations l ON mu.location_id = l.id
            WHERE mm.id = $1;
        ";

        if let Ok(location_rows) = client.query(locations_query, &[&machine_id]).await {
            if location_rows.is_empty() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "No se han encontrado ubicaciones para la máquina solicitada",
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
            "message": "Se ha producido un error interno en el servidor",
        })),
    );
}

pub async fn get_units_unavailable_dates(
    State(state): State<AppState>,
    Query(query_params): Query<ModelAndLocation>,
    Json(payload): Json<Access>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Some(invalid_response) = validate_client(&payload.access) {
        return invalid_response;
    }

    if let Ok(client) = state.pool.get().await {
        let machine_units_query = "
            SELECT mu.id 
            FROM machinery_units mu
            INNER JOIN machinery_models mm ON mu.model_id = mm.id
            INNER JOIN locations l ON mu.location_id = l.id
            WHERE mm.id = $1 AND mu.location_id = $2;
        ";

        let model_id = query_params.model_id;
        let location_id = query_params.location_id;

        if let Ok(machine_units) = client
            .query(machine_units_query, &[&model_id, &location_id])
            .await
        {
            if machine_units.is_empty() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "No se han encontrado unidades de la máquina en la ubicación solicitada",
                    })),
                );
            }

            let unit_ids: Vec<i32> = machine_units.iter().map(|row| row.get(0)).collect();

            let unavailable_dates_query = "
                SELECT start_date, (end_date + INTERVAL '7 days')::date AS end_date
                FROM rentals r 
                WHERE r.status IN ('active', 'pending_payment') AND (machine_id = $1);
            ";

            let mut machines_info: Vec<UnitAndDates> = Vec::new();

            for unit_id in &unit_ids {
                let machine_unit_info = if let Ok(date_rows) =
                    client.query(unavailable_dates_query, &[&unit_id]).await
                {
                    let unavailable_periods = date_rows
                        .iter()
                        .map(|row| DateRange {
                            start_date: row.get("start_date"),
                            end_date: row.get("end_date"),
                        })
                        .collect();

                    UnitAndDates {
                        unit_id: *unit_id,
                        periods: unavailable_periods,
                    }
                } else {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "message": "Se ha producido un error interno en el servidor",
                        })),
                    );
                };
                machines_info.push(machine_unit_info);
            }

            return (
                StatusCode::OK,
                Json(json!({
                    "units_and_their_unavailable_dates": machines_info,
                })),
            );
        }
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": "Se ha producido un error interno en el servidor",
        })),
    );
}

pub async fn new_model(
    State(state): State<AppState>,
    Json(mut payload): Json<NewModel>,
) -> Response {
    if payload.extra_images.len() > 10 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Cannot upload more than 10 images"})),
        )
            .into_response();
    }

    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role != 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Not enough permissions"})),
        )
            .into_response();
    }

    let mut client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    let transaction = match client.transaction().await {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to create a DB transaction",})),
            )
                .into_response()
        }
    };

    let row = match transaction
        .query_one(
            "INSERT INTO machinery_models
        (name, brand, model, year, policy, description, price, image)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'tempvalue') RETURNING id;",
            &[
                &payload.name,
                &payload.brand,
                &payload.model,
                &payload.year,
                &payload.policy,
                &payload.description,
                &payload.price,
            ],
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            if let Some(db_err) = e.as_db_error() {
                if db_err.code() == &SqlState::UNIQUE_VIOLATION && db_err.message().contains("brand_model_year")
                {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"message": "A model with brand, model and year already exists"})),
                    )
                        .into_response();
                }
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to execute transaction"})),
            )
                .into_response()
        }
    };

    let model_id: i32 = row.get("id");

    //Link the model with the categories
    for cat_name in payload.categories.iter().map(|c| c.to_lowercase()) {
        //Get or create category
        let Ok(row) = transaction
            .query_one(
                "WITH inserted AS (
            INSERT INTO categories (name)
            VALUES ($1)
            ON CONFLICT (name) DO NOTHING
            RETURNING id)
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM categories WHERE name = $1
        LIMIT 1;",
                &[&cat_name],
            )
            .await
        else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to save the categories"})),
            )
                .into_response();
        };
        let cat_id: i32 = row.get("id");
        //Link the category to the model
        if transaction
            .execute(
                "INSERT INTO machinery_categories (model_id, category_id)
            VALUES ($1, $2);",
                &[&model_id, &cat_id],
            )
            .await
            .is_err()
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to save the categories"})),
            )
                .into_response();
        };
    }

    payload.extra_images.insert(0, payload.image); //So that we can process image as another extra_image
    let mut first: bool = true;
    for b64 in &payload.extra_images {
        let Ok(bytes) = STANDARD.decode(b64) else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid base64 in images"})),
            )
                .into_response();
        };

        // Decode into a DynamicImage
        let Ok(image) = image::load_from_memory(&bytes) else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Failed to decode image"})),
            )
                .into_response();
        };

        // Generate 64-character filename
        let name_without_extension = generate_random_string(64);
        let filename = format!("{}.webp", name_without_extension);

        // Save image as JPG
        let filepath = PathBuf::from(format!("media/machines/{}", filename));
        let Ok(file) = File::create(&filepath) else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to save the images"})),
            )
                .into_response();
        };
        let mut writer = BufWriter::new(file);
        if image.write_to(&mut writer, ImageFormat::WebP).is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message":"Failed to save the images"})),
            )
                .into_response();
        }

        //image should be stored with the model
        if first {
            if transaction
                .execute(
                    "UPDATE machinery_models SET image = $1 WHERE id = $2;",
                    &[&name_without_extension, &model_id],
                )
                .await
                .is_err()
            {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message":"Failed to save the images"})),
                )
                    .into_response();
            }
            first = false;
        } else {
            // Store only filename
            if transaction
                .execute(
                    "INSERT INTO model_extra_images (name, id) VALUES ($1, $2)",
                    &[&name_without_extension, &model_id],
                )
                .await
                .is_err()
            {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message":"Failed to save the images"})),
                )
                    .into_response();
            }
        }
    }

    match transaction.commit().await {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(json!({"message":"Model created successfully"})),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to commit transaction"})),
            )
                .into_response()
        }
    };
}

pub async fn new_rental(
    State(state): State<AppState>,
    Json(payload): Json<NewRental>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Some(invalid_response) = validate_client(&payload.access) {
        return invalid_response;
    }

    let token_claims = match get_claims_from_token(&payload.access) {
        Some(claims) => claims,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    };

    if let Err(_) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Ingreso de información inválida",
            })),
        );
    }

    if let Ok(client) = state.pool.get().await {
        let machine_id = payload.machine_id;
        let user_id = token_claims.user_id;
        let start_date = payload.start_date;
        let end_date = payload.end_date;
        let total_price = payload.total_price;

        let unavailable_dates_query = "
            SELECT start_date, (end_date + INTERVAL '7 days')::date AS end_date
            FROM rentals r 
            WHERE r.status IN ('active', 'pending_payment') AND (machine_id = $1);
        ";

        let unavailable_dates = match client.query(unavailable_dates_query, &[&machine_id]).await {
            Ok(rows) => rows
                .iter()
                .map(|row| DateRange {
                    start_date: row.get("start_date"),
                    end_date: row.get("end_date"),
                })
                .collect::<Vec<DateRange>>(),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Se ha producido un error interno en el servidor",
                    })),
                );
            }
        };

        let end_date_with_maintenance_period = end_date + Duration::days(7);

        let is_overlap = unavailable_dates.iter().any(|period| {
            date_is_overlap(
                start_date,
                end_date_with_maintenance_period,
                period.start_date,
                period.end_date,
            )
        });

        let duration_days = (end_date - start_date).num_days();

        if end_date < start_date || duration_days < 7 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "El período indicado no es válido.
                        Debe ser al menos 7 días y la fecha de fin no puede ser anterior a la de inicio.",
                })),
            );
        }

        if is_overlap {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "message": "Las fechas de inicio y fin se superponen con un alquiler existente,
                        considerando el período de mantenimiento planificado",
                })),
            );
        }

        let price_query = "
            SELECT price FROM machinery_models mm 
            INNER JOIN machinery_units mu 
            ON mm.id = mu.model_id
            WHERE mu.id = $1;
        ";

        let price_row = match client.query_one(price_query, &[&machine_id]).await {
            Ok(row) => row,
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "No se ha encontrado la máquina solicitada",
                    })),
                );
            }
        };

        let machine_price: f32 = price_row.get("price");
        let number_of_days = (end_date - start_date).num_days() as f32;

        if total_price != number_of_days * machine_price {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "El precio total no es correcto",
                })),
            );
        }

        let user_query = "SELECT * FROM users WHERE id = $1;";

        if let Err(_) = client.query_one(user_query, &[&user_id]).await {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": "No se ha encontrado al usuario. Verifique su acceso.",
                })),
            );
        };

        let insert_query = "
            INSERT INTO rentals (user_id, machine_id, start_date, end_date, total_price, status)
            VALUES ($1, $2, $3, $4, $5, 'pending_payment')
            RETURNING id;
        ";

        if let Ok(rent_row) = client
            .query_one(
                insert_query,
                &[&user_id, &machine_id, &start_date, &end_date, &total_price],
            )
            .await
        {
            let rental_id: i32 = rent_row.get(0);
            return (
                StatusCode::CREATED,
                Json(json!({
                    "rental_id": rental_id,
                    "user_id": user_id,
                })),
            );
        }
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": "Se ha producido un error interno en el servidor",
        })),
    );
}

pub async fn check_rental_payment(
    State(state): State<AppState>,
    query_params: Query<CheckPayment>,
    Json(payload): Json<RentalIdAndToken>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Some(invalid_response) = validate_client(&payload.access) {
        return invalid_response;
    }

    let claims = match get_claims_from_token(&payload.access) {
        Some(c) => c,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    };

    if let Ok(client) = state.pool.get().await {
        let rental_id = payload.rental_id;
        let payment_id = &query_params.payment_id;
        let user_id = claims.user_id;
        let payment_status = &query_params.status;

        match payment_status {
            PaymentStatus::Approved => {
                let approved_query = "
                    UPDATE rentals 
                    SET status = 'active', payment_id = $1, updated_at = NOW()
                    WHERE id = $2 AND status = 'pending_payment';
                ";

                match client
                    .execute(approved_query, &[&payment_id, &rental_id])
                    .await
                {
                    Ok(rows_updated) => {
                        if rows_updated == 0 {
                            return (
                                StatusCode::NOT_FOUND,
                                Json(json!({
                                    "message": "No se ha encontrado el alquiler pendiente de pago",
                                })),
                            );
                        }

                        if let Ok(user_row) = client
                            .query_one("SELECT * FROM users WHERE id = $1;", &[&user_id])
                            .await
                        {
                            let get_rental_query = "
                            SELECT r.*, l.street, l.number, l.city, mm.brand, mm.name, mm.model 
                            FROM rentals r
                            INNER JOIN machinery_units mu ON r.machine_id = mu.id
                            INNER JOIN machinery_models mm ON mu.model_id = mm.id
                            INNER JOIN locations l ON mu.location_id = l.id
                            WHERE r.id = $1;
                        ";

                            match client.query_one(get_rental_query, &[&rental_id]).await {
                                Ok(rental_row) => {
                                    let rent = RentalInfo {
                                        id: rental_id,
                                        machine_brand: rental_row.get("brand"),
                                        machine_name: rental_row.get("name"),
                                        machine_model: rental_row.get("model"),
                                        start_date: rental_row.get("start_date"),
                                        end_date: rental_row.get("end_date"),
                                        city: rental_row.get("city"),
                                        street: rental_row.get("street"),
                                        number: rental_row.get("number"),
                                        payment_id: payment_id.to_string(),
                                    };

                                    let formatted_start =
                                        rent.start_date.format("%d/%m/%Y").to_string();
                                    let formatted_end =
                                        rent.end_date.format("%d/%m/%Y").to_string();

                                    let user_email: String = user_row.get("email");
                                    let user_name: String = user_row.get("name");

                                    let subject = format!(
                                        "Alquiler n° {} aprobado - Bob el Alquilador",
                                        rental_id
                                    );
                                    let body = format!(
                                        "Hola {},\n\n\
                                    Tu alquiler ha sido aprobado.\n\n\
                                    \n\
                                    Detalles del Alquiler:\n\
                                    \n\n\
                                    Número de alquiler:\t\t\t {}\n\
                                    Período:\t\t\t {} - {}\n\
                                    Máquina:\t\t\t {} {} {}\n\
                                    Ubicación:\t\t\t {}, {}, {}\n\
                                    Identificador del pago:\t\t\t {}\n\n\
                                    \n\n\
                                    Gracias por confiar en nosotros.\n\n\
                                    Saludos cordiales,\n\
                                    El equipo de Bob el Alquilador\n",
                                        user_name,
                                        rental_id,
                                        formatted_start,
                                        formatted_end,
                                        rent.machine_name,
                                        rent.machine_brand,
                                        rent.machine_model,
                                        rent.city,
                                        rent.street,
                                        rent.number,
                                        rent.payment_id,
                                    );

                                    match send_mail(&user_email, &subject, &body) {
                                        Ok(_) => {
                                            return (
                                                StatusCode::OK,
                                                Json(json!({
                                                    "message": "El alquiler ha sido aprobado y el usuario ha sido notificado",
                                                })),
                                            );
                                        }
                                        Err(_) => {
                                            return (
                                                StatusCode::INTERNAL_SERVER_ERROR,
                                                Json(json!({
                                                    "message": "Se ha producido un error al enviar la notificación al usuario",
                                                })),
                                            );
                                        }
                                    }
                                }
                                Err(_) => {
                                    return (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(json!({
                                            "message": "Se ha producido un error al obtener los datos del alquiler",
                                        })),
                                    );
                                }
                            }
                        }
                    }
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({
                                "message": "Se ha producido un error al actualizar el estado del alquiler",
                            })),
                        );
                    }
                }
            }
            PaymentStatus::Rejected => {
                let rejected_query = "
                    UPDATE rentals 
                    SET status = 'failed' 
                    WHERE id = $1 AND status = 'pending_payment';
                ";

                if let Ok(rows_updated) = client.execute(rejected_query, &[&rental_id]).await {
                    if rows_updated == 0 {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(json!({
                                "message": "No se ha encontrado el alquiler pendiente de pago",
                            })),
                        );
                    } else {
                        return (
                            StatusCode::BAD_GATEWAY,
                            Json(json!({
                                "message": "Ha ocurrido un error en el pago por lo que no se pudo realizar el alquiler.",
                            })),
                        );
                    }
                }
            }
            _ => {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({
                        "message": "El pago no ha sido aprobado ni rechazado",
                    })),
                );
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

pub async fn new_unit(State(state): State<AppState>, Json(payload): Json<NewUnit>) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role != 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Not enough permissions"})),
        )
            .into_response();
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    match client
        .execute(
            "INSERT INTO machinery_units
        (serial_number, status, model_id, location_id)
        VALUES ($1, 'available', $2, $3) RETURNING id;",
            &[
                &payload.serial_number,
                &payload.model_id,
                &payload.location_id,
            ],
        )
        .await
    {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(json!({"message": "Unit created successfully"})),
            )
                .into_response()
        }
        Err(e) => {
            let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let mut message = "Failed to save unit";
            if let Some(db_err) = e.as_db_error() {
                match db_err.code() {
                    &SqlState::UNIQUE_VIOLATION => {
                        status_code = StatusCode::BAD_REQUEST;
                        message = "The serial_number is already registered";
                    }
                    &SqlState::FOREIGN_KEY_VIOLATION => {
                        let detail = db_err.message().to_lowercase();
                        if detail.contains("model_id") {
                            status_code = StatusCode::BAD_REQUEST;
                            message = "model_id is invalid";
                        } else if detail.contains("location") {
                            status_code = StatusCode::BAD_REQUEST;
                            message = "location_id is invalid";
                        }
                    }
                    _ => {}
                }
            }
            return (status_code, Json(json!({"message": message}))).into_response();
        }
    };
}

pub async fn get_my_rentals(
    State(state): State<AppState>,
    Json(payload): Json<Access>,
) -> Response {
    let nginx_url = match env::var("NGINX_URL") {
        Ok(e) => e,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "NGINX_URL must be set in the .env file"})),
            )
                .into_response()
        }
    };

    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role != 2 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "The user is not a client"})),
        )
            .into_response();
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    // Get rental_ids that have reviews
    let service_review_ids: HashSet<i32> = match client
        .query("SELECT rental_id FROM service_reviews WHERE user_id = $1", &[&claims.user_id])
        .await
    {
        Ok(rows) => rows.iter().map(|row| row.get(0)).collect(),
        Err(_) => HashSet::new(),
    };

    let machine_review_ids: HashSet<i32> = match client
        .query("SELECT rental_id FROM machine_reviews WHERE user_id = $1", &[&claims.user_id])
        .await
    {
        Ok(rows) => rows.iter().map(|row| row.get(0)).collect(),
        Err(_) => HashSet::new(),
    };

    match client
        .query(
            "SELECT
            rentals.id AS rental_id,
            rentals.return_date,
            rentals.retirement_date,
            rentals.start_date,
            rentals.end_date,
            rentals.total_price,
            rentals.status::TEXT,
            rentals.created_at,
            rentals.updated_at,
            machinery_units.id AS unit_id,
            machinery_units.serial_number AS unit_serial_number,
            machinery_models.id AS model_id,
            machinery_models.name AS model_name,
            machinery_models.brand AS model_brand,
            machinery_models.model AS model_model,
            machinery_models.year AS model_year,
            machinery_models.policy AS model_policy,
            machinery_models.description AS model_description,
            machinery_models.image AS model_image
        FROM rentals
        JOIN machinery_units ON rentals.machine_id = machinery_units.id
        JOIN machinery_models ON machinery_units.model_id = machinery_models.id
        WHERE rentals.user_id = $1;",
            &[&claims.user_id],
        )
        .await
    {
        Ok(rows) => {
            let employees: Vec<MyRentalInfo> = rows
                .iter()
                .map(|row| {
                    let rental_id: i32 = row.get("rental_id");

                    MyRentalInfo {
                        rental_id,
                        return_date: row.get("return_date"),
                        retirement_date: row.get("retirement_date"),
                        start_date: row.get("start_date"),
                        end_date: row.get("end_date"),
                        total_price: row.get("total_price"),
                        status: row.get("status"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                        unit_id: row.get("unit_id"),
                        unit_serial_number: row.get("unit_serial_number"),
                        model_id: row.get("model_id"),
                        model_name: row.get("model_name"),
                        model_brand: row.get("model_brand"),
                        model_model: row.get("model_model"),
                        model_year: row.get("model_year"),
                        model_policy: row.get("model_policy"),
                        model_description: row.get("model_description"),
                        model_image: format!(
                            "{}/media/machines/{}.webp",
                            nginx_url,
                            row.get::<_, String>("model_image")
                        ),
                        days_late: None,
                        percentage_per_late_day: None,
                        has_service_review: service_review_ids.contains(&rental_id),
                        has_machine_review: machine_review_ids.contains(&rental_id),
                    }
                })
                .collect();
            return (StatusCode::OK, Json(json!({"rentals": employees}))).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to get the rentals"})),
            )
                .into_response()
        }
    };
}

pub async fn load_retirement(
    State(state): State<AppState>,
    Json(payload): Json<LoadRetirement>,
) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role == 2 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Not enough permissions"})),
        )
            .into_response();
    }

    let mut client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    let transaction = match client.transaction().await {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to create a DB transaction",})),
            )
                .into_response()
        }
    };

    let row = match transaction
        .query_one(
            "UPDATE rentals
            SET retirement_employee_id = $1,
                 retirement_date = CURRENT_DATE
            WHERE id = $2
            RETURNING machine_id, end_date, status::TEXT;",
            &[&claims.user_id, &payload.rental_id],
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            if e.to_string().contains("unexpected number of rows") {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"message": "rental_id is invalid"})),
                )
                    .into_response();
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to execute transaction"})),
            )
                .into_response();
        }
    };

    let status: String = row.get("status");
    let end_date: NaiveDate = row.get("end_date");
    let machine_id: i32 = row.get("machine_id");

    if status != "active" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The rental is not active"})),
        )
            .into_response();
    }

    if Local::now().naive_local().date() >= end_date {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The rental has expired"})),
        )
            .into_response();
    }

    // Now update the machine status if it's currently 'available'
    match transaction
        .execute(
            "UPDATE machinery_units
             SET status = 'rented'
             WHERE id = $1 AND status = 'available';",
            &[&machine_id],
        )
        .await
    {
        Ok(rows) if rows > 0 => (),
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to execute transaction"})),
            )
                .into_response()
        }
    }

    match transaction.commit().await {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(json!({"message": "Retirement loaded successfully"})),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to commit transaction"})),
            )
                .into_response()
        }
    };
}

pub async fn load_return(
    State(state): State<AppState>,
    Json(payload): Json<LoadReturn>,
) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role == 2 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Not enough permissions"})),
        )
            .into_response();
    }

    let mut client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    let transaction = match client.transaction().await {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to create a DB transaction",})),
            )
                .into_response()
        }
    };

    let row = match transaction
        .query_one(
            "UPDATE rentals
             SET return_employee_id = $1,
                 return_date = CURRENT_DATE,
                 status = 'completed'
             WHERE id = $2
             RETURNING machine_id;",
            &[&claims.user_id, &payload.rental_id],
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            if e.to_string().contains("unexpected number of rows") {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"message": "rental_id is invalid"})),
                )
                    .into_response();
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to update rental"})),
            )
                .into_response();
        }
    };

    let machine_id: i32 = row.get("machine_id");

    let row = match transaction.query_one(
            "SELECT location_id, assigned_at FROM machinery_units WHERE id = $1 AND status = 'rented';",
            &[&machine_id],
        )
        .await {
        Ok(r) => r,
        Err(_) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to find related unit"})),
            )
                .into_response(),
    };

    let location_id: i32 = row.get("location_id");
    let assigned_at: NaiveDateTime = row.get("assigned_at");

    //If the machine was returned to the same location, there is no need to update location history
    if location_id == payload.location_id {
        // Now update the unit
        match transaction
            .execute(
                "UPDATE machinery_units
                 SET status = 'maintenance'
                 WHERE id = $1;",
                &[&machine_id],
            )
            .await
        {
            Ok(rows) if rows > 0 => (),
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to update unit"})),
                )
                    .into_response()
            }
        }
    } else {
        //Update the location history
        match transaction
            .execute(
                "INSERT INTO machinery_location_history
                (unit_id, location_id, assigned_at, unassigned_at) VALUES
                ($1, $2, $3, NOW());",
                &[&machine_id, &location_id, &assigned_at],
            )
            .await
        {
            Ok(rows) if rows > 0 => (),
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to update location history"})),
                )
                    .into_response()
            }
        }
        //Update the unit
        match transaction
            .execute(
                "UPDATE machinery_units
                SET status = 'maintenance',
                assigned_at = NOW(),
                location_id = $1
                WHERE id = $2;",
                &[&payload.location_id, &machine_id],
            )
            .await
        {
            Ok(rows) if rows > 0 => (),
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"message": "location_id is invalid"})),
                )
                    .into_response()
            }
        }
    }

    let row = match transaction
        .query_one(
            "SELECT r.end_date, r.return_date, m.price
        FROM rentals r
        JOIN machinery_units u ON u.id = r.machine_id
        JOIN machinery_models m ON m.id = u.model_id
        WHERE r.id = $1;",
            &[&payload.rental_id],
        )
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to retrieve delay data"})),
            )
                .into_response();
        }
    };

    let end_date: NaiveDate = row.get("end_date");
    let return_date: NaiveDate = row.get("return_date");
    let price: f32 = row.get("price");

    let days_late = (return_date - end_date).num_days().max(0); // avoid negative values
    let fine = days_late as f32 * price * LATE_RETURN_FINE;

    match transaction.commit().await {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(json!({"message": "Return loaded successfully",
                            "fine":fine,
                            "days_late":days_late})),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to commit transaction"})),
            )
                .into_response()
        }
    };
}

pub async fn cancel_rental(
    State(state): State<AppState>,
    Json(payload): Json<CancelRentalInfo>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            );
        }
    };

    let get_rental_query = "
            SELECT * FROM rentals 
            WHERE id = $1 AND status IN ('pending_payment', 'active');
        ";

    if let Ok(rental_row) = client
        .query_one(get_rental_query, &[&payload.rental_id])
        .await
    {
        let rental_start_date: NaiveDate = rental_row.get("start_date");
        let rental_end_date: NaiveDate = rental_row.get("end_date");
        let retirement_date: Option<NaiveDate> = rental_row.get("retirement_date");

        if let Some(_) = retirement_date {
            return (
                StatusCode::BAD_REQUEST,
                Json(
                    json!({"message": "No se puede cancelar un alquiler que ya ha sido retirado"}),
                ),
            );
        }

        if (Local::now().date_naive() > rental_end_date)
            || (Local::now().date_naive() <= rental_start_date)
        {
            if claims.role == 2 {
                let update_client_query = "
                UPDATE rentals 
                SET status = 'cancelled', updated_at = NOW()
                WHERE id = $1 AND user_id = $2 AND status IN ('pending_payment', 'active');
            ";

                match client
                    .execute(update_client_query, &[&payload.rental_id, &claims.user_id])
                    .await
                {
                    Ok(rows_updated) if rows_updated == 1 => {
                        return (
                            StatusCode::OK,
                            Json(json!({"message": "El alquiler ha sido cancelado exitosamente"})),
                        );
                    }
                    Ok(_) => {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(
                                json!({"message": "El alquiler no se ha encontrado o no puede ser cancelado"}),
                            ),
                        );
                    }
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                json!({"message": "Se produjo un error interno al cancelar el alquiler"}),
                            ),
                        );
                    }
                };
            } else {
                let update_query = "
                UPDATE rentals 
                SET status = 'cancelled', notes = $1, updated_at = NOW()
                WHERE id = $2 AND status IN ('pending_payment', 'active');
            ";

                let cancel_reason = payload
                    .reason
                    .as_deref()
                    .unwrap_or("No se indicó un motivo");

                match client
                    .execute(update_query, &[&cancel_reason, &payload.rental_id])
                    .await
                {
                    Ok(rows_updated) if rows_updated == 1 => {
                        let client_id = rental_row.get::<_, i32>("user_id");

                        let get_client_query = "
                        SELECT email, name FROM users WHERE id = $1;
                    ";

                        let user_row = match client.query_one(get_client_query, &[&client_id]).await
                        {
                            Ok(row) => row,
                            Err(_) => {
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(
                                        json!({"message": "Error al obtener los datos del usuario"}),
                                    ),
                                );
                            }
                        };

                        let get_rental_extra_info_query = "
                            SELECT l.street, l.number, l.city, mm.brand, mm.name, mm.model 
                            FROM rentals r
                            INNER JOIN machinery_units mu ON r.machine_id = mu.id
                            INNER JOIN machinery_models mm ON mu.model_id = mm.id
                            INNER JOIN locations l ON mu.location_id = l.id
                            WHERE r.id = $1;
                        ";

                        match client
                            .query_one(get_rental_extra_info_query, &[&payload.rental_id])
                            .await
                        {
                            Ok(extra_info_rental_row) => {
                                let rental_end_date: NaiveDate = rental_row.get("end_date");

                                let user_email: String = user_row.get("email");
                                let user_name: String = user_row.get("name");

                                let subject = format!(
                                    "Alquiler n° {} cancelado - Bob el Alquilador",
                                    payload.rental_id
                                );
                                let body = format!(
                                "Hola {},\n\n\
                    Se le informa que se ha cancelado su alquiler.
                    \n\n\
                    Detalles del Alquiler:
                    \n\
                    Número de alquiler:\t\t\t {}\n\
                    Período:\t\t\t {} - {}\n\
                    Máquina:\t\t\t {} {} {}\n\
                    Ubicación:\t\t\t {}, {}, {}
                    \n\n\
                    En caso de que corresponda, en la brevedad se le reintegrará la totalidad del monto abonado.\n\
                    Nos disculpamos por las molestias ocasionadas.\n\n\
                    \n\
                    Saludos cordiales,\n\
                    El equipo de Bob el Alquilador\n",
                                user_name,
                                payload.rental_id,
                                rental_start_date.format("%d/%m/%Y").to_string(),
                                rental_end_date.format("%d/%m/%Y").to_string(),
                                extra_info_rental_row.get::<_, String>("name"),
                                extra_info_rental_row.get::<_, String>("brand"),
                                extra_info_rental_row.get::<_, String>("model"),
                                extra_info_rental_row.get::<_, String>("city"),
                                extra_info_rental_row.get::<_, String>("street"),
                                extra_info_rental_row.get::<_, String>("number"),
                            );

                                match send_mail(&user_email, &subject, &body) {
                                    Ok(_) => {
                                        return (
                                            StatusCode::OK,
                                            Json(
                                                json!({"message": "El alquiler ha sido cancelado exitosamente y el cliente ha sido notificado"}),
                                            ),
                                        );
                                    }
                                    Err(_) => {
                                        return (
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            Json(
                                                json!({"message": "Error al enviar la notificación al usuario"}),
                                            ),
                                        );
                                    }
                                }
                            }
                            Err(_) => {
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(
                                        json!({"message": "Error al obtener los datos del alquiler"}),
                                    ),
                                );
                            }
                        }
                    }
                    Ok(_) => {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(
                                json!({"message": "El alquiler no se ha encontrado o no puede ser cancelado"}),
                            ),
                        );
                    }
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                json!({"message": "Se produjo un error interno al cancelar el alquiler"}),
                            ),
                        );
                    }
                };
            }
        } else {
            return (
                StatusCode::BAD_REQUEST,
                Json(
                    json!({"message": "No se puede cancelar un alquiler que ya ha comenzado y no ha finalizado"}),
                ),
            );
        }
    }

    return (
        StatusCode::NOT_FOUND,
        Json(json!({"message": "El alquiler no se ha encontrado o ya ha sido cancelado"})),
    );
}

#[axum::debug_handler]
pub async fn get_staff_rentals(
    State(state): State<AppState>,
    Query(query_params): Query<GetRentalQueryParams>,
    Json(payload): Json<Access>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    if (claims.role != 0) && (claims.role != 1) {
        return (
            StatusCode::FORBIDDEN,
            Json(
                json!({"message": "Solo empleados y administradores pueden acceder a esta información"}),
            ),
        );
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            );
        }
    };

    let param_idx = 1;
    let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();

    let where_clause = if let Some(rental_id) = query_params.id {
        let formatted_where_clause = format!("WHERE rentals.id = ${}", param_idx);

        params.push(Box::new(rental_id));

        formatted_where_clause
    } else {
        "".to_string()
    };

    let rental_query = format!(
        "SELECT
            rentals.id AS rental_id,
            rentals.return_date,
            rentals.retirement_date,
            rentals.start_date,
            rentals.end_date,
            rentals.total_price,
            rentals.status::TEXT,
            rentals.created_at,
            rentals.updated_at,
            machinery_units.id AS unit_id,
            machinery_units.serial_number AS unit_serial_number,
            machinery_models.id AS model_id,
            machinery_models.name AS model_name,
            machinery_models.brand AS model_brand,
            machinery_models.model AS model_model,
            machinery_models.year AS model_year,
            machinery_models.policy AS model_policy,
            machinery_models.description AS model_description,
            machinery_models.image AS model_image
        FROM rentals
        INNER JOIN machinery_units ON rentals.machine_id = machinery_units.id
        INNER JOIN machinery_models ON machinery_units.model_id = machinery_models.id
        {}
        ORDER BY rentals.created_at DESC;",
        where_clause
    );

    let all_params_slice: Vec<&(dyn ToSql + Sync + Send)> =
        params.iter().map(|p| p.as_ref()).collect();

    let query_params: Vec<&(dyn ToSql + Sync)> = all_params_slice
        .iter()
        .map(|p_ref| *p_ref as &(dyn ToSql + Sync))
        .collect();

    match client.query(&rental_query, &query_params).await {
        Ok(rows) => {
            let nginx_url = match env::var("NGINX_URL") {
                Ok(url) => url,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "NGINX_URL must be set in the .env file"})),
                    );
                }
            };

            if rows.is_empty() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message": "No se encontraron alquileres"})),
                );
            }

            let rentals: Vec<MyRentalInfo> = rows
                .iter()
                .map(|row| MyRentalInfo {
                    rental_id: row.get("rental_id"),
                    return_date: row.get("return_date"),
                    retirement_date: row.get("retirement_date"),
                    start_date: row.get("start_date"),
                    end_date: row.get("end_date"),
                    total_price: row.get("total_price"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    unit_id: row.get("unit_id"),
                    unit_serial_number: row.get("unit_serial_number"),
                    model_id: row.get("model_id"),
                    model_name: row.get("model_name"),
                    model_brand: row.get("model_brand"),
                    model_model: row.get("model_model"),
                    model_year: row.get("model_year"),
                    model_policy: row.get("model_policy"),
                    model_description: row.get("model_description"),
                    model_image: format!(
                        "{}/media/machines/{}.webp",
                        nginx_url,
                        row.get::<_, String>("model_image")
                    ),
                    days_late: {
                        let status = row.get::<_, String>("status");
                        let today = Local::now().date_naive();
                        let end_date = row.get::<_, NaiveDate>("end_date");
                        if ((status == "active") || (status == "pending_payment"))
                            && (end_date < today)
                        {
                            Some((today - end_date).num_days())
                        } else {
                            None
                        }
                    },
                    percentage_per_late_day: {
                        let status = row.get::<_, String>("status");
                        let today = Local::now().date_naive();
                        let end_date = row.get::<_, NaiveDate>("end_date");
                        if ((status == "active") || (status == "pending_payment"))
                            && (end_date < today)
                        {
                            Some("10% del precio de la máquina por día de retraso".to_string())
                        } else {
                            None
                        }
                    },
                    has_service_review: false,
                    has_machine_review: false,

                })
                .collect();

            return (
                StatusCode::OK,
                Json(json!({
                    "rentals": rentals,
                })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"message": "Se produjo un error interno al intentar obtener los alquileres"}),
                ),
            );
        }
    }
}

pub async fn get_locations(
    State(state): State<AppState>,
    Json(payload): Json<Access>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    if claims.role != 0 && claims.role != 1 {
        return (
            StatusCode::FORBIDDEN,
            Json(
                json!({"message": "Solo empleados y administradores pueden acceder a esta información"}),
            ),
        );
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"message": "Se produjo un error interno al intentar conectarse a la base de datos"}),
                ),
            );
        }
    };

    match client
        .query("SELECT * FROM locations ORDER BY city;", &[])
        .await
    {
        Ok(rows) => {
            let locations: Vec<Location> = rows
                .iter()
                .map(|row| Location::build_from_row(row))
                .collect();
            return (StatusCode::OK, Json(json!({"locations": locations})));
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"message": "Se produjo un error interno al intentar obtener las ubicaciones"}),
                ),
            );
        }
    };
}

pub async fn get_models(State(state): State<AppState>, Json(payload): Json<Access>) -> Response {
    let nginx_url = match env::var("NGINX_URL") {
        Ok(url) => url,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "NGINX_URL must be set in the .env file"})),
            )
                .into_response();
        }
    };

    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role != 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "The user is not an admin"})),
        )
            .into_response();
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    match client.query("SELECT * FROM machinery_models;", &[]).await {
        Ok(rows) => {
            let models: Vec<MachineModel> = rows
                .iter()
                .map(|row| MachineModel {
                    id: row.get("id"),
                    name: row.get("name"),
                    brand: row.get("brand"),
                    model: row.get("model"),
                    year: row.get("year"),
                    policy: row.get("policy"),
                    description: row.get("description"),
                    price: row.get("price"),
                    main_image: format!(
                        "{}/media/machines/{}.webp",
                        nginx_url,
                        row.get::<_, String>("image")
                    ),

                    extra_images: Vec::new(),
                    categories: Vec::new(),
                })
                .collect();
            return (StatusCode::OK, Json(json!({"models": models}))).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to get the models"})),
            )
                .into_response()
        }
    };
}

pub async fn verify_client(
    State(state): State<AppState>,
    Json(payload): Json<VerifyClient>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    if claims.role != 1 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Solo empleados pueden acceder a esta funcionalidad"})),
        );
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            );
        }
    };

    let verify_user_query = "
        SELECT id FROM users
        WHERE email = $1 AND role = 2;
        ";

    if let Ok(rows) = client.query(verify_user_query, &[&payload.email]).await {
        if rows.len() == 0 {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "El email no corresponde a un cliente registrado"})),
            );
        }

        if let Some(row) = rows.first() {
            let user_id = row.get::<_, i32>("id");

            return (StatusCode::OK, Json(json!({"user_id": user_id})));
        }
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"message": "Se produjo un error interno al intentar verificar el usuario"})),
    );
}

pub async fn get_units_by_model_and_location(
    State(state): State<AppState>,
    Json(payload): Json<GetUnitsByLocation>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    if claims.role != 1 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Solo empleados pueden acceder a esta funcionalidad"})),
        );
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            );
        }
    };

    let get_units_query = "
        SELECT mu.id
        FROM machinery_units mu
        INNER JOIN machinery_models mm ON mu.model_id = mm.id
        WHERE mm.id = $1 AND mu.location_id = $2;
    ";

    if let Ok(rows) = client
        .query(get_units_query, &[&payload.model_id, &payload.location_id])
        .await
    {
        if rows.is_empty() {
            return (
                StatusCode::NOT_FOUND,
                Json(
                    json!({"message": "No se encontraron unidades disponibles en la ubicación especificada"}),
                ),
            );
        }

        let units_id: Vec<i32> = rows.iter().map(|row| row.get("id")).collect();
        return (StatusCode::OK, Json(json!({"units_id": units_id})));
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"message": "Se produjo un error interno al intentar obtener las unidades"})),
    );
}

pub async fn validate_rental_dates(
    State(state): State<AppState>,
    Json(payload): Json<ValidateRentalDates>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    if claims.role != 1 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Solo empleados pueden acceder a esta funcionalidad"})),
        );
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            );
        }
    };

    let machine_id = payload.unit_id;
    let start_date = payload.start_date;
    let end_date = payload.end_date;

    let unavailable_dates_query = "
            SELECT start_date, (end_date + INTERVAL '7 days')::date AS end_date
            FROM rentals r 
            WHERE r.status IN ('active', 'pending_payment') AND (machine_id = $1);
        ";

    let unavailable_dates = match client.query(unavailable_dates_query, &[&machine_id]).await {
        Ok(rows) => rows
            .iter()
            .map(|row| DateRange {
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
            })
            .collect::<Vec<DateRange>>(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Se ha producido un error al intentar obtener las fechas de alquileres no disponibles",
                })),
            );
        }
    };

    let end_date_with_maintenance_period = end_date + Duration::days(7);

    let overlaped_date = unavailable_dates.iter().find(|period| {
        date_is_overlap(
            start_date,
            end_date_with_maintenance_period,
            period.start_date,
            period.end_date,
        )
    });

    let duration_days = (end_date - start_date).num_days();

    if end_date < start_date || duration_days < 7 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "El período indicado no es válido. Debe ser al menos 7 días y la fecha de fin no puede ser anterior a la de inicio.",
            })),
        );
    }

    if let Some(date) = overlaped_date {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "message": "Las fechas de inicio y fin se superponen con un alquiler existente, considerando el período de mantenimiento planificado",
                "overlaped_date": date,
            })),
        );
    }

    return (
        StatusCode::OK,
        Json(json!({"message": "Las fechas de alquiler son válidas"})),
    );
}

pub async fn new_in_person_rental(
    State(state): State<AppState>,
    Json(payload): Json<NewInPersonRental>,
) -> (StatusCode, Json<serde_json::Value>) {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            );
        }
    }
    .claims;

    if claims.role != 1 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Solo empleados pueden acceder a esta funcionalidad"})),
        );
    }

    if let Err(_) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Ingreso de información inválida",
            })),
        );
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            );
        }
    };

    let machine_id = payload.machine_id;
    let user_id = payload.user_id;
    let start_date = payload.start_date;
    let end_date = payload.end_date;
    let total_price = payload.total_price;
    let rental_employee_id = claims.user_id;

    let unavailable_dates_query = "
            SELECT start_date, (end_date + INTERVAL '7 days')::date AS end_date
            FROM rentals r 
            WHERE r.status IN ('active', 'pending_payment') AND (machine_id = $1);
        ";

    let unavailable_dates = match client.query(unavailable_dates_query, &[&machine_id]).await {
        Ok(rows) => rows
            .iter()
            .map(|row| DateRange {
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
            })
            .collect::<Vec<DateRange>>(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Se ha producido un error interno en el servidor",
                })),
            );
        }
    };

    let end_date_with_maintenance_period = end_date + Duration::days(7);

    let is_overlap = unavailable_dates.iter().any(|period| {
        date_is_overlap(
            start_date,
            end_date_with_maintenance_period,
            period.start_date,
            period.end_date,
        )
    });

    let duration_days = (end_date - start_date).num_days();

    if end_date < start_date || duration_days < 7 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "El período indicado no es válido. Debe ser al menos 7 días y la fecha de fin no puede ser anterior a la de inicio.",
            })),
        );
    }

    if is_overlap {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "message": "Las fechas de inicio y fin se superponen con un alquiler existente, considerando el período de mantenimiento planificado",
            })),
        );
    }

    let price_query = "
            SELECT price FROM machinery_models mm 
            INNER JOIN machinery_units mu 
            ON mm.id = mu.model_id
            WHERE mu.id = $1;
        ";

    let price_row = match client.query_one(price_query, &[&machine_id]).await {
        Ok(row) => row,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": "No se ha encontrado la máquina solicitada",
                })),
            );
        }
    };

    let machine_price: f32 = price_row.get("price");
    let number_of_days = (end_date - start_date).num_days() as f32;

    if total_price != number_of_days * machine_price {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "El precio total no es correcto",
            })),
        );
    }

    let user_query = "SELECT * FROM users WHERE id = $1 AND role = 2;";

    if let Err(_) = client.query_one(user_query, &[&user_id]).await {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "message": "No se ha encontrado al usuario o no es un cliente",
            })),
        );
    };

    let insert_query = "
            INSERT INTO rentals (user_id, rental_employee_id, machine_id, start_date, end_date, total_price, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'active')
            RETURNING id;
        ";

    match client
        .query_one(
            insert_query,
            &[
                &user_id,
                &rental_employee_id,
                &machine_id,
                &start_date,
                &end_date,
                &total_price,
            ],
        )
        .await
    {
        Ok(rent_row) => {
            let rental_id: i32 = rent_row.get(0);
            let payment_id = format!("{}", INTERNAL_PAYMENT_ID_PREFIX + (rental_id as u32));

            let update_payment_id_query = "
                UPDATE rentals 
                SET payment_id = $1 
                WHERE id = $2;
                ";

            if let Err(_) = client
                .execute(update_payment_id_query, &[&payment_id, &rental_id])
                .await
            {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Se produjo un error al intentar actualizar el ID de pago del alquiler",
                    })),
                );
            }

            match client
                .query_one("SELECT * FROM users WHERE id = $1;", &[&user_id])
                .await
            {
                Ok(user_row) => {
                    let get_rental_query = "
                            SELECT r.*, l.street, l.number, l.city, mm.brand, mm.name, mm.model 
                            FROM rentals r
                            INNER JOIN machinery_units mu ON r.machine_id = mu.id
                            INNER JOIN machinery_models mm ON mu.model_id = mm.id
                            INNER JOIN locations l ON mu.location_id = l.id
                            WHERE r.id = $1;
                        ";

                    match client.query_one(get_rental_query, &[&rental_id]).await {
                        Ok(rental_row) => {
                            let rent = RentalInfo {
                                id: rental_id,
                                machine_brand: rental_row.get("brand"),
                                machine_name: rental_row.get("name"),
                                machine_model: rental_row.get("model"),
                                start_date: rental_row.get("start_date"),
                                end_date: rental_row.get("end_date"),
                                city: rental_row.get("city"),
                                street: rental_row.get("street"),
                                number: rental_row.get("number"),
                                payment_id,
                            };

                            let formatted_start = rent.start_date.format("%d/%m/%Y").to_string();
                            let formatted_end = rent.end_date.format("%d/%m/%Y").to_string();

                            let user_email: String = user_row.get("email");
                            let user_name: String = user_row.get("name");

                            let subject =
                                format!("Alquiler n° {} aprobado - Bob el Alquilador", rental_id);
                            let body = format!(
                                "Hola {},\n\n\
                                Tu alquiler ha sido aprobado.\n\n\
                                \n\
                                Detalles del Alquiler:\n\
                                \n\n\
                                Número de alquiler: {}\n\
                                Período: {} - {}\n\
                                Máquina: {} {} {}\n\
                                Ubicación: {}, {}, {}\n\
                                Identificador del pago: {}\n\n\
                                \n\n\
                                Gracias por confiar en nosotros.\n\n\
                                Saludos cordiales,\n\
                                El equipo de Bob el Alquilador\n",
                                user_name,
                                rental_id,
                                formatted_start,
                                formatted_end,
                                rent.machine_name,
                                rent.machine_brand,
                                rent.machine_model,
                                rent.city,
                                rent.street,
                                rent.number,
                                rent.payment_id,
                            );

                            match send_mail(&user_email, &subject, &body) {
                                Ok(_) => {
                                    return (
                                        StatusCode::CREATED,
                                        Json(json!({
                                            "message": "El alquiler ha sido registrado exitosamente y se le ha notificado al cliente",
                                        })),
                                    );
                                }
                                Err(_) => {
                                    return (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(json!({
                                            "message": "Se ha producido un error al enviar la notificación al usuario",
                                        })),
                                    );
                                }
                            }
                        }
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({
                                    "message": "Se produjo un error al consultar los datos del alquiler",
                                })),
                            );
                        }
                    }
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "message": "Se produjo un error al consultar datos del cliente",
                        })),
                    );
                }
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Se produjo un error interno al intentar registrar el alquiler",
                })),
            );
        }
    }
}
