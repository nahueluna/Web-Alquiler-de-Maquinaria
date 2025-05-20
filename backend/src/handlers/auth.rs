use axum::{http::StatusCode, Json};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use dotenvy::dotenv;
use hex;
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use sha2::{Digest, Sha256};

use serde_json::json;
use std::env;
use tokio_postgres::{Error, NoTls};

use crate::custom_types::structs::*;

fn generate_random_string(lenght: usize) -> String {
    let random_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect();

    random_string
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn connect_db() -> Result<tokio_postgres::Client, Error> {
    dotenv().ok();

    let data_base_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(&data_base_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

fn is_adult(birth_date: NaiveDate) -> bool {
    let today = Utc::now().naive_utc().date();

    let mut age = today.year() - birth_date.year();

    if today.ordinal() < birth_date.ordinal() {
        age -= 1;
    }

    age >= 18
}

fn encode_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub async fn client_sign_up(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateRegularUser` type
    Json(payload): Json<CreateRegularUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Ok(client) = connect_db().await {
        if let Ok(rows) = client
            .query("SELECT * FROM users WHERE email = $1;", &[&payload.email])
            .await
        {
            if !rows.is_empty() {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({
                        "message": "Email is already registered",
                    })),
                );
            }

            let birth_date = chrono::NaiveDate::parse_from_str(&payload.birth_date, "%d-%m-%Y")
                .expect("Invalid date format");

            if !is_adult(birth_date) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "message": "User age is less than 18",
                    })),
                );
            }

            let password = generate_random_string(8);
            let salt = generate_random_string(16);
            let hashed_password = encode_password(&format!("{}{}", salt, password));

            let role: i16 = 2; // 2 = client user role

            if let Ok(_) = client
                .query(
                    "INSERT INTO users (email, name, surname, birthdate, id_card, phone, password, role)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
                    &[
                        &payload.email,
                        &payload.name,
                        &payload.surname,
                        &birth_date,
                        &payload.id_card,
                        &payload.phone,
                        &hashed_password,
                        &role,
                    ],
                )
                .await
            {
                return (
                    StatusCode::CREATED,
                    Json(json!({"message": "Client user successfully registered"})),
                );
            }
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": "Error connecting to database",
        })),
    )
}

fn generate_token(
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
fn check_password(password_from_input: &str, stored_password_hash: &str) -> bool {
    return encode_password(password_from_input) == stored_password_hash;
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
        let stored_password_hash = get_password_hash_from_db(&payload.email).await;
        {
            if check_password(&password_with_salt, &stored_password_hash) {
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
