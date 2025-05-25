use serde::{Deserialize, Serialize};
use validator::Validate;
use deadpool_postgres::Pool;
use std::sync::Arc;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub birthdate: NaiveDate,
    pub id_card: String,
    pub phone: Option<String>,
    pub psw_hash: String,
    pub salt: String,
    pub role: i16,
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

// the input to our `client_login` handler
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,     // expiration time (as UTC timestamp)
    pub role: i16,       // user role
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool>,
}
