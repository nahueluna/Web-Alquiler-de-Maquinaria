use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the input to our `client_login` handler
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // subject (user id)
    pub exp: usize,         // expiration time (as UTC timestamp)
    pub role: String,       // user role
    pub token_type: String, // type of token (access or refresh)
}
