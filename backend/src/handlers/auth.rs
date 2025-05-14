use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString},
    Argon2,
};
use axum::{http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::json;

use crate::custom_types::structs::*;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub fn generate_token(
    user_id: &str,
    role: &str,
    token_type: &str,
    expires_in_minutes: usize,
    secret_key: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(expires_in_minutes as i64))
        .expect("date out of range")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        role: role.to_string(),
        token_type: token_type.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
}

// Checks if a password with salt is correct
pub fn check_password(
    password_from_input: &str,
    stored_password_hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&stored_password_hash)?;

    match Argon2::default().verify_password(password_from_input.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

// Gets an user's password hash from the database
pub async fn get_password_hash_from_db(email: &str) -> String {
    let hardcoed_password_hash = "$argon2i$v=19$m=65536,t=4,p=1$randomsalt$randomhash";

    hardcoed_password_hash.to_string()
}

// This handler checks if the email and password are correct
// and returns a JSON response with a message
pub async fn client_login(
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Hardcoded email, replace with database data
    let email = "user@example.com";

    if payload.email == email {
        let salt = "randomsalt";

        let password_with_salt = format!("{}{}", payload.password, salt);

        if let Ok(is_valid) = check_password(
            &password_with_salt,
            get_password_hash_from_db(&email).await.as_str(),
        ) {
            if is_valid {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "message": "Successfully logged in",
                    })),
                );
            }

            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "message": "Invalid password",
                })),
            );
        }

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Error checking password",
            })),
        );
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "message": "Invalid email",
        })),
    )
}
