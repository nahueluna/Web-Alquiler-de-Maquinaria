use crate::custom_types::structs::*;
use crate::custom_types::enums::ReviewOrder;
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

pub async fn get_service_reviews(
    State(state): State<AppState>,
    Json(payload): Json<GetServiceReviews>,
) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data.claims,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response();
        }
    };

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
                .into_response();
        }
    };

    // Construct ORDER BY clause
    let order_clause = match payload.order {
        Some(ReviewOrder::MoreRating) => "ORDER BY sr.rating DESC, sr.created_at DESC",
        Some(ReviewOrder::LessRating) => "ORDER BY sr.rating ASC, sr.created_at DESC",
        Some(ReviewOrder::Recent) | None => "ORDER BY sr.created_at DESC",
    };

    // Construct WHERE clause
    let rating;
    let (where_clause, params): (&str, Vec<&(dyn tokio_postgres::types::ToSql + Sync)>) =
        if let Some(r) = payload.rating {
            rating = r;
            ("WHERE sr.rating = $1", vec![&rating])
        } else {
            ("", vec![])
        };

    // Execute query
    let query = format!(
        "
        SELECT
            CONCAT(u.name, ' ', u.surname) AS user_name,
            sr.rating,
            sr.content,
            sr.created_at,
            CONCAT(rental_emp.name, ' ', rental_emp.surname) AS rental_employee_name,
            CONCAT(retirement_emp.name, ' ', retirement_emp.surname) AS retirement_employee_name,
            CONCAT(return_emp.name, ' ', return_emp.surname) AS return_employee_name,
            r.id AS rental_id,
            mm.brand AS model_brand,
            mm.name AS model_name,
            mm.model AS model_model,
            mu.serial_number
        FROM service_reviews sr
        JOIN users u ON sr.user_id = u.id
        JOIN rentals r ON sr.rental_id = r.id
        JOIN machinery_units mu ON r.machine_id = mu.id
        JOIN machinery_models mm ON mu.model_id = mm.id
        LEFT JOIN users rental_emp ON r.rental_employee_id = rental_emp.id
        LEFT JOIN users retirement_emp ON r.retirement_employee_id = retirement_emp.id
        LEFT JOIN users return_emp ON r.return_employee_id = return_emp.id
        {where_clause}
        {order_clause};
        "
    );

    let rows = match client.query(&query, &params).await {
        Ok(rows) => rows,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to retrieve service reviews"})),
            )
                .into_response();
        }
    };

    // Build response
    let reviews: Vec<ServiceReview> = rows
        .into_iter()
        .map(|row| ServiceReview {
            user_name: row.get("user_name"),
            rating: row.get("rating"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            rental_employee_name: row.get("rental_employee_name"),
            retirement_employee_name: row.get("retirement_employee_name"),
            return_employee_name: row.get("return_employee_name"),
            rental_id: row.get("rental_id"),
            model_brand: row.get("model_brand"),
            model_name: row.get("model_name"),
            model_model: row.get("model_model"),
            serial_number: row.get("serial_number"),
        })
        .collect();

    (StatusCode::OK, Json(json!({ "reviews": reviews }))).into_response()
}
pub async fn get_machine_reviews(
    State(state): State<AppState>,
    Json(payload): Json<GetMachineReviews>,
) -> Response {
    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response();
        }
    };

    // ORDER BY clause
    let order_clause = match payload.order {
        Some(ReviewOrder::MoreRating) => "ORDER BY mr.rating DESC, mr.created_at DESC",
        Some(ReviewOrder::LessRating) => "ORDER BY mr.rating ASC, mr.created_at DESC",
        Some(ReviewOrder::Recent) | None => "ORDER BY mr.created_at DESC",
    };

    // WHERE clause for reviews
    let rating;
    let (where_clause, params): (&str, Vec<&(dyn tokio_postgres::types::ToSql + Sync)>) =
        if let Some(r) = payload.rating {
            rating = r;
            ("WHERE mr.model_id = $1 AND mr.rating = $2", vec![&payload.model_id, &rating])
        } else {
            ("WHERE mr.model_id = $1", vec![&payload.model_id])
        };

    let query = format!(
        "
        SELECT
            CONCAT(u.name, ' ', u.surname) AS user_name,
            mr.rating,
            mr.content,
            mr.created_at,
            mr.rental_id
        FROM machine_reviews mr
        JOIN users u ON mr.user_id = u.id
        {where_clause}
        {order_clause};
        "
    );

    let rows = match client.query(&query, &params).await {
        Ok(rows) => rows,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to retrieve machine reviews"})),
            )
                .into_response();
        }
    };

    let reviews: Vec<MachineReview> = rows
        .into_iter()
        .map(|row| MachineReview {
            user_name: row.get("user_name"),
            rating: row.get("rating"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            rental_id: row.get("rental_id"),
        })
        .collect();

    // Always calculate average rating over all reviews of the model_id
    let avg_rating: Option<f64> = match client
        .query_one("SELECT AVG(rating)::FLOAT FROM machine_reviews WHERE model_id = $1", &[&payload.model_id])
        .await
    {
        Ok(row) => row.get(0),
        Err(_) => None,
    };

    (StatusCode::OK, Json(json!({
        "average_rating": avg_rating,
        "reviews": reviews
    })))
    .into_response()
}
