use crate::custom_types::structs::*;
use crate::helpers::auth::*;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tokio_postgres::error::SqlState;

pub async fn new_machine_review(State(state): State<AppState>, Json(payload): Json<NewReview>) -> Response {
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
            Json(json!({"message": "Invalid role"})),
        )
            .into_response();
    }

    if payload.rating < 1 || payload.rating > 5 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Rating must be between 1 and 5 inclusive"})),
        )
            .into_response();
    }

    if payload.rating < 5 {
        if let Some(content) = &payload.content {
            if content.len() < 1 || content.len() > 256 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"message": "Content must be between 1 and 256 characters inclusive"})),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Content is required on reviews with less than 5 stars"})),
            )
                .into_response();
        }
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

    // Check that the rental exists, belongs to the user, it's completed, and get model_id
    let row = match client
        .query_opt(
            "
            SELECT mu.model_id
            FROM rentals r
            INNER JOIN machinery_units mu ON r.machine_id = mu.id
            WHERE r.id = $1 AND r.user_id = $2 AND r.status = 'completed';
            ",
            &[&payload.rental_id, &claims.user_id],
        )
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Rental not found, does not belong to user, or is not completed"})),
            )
            .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to query rental"})),
            )
            .into_response();
        }
    };

    let model_id: i32 = row.get("model_id");

    // Try to insert the review
    let result = client
        .execute(
            "
            INSERT INTO machine_reviews (rental_id, user_id, model_id, rating, content)
            VALUES ($1, $2, $3, $4, $5);
            ",
            &[
                &payload.rental_id,
                &claims.user_id,
                &model_id,
                &payload.rating,
                &payload.content,
            ],
        )
        .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({"message": "Review saved successfully"})),
        )
        .into_response(),
        Err(e) => {
            let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let mut message = "Failed to save machine review";
            if let Some(db_err) = e.as_db_error() {
                if db_err.code() == &SqlState::UNIQUE_VIOLATION {
                    status_code = StatusCode::BAD_REQUEST;
                    message = "This rental has already been reviewed";
                }
            }
            (status_code, Json(json!({"message": message}))).into_response()
        }
    }
}

pub async fn new_service_review(State(state): State<AppState>, Json(payload): Json<NewReview>) -> Response {
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
            Json(json!({"message": "Invalid role"})),
        )
            .into_response();
    }

    if payload.rating < 1 || payload.rating > 5 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Rating must be between 1 and 5 inclusive"})),
        )
            .into_response();
    }

    if payload.rating < 5 {
        if let Some(content) = &payload.content {
            if content.len() < 1 || content.len() > 256 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"message": "Content must be between 1 and 256 characters inclusive"})),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Content is required on reviews with less than 5 stars"})),
            )
                .into_response();
        }
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

    // Check that the rental exists, belongs to the user, and it's completed
    match client
        .query_opt(
            "
            SELECT *
            FROM rentals r
            INNER JOIN machinery_units mu ON r.machine_id = mu.id
            WHERE r.id = $1 AND r.user_id = $2 AND r.status = 'completed';
            ",
            &[&payload.rental_id, &claims.user_id],
        )
        .await
    {
        Ok(Some(_)) => (),
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Rental not found, does not belong to user, or is not completed"})),
            )
            .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to query rental"})),
            )
            .into_response();
        }
    };


    // Try to insert the review
    let result = client
        .execute(
            "
            INSERT INTO service_reviews (rental_id, user_id, rating, content)
            VALUES ($1, $2, $3, $4);
            ",
            &[
                &payload.rental_id,
                &claims.user_id,
                &payload.rating,
                &payload.content,
            ],
        )
        .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({"message": "Review saved successfully"})),
        )
        .into_response(),
        Err(e) => {
            let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let mut message = "Failed to save service review";
            if let Some(db_err) = e.as_db_error() {
                if db_err.code() == &SqlState::UNIQUE_VIOLATION {
                    status_code = StatusCode::BAD_REQUEST;
                    message = "This rental has already been reviewed";
                }
            }
            (status_code, Json(json!({"message": message}))).into_response()
        }
    }
}
