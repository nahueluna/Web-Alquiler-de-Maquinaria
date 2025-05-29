use crate::custom_types::enums::{OrderByField, OrderDirection};
use crate::custom_types::structs::{
    Access, AppState, CatalogParams, Location, MachineModel, ModelAndLocation,
};
use crate::helpers::auth::validate_jwt;
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use axum_extra::extract::Query;
use serde_json::json;
use tokio_postgres::types::ToSql;
use validator::Validate;

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
