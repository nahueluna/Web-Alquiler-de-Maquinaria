use axum::{extract::State, http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response}, Json};
use deadpool_postgres::GenericClient;
use validator::Validate;
use serde_json::json;
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
                    "INSERT INTO user_info (id, birthdate, id_card, phone) VALUES (currval('users_id_seq'), $1, $2, $3)",
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
) -> Response {
    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to connect to the DB"}))).into_response(),
    };

    let row = match client
        .query_one("SELECT * FROM users WHERE email = $1;", &[&payload.email]).await {
        Ok(r) => r,
        Err(_) => return (StatusCode::BAD_REQUEST,
                    Json(json!({"message": "The user does not exist"}))).into_response(),
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
        return (StatusCode::BAD_REQUEST, Json(json!({"message": "The password is invalid"}))).into_response();
    }

    let pub_user = PubUser::from(user);

    let user_info = if pub_user.role != 2 {
        let row = match client
            .query_one("SELECT * FROM user_info WHERE id = $1;", &[&pub_user.id]).await {
            Ok(r) => r,
            Err(_) => return (StatusCode::BAD_REQUEST,
                        Json(json!({"message": "The user does not exist"}))).into_response(),
        };
        Some(UserInfo {
            id: row.get("id"),
            birthdate: row.get("birthdate"),
            id_card: row.get("id_card"),
            phone: row.get("phone"),
        })
    } else {
        None
    };


    if pub_user.role != 2 {
        if let Some(code) = payload.code {
            match client
                .query_one("SELECT * FROM codes_2fa WHERE id = $1 AND code = $2;",
                    &[&pub_user.id, &code]).await {
                Ok(r) => r,
                Err(_) => return (StatusCode::BAD_REQUEST,
                            Json(json!({"message": "The code provided is invalid"}))).into_response(),
            };
        } else {
            let code = create_2fa_code();
            let subject = "Verificación en dos pasos - Bob el Alquilador";
            let body = format!(
                "Hola, {}. Su código de verificación de dos pasos es: {}.\nSi solicitó más de un código, solo el último que haya recibido es válido.",
                pub_user.name, code
            );

            let send_mail_res = send_mail(&payload.email, subject, &body);

            if send_mail_res.is_err() {
                return (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to send the email"}))).into_response();
            }

            let del_q = client.execute("DELETE FROM codes_2fa WHERE id = $1;",
                &[&pub_user.id]).await;

            let ins_q = client.execute("INSERT INTO codes_2fa (id, code) VALUES ($1, $2);",
                &[&pub_user.id, &code]).await;

            if del_q.is_ok() && ins_q.is_ok() {
                return (StatusCode::OK,
                    Json(json!({"message": "2FA email sent"}))).into_response();
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to save the 2FA code"}))).into_response();
            }
        }
    }

    let access = match generate_jwt(pub_user.id, pub_user.role, false) {
        Ok(a) => a,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed create the JWT"}))).into_response(),
    };
    let refresh = match generate_jwt(pub_user.id, pub_user.role, true) {
        Ok(a) => a,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed create the JWT"}))).into_response(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={}; HttpOnly; SameSite=Lax; Path=/refresh",
            refresh
        ))
        .unwrap(),
    );

    let body = Json(json!({"access": access,
        "pub_user": pub_user,"user_info": user_info}));

    (StatusCode::OK, headers, body).into_response()
}
