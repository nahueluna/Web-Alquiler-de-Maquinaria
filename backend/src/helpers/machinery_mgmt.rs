use crate::{custom_types::structs::Claims, helpers::auth::validate_jwt};
use axum::{http::StatusCode, Json};
use chrono::NaiveDate;
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

pub fn get_claims_from_token(access_token: &str) -> Option<Claims> {
    match validate_jwt(&access_token) {
        Some(data) => Some(data.claims),
        None => None,
    }
}

pub fn date_is_overlap(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> bool {
    a_start <= b_end && b_start <= a_end
}

pub fn clean_strings(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace()) // ¡Aquí está el cambio!
        .collect()
}
