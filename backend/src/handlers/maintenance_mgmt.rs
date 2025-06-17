use crate::custom_types::enums::*;
use crate::custom_types::structs::*;
use crate::helpers::auth::*;
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use serde_json::json;

pub async fn get_machine_unit(
    State(state): State<AppState>,
    Path(serial_number_info): Path<SerialNumber>,
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

    let serial_number = serial_number_info
        .serial_number
        .trim()
        .to_uppercase()
        .to_string();

    if serial_number.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "El número de serie no puede estar vacío"})),
        );
    }

    let unit_info_query = "
    SELECT mm.id AS model_id, mm.name, mm.brand, mm.model, mm.year, mm.image,
    mu.id AS unit_id, mu.serial_number, mu.status::TEXT, l.id AS location_id, l.city, l.street, l.number
    FROM machinery_units mu 
    INNER JOIN machinery_models mm ON mu.model_id = mm.id
    INNER JOIN locations l ON mu.location_id = l.id
    WHERE serial_number = $1;";

    if let Ok(row) = client.query_one(unit_info_query, &[&serial_number]).await {
        let unit_info_result = GetMachineUnit::build_from_row(&row);

        let unit_info = match unit_info_result {
            Ok(info) => info,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Ocurrió un error al procesar la imagen de la unidad"})),
                );
            }
        };

        return (
            StatusCode::OK,
            Json(json!({
                "unit_info": unit_info,
            })),
        );
    }

    return (
        StatusCode::NOT_FOUND,
        Json(json!({"message": "El número de serie ingresado no existe"})),
    );
}

pub async fn get_unit_history(
    State(state): State<AppState>,
    Path(unit_id): Path<i32>,
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

    let history_query = "
    SELECT uhe.id AS event_id, uhe.description, uhe.previous_status::TEXT, uhe.new_status::TEXT, uhe.created_at 
    FROM machinery_units mu INNER JOIN unit_history_events uhe ON mu.id = uhe.unit_id
    WHERE mu.id = $1
    ORDER BY uhe.created_at DESC;
    ";

    if let Ok(rows) = client.query(history_query, &[&unit_id]).await {
        if rows.is_empty() {
            if let Err(_) = client
                .query_one("SELECT id FROM machinery_units WHERE id = $1", &[&unit_id])
                .await
            {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message": "La unidad solicitada no existe"})),
                );
            }
        }

        let mut history_events = Vec::new();

        rows.iter().for_each(|r| {
            let event = UnitHistoryEvent::build_from_row(r);
            history_events.push(event);
        });

        return (
            StatusCode::OK,
            Json(json!({
                "unit_id": unit_id,
                "history": history_events,
            })),
        );
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"message": "Ocurrió un error al obtener el historial de la unidad"})),
    );
}

pub async fn update_unit_history(
    State(state): State<AppState>,
    Json(payload): Json<UpdateUnitHistoryInfo>,
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

    let unit_row = match client
        .query_one(
            "SELECT status::TEXT FROM machinery_units WHERE id = $1",
            &[&payload.unit_id],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "La unidad indicada no existe"})),
            );
        }
    };

    let previous_status = format!("'{}'", unit_row.get::<_, String>(0));

    let new_status = match payload.new_status {
        UnitStatusEvents::Available => "'available'",
        UnitStatusEvents::Maintenance => "'maintenance'",
    };

    let update_unit_status_query = format!(
        "
    UPDATE machinery_units
    SET status = {}
    WHERE id = $1;
    ",
        new_status
    );

    if let Err(_) = client
        .execute(&update_unit_status_query, &[&payload.unit_id])
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Ocurrió un error al actualizar el estado de la unidad"})),
        );
    }

    let insert_history_event_query = format!(
        "
    INSERT INTO unit_history_events (unit_id, description, previous_status, new_status)
    VALUES ($1, $2, {}, {});
    ",
        previous_status, new_status
    );

    match client
        .execute(
            &insert_history_event_query,
            &[&payload.unit_id, &payload.description],
        )
        .await
    {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(json!({"message": "El evento se ha registrado correctamente"})),
            );
        }

        Err(e) => {
            eprintln!("Error al registrar el evento en el historial: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Ocurrió un error al registrar el evento en el historial"})),
            );
        }
    }
}
