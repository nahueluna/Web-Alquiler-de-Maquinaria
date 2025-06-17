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

pub async fn get_unanswered_questions(State(state): State<AppState>, Json(payload): Json<Access>) -> Response {
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
        .query("SELECT
            questions.id,
            questions.content,
            questions.created_at,
            users.name AS user_name,
            users.surname,
            machinery_models.id AS model_id,
            machinery_models.brand,
            machinery_models.name AS model_name,
            machinery_models.model
            FROM questions
            JOIN users ON questions.user_id = users.id
            JOIN machinery_models ON questions.model_id = machinery_models.id
            WHERE NOT EXISTS (
                SELECT 1
                FROM answers
                WHERE answers.question_id = questions.id
            )
            ORDER BY questions.created_at ASC;",
            &[],
        )
        .await
    {
        Ok(rows) => {
            let questions: Vec<UnansweredQuestion> = rows
                .iter()
                .map(|row| UnansweredQuestion {
                    question_id: row.get("id"),
                    content: row.get("content"),
                    created_at: row.get("created_at"),
                    user_name: row.get("user_name"),
                    user_surname: row.get("surname"),
                    model_id: row.get("model_id"),
                    model_brand: row.get("brand"),
                    model_name: row.get("model_name"),
                    model_model: row.get("model"),
                })
                .collect();
            return (StatusCode::OK, Json(json!({"questions": questions}))).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to get the questions"})),
            )
                .into_response()
        }
    };
}

pub async fn get_questions(
    State(state): State<AppState>,
    Json(payload): Json<GetQuestions>,
) -> Response {
    let user_id = if let Some(access) = payload.access {
        match validate_jwt(&access) {
            Some(data) => data.claims.user_id,
            None => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"message": "Invalid access token"})),
                )
                    .into_response()
            }
        }
    } else {
        0 //User 0 is guaranteed to not exist, so it wont have any votes
    };

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

    let order_clause = match payload.order_by_recent {
        true => "questions.created_at DESC, upvote_count DESC, question_id ASC",
        false => "upvote_count DESC, questions.created_at DESC, question_id ASC",
    };

    let query = format!("
        SELECT
            questions.id AS question_id,
            questions.content,
            questions.created_at,
            askers.name AS user_name,
            askers.surname AS user_surname,
            COALESCE(uv.count, 0) AS upvote_count,
            EXISTS (
                SELECT 1 FROM question_votes
                WHERE question_votes.question_id = questions.id
                AND question_votes.user_id = $1
            ) AS upvoted,
            answers.question_id IS NOT NULL AS has_answer,
            answers.content AS answer_content,
            answers.created_at AS answer_created_at,
            answerers.name AS answer_user_name,
            answerers.surname AS answer_user_surname
        FROM questions
        JOIN users askers ON questions.user_id = askers.id
        LEFT JOIN (
            SELECT question_id, COUNT(*) AS count
            FROM question_votes
            GROUP BY question_id
        ) uv ON questions.id = uv.question_id
        LEFT JOIN answers ON questions.id = answers.question_id
        LEFT JOIN users answerers ON answers.user_id = answerers.id
        WHERE questions.model_id = $2
        ORDER BY {order_clause};
        "
    );

    let result = client.query(&query, &[&user_id, &payload.model_id]).await;

    match result {
        Ok(rows) => {
            let questions: Vec<Question> = rows.iter().map(|row| {

                let answer = if row.get::<_, bool>("has_answer") {
                    Some(Answer {
                        user_name: row.get("answer_user_name"),
                        user_surname: row.get("answer_user_surname"),
                        created_at: row.get("answer_created_at"),
                        content: row.get("answer_content"),
                    })
                } else {
                    None
                };

                Question {
                    question_id: row.get("question_id"),
                    content: row.get("content"),
                    created_at: row.get("created_at"),
                    user_name: row.get("user_name"),
                    user_surname: row.get("user_surname"),
                    upvotes: row.get("upvote_count"),
                    upvoted: row.get("upvoted"),
                    answer,
                }




            }).collect();
            (StatusCode::OK, Json(json!({ "questions": questions }))).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to retrieve questions" })),
        )
            .into_response(),
    }
}
