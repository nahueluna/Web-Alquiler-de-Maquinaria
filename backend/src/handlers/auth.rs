use axum::{http::StatusCode, Json, extract::State};
use validator::Validate;
use serde_json::json;
use crate::custom_types::structs::*;
use crate::helpers::auth::*;

// basic handler that responds with a static string
pub async fn root(State(state): State<AppState>) -> &'static str {
    if let Ok(client) = state.pool.get().await {
        let rows = client.query("SELECT * FROM users WHERE id = 1;", &[]).await.unwrap();
        let email: String = rows.get(0).unwrap().get("email");
        println!("success: {:?}", email);
    } else {
        println!("error");
    }
    "Hello, World!"
}

pub async fn client_sign_up(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateRegularUser` type
    State(state): State<AppState>,
    payload_result: Result<Json<CreateRegularUser>, axum::extract::rejection::JsonRejection>,
) -> (StatusCode, Json<serde_json::Value>) {
    let payload = match payload_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Deserialization error: {:?}", e);
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "message": "Invalid JSON format",
                })),
            );
        }
    };

    if let Err(e) = payload.validate() {
        eprintln!("Validation error: {:?}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Invalid input data",
            })),
        );
    }

    if let Ok(client) = state.pool.get().await {
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
                let subject = "Contraseña generada para sistema de Bob el Alquilador";
                let body = format!(
                    "Hola, {}. Tu contraseña es: {}. \nSi desea, puede cambiarla luego de iniciar sesión.",
                    payload.name, password
                );
                send_mail(&payload.email, subject, &body);

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
