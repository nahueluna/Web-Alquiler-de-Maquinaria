use super::enums::{OrderByField, OrderDirection, PaymentStatus};
use axum::Json;
use chrono::{NaiveDate, NaiveDateTime};
use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub psw_hash: String,
    pub salt: String,
    pub role: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubUser {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub role: i16,
}

impl From<User> for PubUser {
    fn from(user: User) -> Self {
        PubUser {
            id: user.id,
            email: user.email,
            name: user.name,
            surname: user.surname,
            role: user.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub birthdate: NaiveDate,
    pub id_card: String,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubUserWithInfo {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub role: i16,
    pub birthdate: NaiveDate,
    pub id_card: String,
    pub phone: Option<String>,
}

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

#[derive(Deserialize, Validate)]
pub struct CreateEmployee {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub surname: String,
    pub birthdate: String,
    #[validate(length(min = 1))]
    pub id_card: String,
    pub phone: Option<String>,
    pub access: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub code: Option<i32>, //2FA Code
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,       // expiration time (as UTC timestamp)
    pub role: i16,        // user role
    pub is_refresh: bool, //Whether it is an access or a refresh token
    pub nonce: u32,       //To ensure randomness
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Access {
    pub access: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePhone {
    pub access: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEmployee {
    pub access: String,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangePsw {
    #[validate(length(min = 8))]
    pub new_password: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckChangePswCode {
    pub code: String,
}

#[derive(Deserialize, Default, Debug, Validate)]
pub struct CatalogParams {
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub order_by: Option<OrderByField>,
    pub order_dir: Option<OrderDirection>,
    #[serde(default, rename = "category")]
    pub categories: Vec<String>,
    #[validate(range(min = 0.0))]
    pub min_price: Option<f32>,
    #[validate(range(min = 0.0))]
    pub max_price: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineModel {
    pub id: i32,
    pub name: String,
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub policy: String,
    pub description: String,
    pub price: f32,
    pub categories: Vec<Category>,
    pub main_image: String, //base64 encoded string
    pub extra_images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyRentalInfo {
    pub rental_id: i32,
    pub return_date: Option<NaiveDate>,
    pub retirement_date: Option<NaiveDate>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_price: f32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub unit_id: i32,
    pub unit_serial_number: String,
    pub model_id: i32,
    pub model_name: String,
    pub model_brand: String,
    pub model_model: String,
    pub model_year: i32,
    pub model_policy: String,
    pub model_description: String,
    pub model_image: String,
    pub days_late: Option<i64>,
    pub percentage_per_late_day: Option<String>,
}

impl MachineModel {
    pub fn build_from_row(
        row: &tokio_postgres::Row,
    ) -> Result<Self, (StatusCode, Json<serde_json::Value>)> {
        let nginx_url = match env::var("NGINX_URL") {
            Ok(url) => url,
            Err(_) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "NGINX_URL environment variable is not set. Cannot get machine image."
                    })),
                ))
            }
        };

        let machine = MachineModel {
            id: row.get("id"),
            name: row.get("name"),
            brand: row.get("brand"),
            model: row.get("model"),
            year: row.get("year"),
            policy: row.get("policy"),
            description: row.get("description"),
            price: row.get("price"),
            categories: Vec::new(),
            main_image: format!(
                "{}/media/machines/{}.webp",
                nginx_url,
                row.get::<_, String>("image")
            ),
            extra_images: Vec::new(),
        };

        Ok(machine)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub street: String,
    pub number: String,
    pub city: String,
}

impl Location {
    pub fn build_from_row(row: &tokio_postgres::Row) -> Self {
        Location {
            id: row.get("id"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            street: row.get("street"),
            number: row.get("number"),
            city: row.get("city"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelAndLocation {
    pub model_id: i32,
    pub location_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitDatesInfo {
    machines_info: Vec<UnitAndDates>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitAndDates {
    pub unit_id: i32,
    pub periods: Vec<DateRange>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct NewRental {
    pub machine_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    #[validate(range(min = 0.0))]
    pub total_price: f32,
    pub access: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewModel {
    pub name: String,
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub policy: String,
    pub description: String,
    pub price: f32,
    pub categories: Vec<String>,
    pub extra_images: Vec<String>, //base64 encoded strings
    pub access: String,
    pub image: String, //base64 encoded strings
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUnit {
    pub serial_number: String,
    pub model_id: i32,
    pub location_id: i32,
    pub access: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPayment {
    pub payment_id: String,
    pub status: PaymentStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RentalIdAndToken {
    pub rental_id: i32,
    pub access: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RentalInfo {
    pub id: i32,
    pub machine_brand: String,
    pub machine_model: String,
    pub machine_name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub city: String,
    pub street: String,
    pub number: String,
    pub payment_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRetirement {
    pub access: String,
    pub rental_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadReturn {
    pub access: String,
    pub rental_id: i32,
    pub location_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelRentalInfo {
    pub access: String,
    pub rental_id: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRentalQueryParams {
    pub id: Option<i32>, // Rental ID
}
