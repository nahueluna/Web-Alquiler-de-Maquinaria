use chrono::{Duration, Utc};
use deadpool_postgres::Pool;
use serde::{de, Deserialize, Serialize};
use sha2::digest::Mac;
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateRegularUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub surname: String,
    pub birth_date: String,
    #[validate(length(min = 1))]
    pub id_card: String,
    pub phone: Option<String>,
}

// the input to our `client_login` handler
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // subject (user id)
    pub exp: usize,         // expiration time (as UTC timestamp)
    pub role: String,       // user role
    pub token_type: String, // type of token (access or refresh)
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool>,
}

#[derive(Deserialize, Default, Debug, Validate)]
pub struct CatalogParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub order_by: Option<String>,
    pub order_dir: Option<String>,
    pub categories: Option<String>,
    #[validate(range(min = 0.0))]
    pub min_price: Option<f32>,
    #[validate(range(min = 0.0))]
    pub max_price: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Machine {
    pub id: i32,
    pub serial_number: String,
    pub name: String,
    pub brand: String,
    pub model: String,
    pub year: i16,
    pub policy: String,
    pub description: String,
    pub price: f32,
}

impl Machine {
    pub fn build_from_row(row: &tokio_postgres::Row) -> Self {
        Machine {
            id: row.get("id"),
            serial_number: row.get("serial_number"),
            name: row.get("name"),
            brand: row.get("brand"),
            model: row.get("model"),
            year: row.get("year"),
            policy: row.get("policy"),
            description: row.get("description"),
            price: row.get("price"),
        }
    }
}
