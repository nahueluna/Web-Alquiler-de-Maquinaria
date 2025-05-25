use axum::{http::StatusCode, Json, extract::State};
use validator::Validate;
use serde_json::json;
use std::env;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::custom_types::structs::*;
use crate::helpers::auth::*;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "OK"
}

pub async fn client_sign_up(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateRegularUser` type
    State(state): State<AppState>,
    payload_result: Result<Json<CreateRegularUser>, axum::extract::rejection::JsonRejection>,
) -> (StatusCode, Json<serde_json::Value>) {
    let payload = match payload_result {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "message": "Invalid JSON format",
                })),
            );
        }
    };

    if let Err(_) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Invalid input data",
            })),
        );
    }

    if let Ok(mut client) = state.pool.get().await {
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

            let birth_date =
                match chrono::NaiveDate::parse_from_str(&payload.birth_date, "%d-%m-%Y") {
                    Ok(date) => date,
                    Err(_) => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                "message": "Invalid birth date format",
                            })),
                        );
                    }
                };

            if !is_adult(birth_date) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "message": "User age is less than 18",
                    })),
                );
            }

            let transaction = match client.transaction().await {
                Ok(t) => t,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to create a DB transaction",}))),
            };

            let password = generate_random_string(8);
            let salt = generate_random_string(16);
            let hashed_password = encode_password(&password, &salt);

            let role: i16 = 2; // 2 = client user role

            let users_t = transaction.execute(
                    "INSERT INTO users (email, name, surname, psw_hash, salt, role) VALUES ($1, $2, $3, $4, $5, $6);",
                    &[&payload.email,&payload.name,&payload.surname,
                      &hashed_password,&salt,&role,],).await;

            let clients_t = transaction.execute(
                    "INSERT INTO clients (id, birthdate, id_card, phone) VALUES (currval('users_id_seq'), $1, $2, $3)",
                    &[&birth_date, &payload.id_card, &payload.phone],
                    ).await;

            if users_t.is_ok() && clients_t.is_ok() {
                let subject = "Contraseña generada para sistema de Bob el Alquilador";
                let body = format!(
                    "Hola, {}. Tu contraseña es: {}. \nSi desea, puede cambiarla luego de iniciar sesión.",
                    payload.name, password
                );

                let send_mail_res = send_mail(&payload.email, subject, &body);
                if send_mail_res.is_ok() {
                    match transaction.commit().await {
                        Ok(_) => return (StatusCode::CREATED,
                        Json(json!({"message": "Client user successfully registered"}))),
                        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Failed to commit DB transaction",}))),
                    };
                } else {
                    return (StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Failed to send the email"})));
                }
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to save the new user"})));
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

// This handler checks if the email and password are correct
// and returns a JSON response with a message
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to connect to the DB"}))),
    };

    let row = match client
        .query_one("SELECT * FROM users WHERE email = $1;", &[&payload.email]).await {
        Ok(r) => r,
        Err(_) => return (StatusCode::BAD_REQUEST,
                    Json(json!({"message": "The user does not exist"}))),
    };

    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        name: row.get("name"),
        surname: row.get("surname"),
        psw_hash: row.get("psw_hash"),
        salt: row.get("salt"),
        role: row.get("role"),
    };

    if encode_password(&payload.password, &user.salt) != user.psw_hash {
        return (StatusCode::BAD_REQUEST, Json(json!({"message": "The password is invalid"})));
    }

    let secret_key = env::var("JWT_SECRET_KEY").expect("JTW_SECRET_KEY must be set in the .env file");

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        user_id: user.id,
        exp: expiration as usize,
        role: user.role,
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())) {
        Ok(t) => (StatusCode::OK,
                    Json(json!({"access": t}))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to create the JWT"}))),
    }
}
