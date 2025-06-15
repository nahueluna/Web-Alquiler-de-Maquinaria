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

pub async fn new_question(State(state): State<AppState>, Json(payload): Json<NewQuestion>) -> Response {

    if payload.content.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The question's content is empty"})),
        )
            .into_response();
    }

    if payload.content.len() > 256 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The question's content is larger than 256 characters"})),
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

    if claims.role != 2 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Invalid role"})),
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
            "INSERT INTO questions
        (user_id, model_id, content)
        VALUES ($1, $2, $3);",
            &[
                &claims.user_id,
                &payload.model_id,
                &payload.content,
            ],
        )
        .await
    {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(json!({"message": "Question created successfully"})),
            )
                .into_response()
        }
        Err(e) => {
            if let Some(db_err) = e.as_db_error() {
                if db_err.code() == &SqlState::FOREIGN_KEY_VIOLATION && db_err.message().contains("model_id")
                {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"message": "model_id is invalid"})),
                    )
                        .into_response();
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to save unit"}))).into_response();
        }
    };
}

pub async fn new_answer(State(state): State<AppState>, Json(payload): Json<NewAnswer>) -> Response {

    if payload.content.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The answer's content is empty"})),
        )
            .into_response();
    }

    if payload.content.len() > 256 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The answer's content is larger than 256 characters"})),
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

    if claims.role == 2 {
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
            "INSERT INTO answers
        (question_id, user_id, content)
        VALUES ($1, $2, $3);",
            &[
                &payload.question_id,
                &claims.user_id,
                &payload.content,
            ],
        )
        .await
    {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(json!({"message": "Answer created successfully"})),
            )
                .into_response()
        }
        Err(e) => {
            let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let mut message = "Failed to save answer";
            if let Some(db_err) = e.as_db_error() {
                match db_err.code() {
                    &SqlState::UNIQUE_VIOLATION => {
                        status_code = StatusCode::BAD_REQUEST;
                        message = "The question has already been answered";
                    }
                    &SqlState::FOREIGN_KEY_VIOLATION => {
                        let detail = db_err.message().to_lowercase();
                        if detail.contains("question_id") {
                            status_code = StatusCode::BAD_REQUEST;
                            message = "question_id is invalid";
                        }
                    }
                    _ => {}
                }
            }
            return (status_code, Json(json!({"message": message}))).into_response();
        }
    }
}

pub async fn vote_question(State(state): State<AppState>, Json(payload): Json<VoteQuestion>) -> Response {
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


    if payload.upvote {
        match client .execute("INSERT INTO question_votes
        (question_id, user_id)
        VALUES ($1, $2);",&[&payload.question_id,&claims.user_id]).await {
            Ok(_) => {
                return (
                    StatusCode::CREATED,
                    Json(json!({"message": "Vote saved successfully"})),
                )
                    .into_response()
            }
            Err(e) => {
                let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
                let mut message = "Failed to save vote";
                if let Some(db_err) = e.as_db_error() {
                    match db_err.code() {
                        &SqlState::UNIQUE_VIOLATION => {
                            status_code = StatusCode::OK;
                            message = "The vote was already saved";
                        }
                        &SqlState::FOREIGN_KEY_VIOLATION => {
                            let detail = db_err.message().to_lowercase();
                            if detail.contains("question_id") {
                                status_code = StatusCode::BAD_REQUEST;
                                message = "question_id is invalid";
                            }
                        }
                        _ => {}
                    }
                }
                return (status_code, Json(json!({"message": message}))).into_response();
            }
        }
    } else {
        match client.execute("DELETE FROM question_votes WHERE question_id = $1 AND user_id = $2;",&[&payload.question_id,&claims.user_id]).await {
            Ok(n) => {
                if n == 1 {
                    return (
                        StatusCode::CREATED,
                        Json(json!({"message": "Vote saved successfully"})),
                    )
                        .into_response()
                } else {
                    if let Ok(rows) = client.query("SELECT * FROM questions WHERE id = $1;",
                        &[&payload.question_id]).await {
                        if rows.len() == 1 {
                            return (
                                StatusCode::OK,
                                Json(json!({"message": "The vote was already saved"}))).into_response();

                        } else {
                                return (
                                    StatusCode::BAD_REQUEST,
                                    Json(json!({"message": "question_id is invalid"})),
                                )
                                    .into_response()
                        }
                    }
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Failed to save vote"})),
                    )
                        .into_response();
                }
            }
            Err(_) => return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to save vote"})),
                )
                    .into_response(),
        }
    }
}
