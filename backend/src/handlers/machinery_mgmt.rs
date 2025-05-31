use crate::custom_types::enums::{OrderByField, OrderDirection, PaymentStatus};
use crate::custom_types::structs::*;
use crate::helpers::{
    auth::*,
    machinery_mgmt::{date_is_overlap, get_claims_from_token, validate_client},
};
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
use image::ImageFormat;
use serde_json::json;
use std::{fs::File, io::BufWriter, path::PathBuf, env};
use tokio_postgres::{types::ToSql, error::SqlState};
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
                let mut machine = MachineModel::build_from_row(&machine_row);

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

pub async fn new_model(State(state): State<AppState>, Json(mut payload): Json<NewModel>) -> Response {
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
        Err(_) => {
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

        let is_overlap = unavailable_dates.iter().any(|period| {
            date_is_overlap(start_date, end_date, period.start_date, period.end_date)
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
                    "message": "Las fechas de inicio y fin se superponen con un alquiler existente",
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
                    SET status = 'active', payment_id = $1
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

    match client.execute("INSERT INTO machinery_units
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
        Ok(_) =>
            return (
                StatusCode::CREATED,
                Json(json!({"message": "Unit created successfully"})),
            )
                .into_response(),
        Err(e) => {
            let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let mut message = "Failed to save unit";
            if let Some(db_err) = e.as_db_error() {
                match db_err.code() {
                    &SqlState::UNIQUE_VIOLATION  => {
                        status_code = StatusCode::BAD_REQUEST;
                        message = "The serial_number is already registered";
                    },
                    &SqlState::FOREIGN_KEY_VIOLATION => {
                        let detail = db_err.message().to_lowercase();
                        if detail.contains("model_id") {
                            status_code = StatusCode::BAD_REQUEST;
                            message = "model_id is invalid";
                        } else if detail.contains("location") {
                            status_code = StatusCode::BAD_REQUEST;
                            message = "location_id is invalid";
                        }
                    },
                    _ => {},
                }
            }
            return (
                status_code,
                Json(json!({"message": message})),
            )
                .into_response();
        },
    };
}

pub async fn get_my_rentals(State(state): State<AppState>, Json(payload): Json<Access>) -> Response {
    let frontend_url = match env::var("FRONTEND_URL") {
        Ok(e) => e,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "FRONTEND_URL must be set in the .env file"})),
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
        &[&claims.user_id],)
        .await
    {
        Ok(rows) => {
            let employees: Vec<MyRentalInfo> = rows
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
                    model_image: format!("{}/media/machines/{}.webp",frontend_url,row.get::<_, String>("model_image")),
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

pub async fn load_retirement(State(state): State<AppState>, Json(payload): Json<LoadRetirement>) -> Response {
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

    let row = match transaction.query_one(
            "UPDATE rentals
             SET retirement_employee_id = $1,
                 retirement_date = CURRENT_DATE
             WHERE id = $2
             RETURNING machine_id;",
            &[&claims.user_id, &payload.rental_id],
        )
        .await {
        Ok(r) => r,
        Err(e) => {
            if e.to_string().contains("unexpected number of rows") {
                return (StatusCode::BAD_REQUEST,
                    Json(json!({"message": "rental_id is invalid"}))).into_response();
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to execute transaction"})),
            )
                .into_response();
        },
    };

    let machine_id: i32 = row.get("machine_id");

    // Now update the machine status if it's currently 'available'
    match transaction
        .execute(
            "UPDATE machinery_units
             SET status = 'rented'
             WHERE id = $1 AND status = 'available';",
            &[&machine_id],
        )
        .await {
        Ok(rows) if rows > 0 => (),
        _ => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to execute transaction"})),
            ).into_response(),
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
