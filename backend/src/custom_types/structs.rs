use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateRegularUser {
    pub email: String,
    pub name: String,
    pub surname: String,
    pub birth_date: String,
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
