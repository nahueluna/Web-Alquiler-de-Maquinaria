use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString},
    Argon2,
};
use axum::{http::StatusCode, Json};
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::json;
use std::env;
use tokio_postgres::{Error, NoTls};

use crate::custom_types::structs::*;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn connect_db() -> Result<tokio_postgres::Client, Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=saga user=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateRegularUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let email = "newuser@example.com";
    let password = "plaintext123";
    let role: i16 = 1;

    if let Ok(client) = connect_db().await {
        let birth_date = chrono::NaiveDate::parse_from_str(&payload.birth_date, "%Y-%m-%d")
            .expect("Invalid date format");

        client
            .query(
                "INSERT INTO users (email, name, surname, birthdate, id_card, password, role) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&payload.email, &payload.name, &payload.surname, &birth_date, &payload.dni, &payload.phone, &password, &role],
            )
            .await
            .unwrap();

        // insert your application logic here
        let user = User {
            id: 1337,
            username: payload.username,
        };

        // this will be converted into a JSON response
        // with a status code of `201 Created`
        (StatusCode::CREATED, Json(json!({"user": user})))
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Error connecting to database",
            })),
        )
    }
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

pub fn send_mail() {
    // Load variables from .env
    dotenv().ok();

    let email_address = env::var("EMAIL").unwrap();
    let app_password = env::var("APP_PASSWORD").unwrap();

    let email = Message::builder()
        .from(Mailbox::new(
            Some("Rust Bot".into()),
            email_address.parse().unwrap(),
        ))
        .reply_to(email_address.parse().unwrap())
        .to("recipient@example.com".parse().unwrap())
        .subject("Hello from Rust!")
        .body("This email was sent securely using dotenv.".to_string())
        .unwrap();

    let creds = Credentials::new(email_address.clone(), app_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Failed to send email: {e}"),
    }
}
