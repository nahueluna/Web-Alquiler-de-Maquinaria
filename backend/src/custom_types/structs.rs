use serde::{Deserialize, Serialize};
use validator::Validate;
use deadpool_postgres::Pool;
use std::sync::Arc;

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
