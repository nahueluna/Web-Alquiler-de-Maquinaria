use crate::helpers::auth::validate_jwt;
use axum::{http::StatusCode, Json};
use serde_json::json;

pub fn validate_client(access_token: &str) -> Option<(StatusCode, Json<serde_json::Value>)> {
    let claims = match validate_jwt(&access_token) {
        Some(data) => data,
        None => {
            return Some((
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            ));
        }
    }
    .claims;

    if claims.role != 2 {
        return Some((
            StatusCode::FORBIDDEN,
            Json(json!({"message": "The user is not a client"})),
        ));
    }

    return None;
}
