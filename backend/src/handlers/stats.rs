use crate::constants::CHANGE_PSW_CODE_EXP_MINS;
use crate::custom_types::structs::*;
use crate::custom_types::enums::{StatType, StatGroupBy};
use crate::helpers::auth::*;
use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::TypedHeader;
use deadpool_postgres::GenericClient;
use headers::Cookie;
use hex;
use rand::RngCore;
use serde_json::json;
use std::env;
use tokio_postgres::error::SqlState;
use validator::Validate;
use chrono::{Datelike, Local, NaiveDate};
use std::collections::HashMap;


pub async fn get_stats_by_month(state: AppState, payload: GetStats) -> Response {
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

    let year = payload.year.unwrap_or_else(|| Local::now().year());

    let select_clause = match payload.stat_type {
        StatType::Rentals => "COUNT(*)::float",
        StatType::Income => "SUM(total_price)::float",
    };

    let query = format!(
        "
        SELECT
            EXTRACT(MONTH FROM created_at)::int AS month,
            {select_clause} AS value
        FROM rentals
        WHERE EXTRACT(YEAR FROM created_at)::int = $1
        GROUP BY month;
        "
    );

    let result = client.query(&query, &[&year]).await;

    match result {
        Ok(rows) => {
            let mut data = HashMap::from([
                ("january", 0.0),
                ("february", 0.0),
                ("march", 0.0),
                ("april", 0.0),
                ("may", 0.0),
                ("june", 0.0),
                ("july", 0.0),
                ("august", 0.0),
                ("september", 0.0),
                ("october", 0.0),
                ("november", 0.0),
                ("december", 0.0),
            ]);

            let month_names = [
                "january",
                "february",
                "march",
                "april",
                "may",
                "june",
                "july",
                "august",
                "september",
                "october",
                "november",
                "december",
            ];

            for row in rows {
                let month: i32 = row.get("month");
                let value: f64 = row.get("value");
                if let Some(name) = month_names.get((month - 1) as usize) {
                    data.insert(name, value);
                }
            }

            (StatusCode::OK, Json(json!({"stats": data}))).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to retrieve stats"})),
        )
            .into_response(),
    }
}

pub async fn get_stats_by_employee(state: AppState, payload: GetStats) -> Response {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Get Stats By Employee: WIP"})),
            )
                .into_response()
}

pub async fn get_stats_by_category(state: AppState, payload: GetStats) -> Response {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Get Stats By Category: WIP"})),
            )
                .into_response()
}

pub async fn get_stats(State(state): State<AppState>, Json(payload): Json<GetStats>) -> Response {
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

    match payload.group_by {
        StatGroupBy::Month => get_stats_by_month(state, payload).await,
        StatGroupBy::Employee => get_stats_by_employee(state, payload).await,
        StatGroupBy::Category => get_stats_by_category(state, payload).await,
    }
}
