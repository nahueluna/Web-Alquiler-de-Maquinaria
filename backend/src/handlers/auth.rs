use crate::constants::CHANGE_PSW_CODE_EXP_MINS;
use crate::custom_types::structs::*;
use crate::helpers::auth::*;
use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::TypedHeader;
use deadpool_postgres::GenericClient;
use headers::Cookie;
use hex;
use rand::RngCore;
use serde_json::json;
use std::env;
use tokio_postgres::error::SqlState;
use validator::Validate;

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
        let birth_date = match chrono::NaiveDate::parse_from_str(&payload.birth_date, "%d-%m-%Y") {
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
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to create a DB transaction",})),
                )
            }
        };

        let password = generate_random_string(8);
        let salt = generate_random_string(16);
        let hashed_password = encode_password(&password, &salt);

        if let Err(e) = transaction.execute(
               "INSERT INTO users (email, name, surname, psw_hash, salt, role, status) VALUES ($1, $2, $3, $4, $5, 2, 'active');",
                &[&payload.email,&payload.name,&payload.surname,
                  &hashed_password,&salt]).await {
            if let Some(db_err) = e.as_db_error() {
                if db_err.code() == &SqlState::UNIQUE_VIOLATION && db_err.message().contains("email") {
                    return (StatusCode::CONFLICT,
                        Json(json!({"message": "A user with this information already exists"})));
                    }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to execute transaction"})));
        }

        if let Err(e) = transaction.execute(
                "INSERT INTO user_info (id, birthdate, id_card, phone) VALUES (currval('users_id_seq'), $1, $2, $3)",
                &[&birth_date, &payload.id_card, &payload.phone],
                ).await {
            if let Some(db_err) = e.as_db_error() {
                if db_err.code() == &SqlState::UNIQUE_VIOLATION && db_err.message().contains("id_card") {
                    return (StatusCode::CONFLICT,
                        Json(json!({"message": "A user with this information already exists"})));
                    }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to execute transaction"})));
        }

        let subject = "Contraseña generada para sistema de Bob el Alquilador";
        let body = format!(
            "Hola, {}. Tu contraseña es: {}. \nSi desea, puede cambiarla luego de iniciar sesión.",
            payload.name, password
        );

        let send_mail_res = send_mail(&payload.email, subject, &body);
        if send_mail_res.is_ok() {
            match transaction.commit().await {
                Ok(_) => {
                    return (
                        StatusCode::CREATED,
                        Json(json!({"message": "Client user successfully registered"})),
                    )
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Failed to commit DB transaction",})),
                    )
                }
            };
        } else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to send the email"})),
            );
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Error connecting to database"})),
        )
    }
}

// This handler checks if the email and password are correct
// and returns a JSON response with a message
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Response {
    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    let row = match client
        .query_one(
            "SELECT * FROM users WHERE email = $1 AND status = 'active';",
            &[&payload.email],
        )
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid credentials"})),
            )
                .into_response()
        }
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
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid credentials"})),
        )
            .into_response();
    }

    let pub_user = PubUser::from(user);

    //Clients(2) and Employees(1) have user_info
    let user_info = if pub_user.role != 0 {
        let row = match client
            .query_one("SELECT * FROM user_info WHERE id = $1;", &[&pub_user.id])
            .await
        {
            Ok(r) => r,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "An unexpected error has occurred"})),
                )
                    .into_response()
            }
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
                .query_one(
                    "SELECT * FROM codes_2fa WHERE id = $1 AND code = $2;",
                    &[&pub_user.id, &code],
                )
                .await
            {
                Ok(r) => r,
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"message": "The code provided is invalid"})),
                    )
                        .into_response()
                }
            };
        } else {
            let code = create_2fa_code() as i32;
            let subject = "Verificación en dos pasos - Bob el Alquilador";
            let body = format!(
                "Hola, {}. Su código de verificación de dos pasos es: {:06}.\nSi solicitó más de un código, solo el último que haya recibido es válido.",
                pub_user.name, code
            );

            let send_mail_res = send_mail(&payload.email, subject, &body);

            if send_mail_res.is_err() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to send the email"})),
                )
                    .into_response();
            }

            let del_q = client
                .execute("DELETE FROM codes_2fa WHERE id = $1;", &[&pub_user.id])
                .await;

            let ins_q = client
                .execute(
                    "INSERT INTO codes_2fa (id, code) VALUES ($1, $2);",
                    &[&pub_user.id, &code],
                )
                .await;

            if del_q.is_ok() && ins_q.is_ok() {
                return (StatusCode::OK, Json(json!({"message": "2FA email sent"})))
                    .into_response();
            } else {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Failed to save the 2FA code"})),
                )
                    .into_response();
            }
        }
    }

    let access = match generate_jwt(pub_user.id, pub_user.role, false) {
        Ok(a) => a,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed create the JWT"})),
            )
                .into_response()
        }
    };
    let refresh = match generate_jwt(pub_user.id, pub_user.role, true) {
        Ok(a) => a,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed create the JWT"})),
            )
                .into_response()
        }
    };

    //Save the refresh
    if client
        .execute(
            "UPDATE users SET refresh = $1
            WHERE id = $2;",
            &[&refresh, &pub_user.id],
        )
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to save the refresh token"})),
        )
            .into_response();
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={}; HttpOnly; SameSite=None; Secure; Path=/refresh",
            refresh
        ))
        .unwrap(),
    );

    let body = Json(json!({"access": access,
        "pub_user": pub_user,"user_info": user_info}));

    (StatusCode::OK, headers, body).into_response()
}

pub async fn refresh(
    State(state): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Response {
    let refresh_token = match cookie.get("refresh_token") {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Missing refresh token"})),
            )
                .into_response()
        }
    };

    let claims = match validate_jwt(refresh_token) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid refresh token"})),
            )
                .into_response()
        }
    }
    .claims;

    if !claims.is_refresh {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"message": "Invalid token type"})),
        )
            .into_response();
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    //If the token is valid but it was rotated, delete the current token
    if client
        .query_one(
            "SELECT * FROM users WHERE
        id = $1 AND refresh = $2;",
            &[&claims.user_id, &refresh_token],
        )
        .await
        .is_err()
    {
        let _ = client
            .execute(
                "UPDATE users SET refresh = NULL
        WHERE id = $1;",
                &[&claims.user_id],
            )
            .await;
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid refresh token"})),
        )
            .into_response();
    };

    let new_access = match generate_jwt(claims.user_id, claims.role, false) {
        Ok(a) => a,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed create the JWT"})),
            )
                .into_response()
        }
    };
    let new_refresh = match generate_jwt(claims.user_id, claims.role, true) {
        Ok(a) => a,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed create the JWT"})),
            )
                .into_response()
        }
    };

    //Save the new refresh
    if client
        .execute(
            "UPDATE users SET refresh = $1
            WHERE id = $2;",
            &[&new_refresh, &claims.user_id],
        )
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to save the refresh token"})),
        )
            .into_response();
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={}; HttpOnly; SameSite=None; Secure; Path=/refresh",
            new_refresh
        ))
        .unwrap(),
    );

    let body = Json(json!({"access": new_access}));

    (StatusCode::OK, headers, body).into_response()
}

pub async fn request_psw_change(
    State(state): State<AppState>,
    Json(payload): Json<Email>,
) -> Response {
    let frontend_url = match env::var("FRONTEND_URL") {
        Ok(e) => e,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "FRONTEND_URL must be set in the .env file"})),
            )
                .into_response()
        }
    };

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    let row = match client
        .query_one("SELECT * FROM users WHERE email = $1;", &[&payload.email])
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "The user does not exist"})),
            )
                .into_response()
        }
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

    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let code = hex::encode(bytes);

    let del_q = client
        .execute("DELETE FROM change_psw_codes WHERE id = $1;", &[&user.id])
        .await;

    let ins_q = client
        .execute(
            "INSERT INTO change_psw_codes (id, code) VALUES ($1, $2);",
            &[&user.id, &code],
        )
        .await;

    if del_q.is_ok() && ins_q.is_ok() {
        let subject = "Cambio de contraseña - Bob el Alquilador";
        let body = format!("Hola, {}. Para establecer una nueva contraseña, siga el siguiente enlace.: {}/changepsw/{}.",
            user.name, frontend_url, code
        );

        let send_mail_res = send_mail(&payload.email, subject, &body);

        if send_mail_res.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to send the email"})),
            )
                .into_response();
        } else {
            return (
                StatusCode::OK,
                Json(json!({"message": "Password change email sent"})),
            )
                .into_response();
        }
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to save the code"})),
        )
            .into_response();
    }
}

pub async fn change_password(
    State(state): State<AppState>,
    Json(payload): Json<ChangePsw>,
) -> Response {
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
            "message": &format!("Invalid input data: {}",e)})),
        )
            .into_response();
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    let row = match client
        .query_one(
            "SELECT * FROM change_psw_codes WHERE code = $1
        AND created_at > NOW() - make_interval(mins => $2::int);",
            &[&payload.code, &CHANGE_PSW_CODE_EXP_MINS],
        )
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "The code is invalid or has expired"})),
            )
                .into_response()
        }
    };

    let id: i32 = row.get("id");

    let row = match client
        .query_one("SELECT * FROM users WHERE id = $1;", &[&id])
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Failed to retrieve the user from the DB"})),
            )
                .into_response()
        }
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

    let salt = generate_random_string(16);
    let hashed_password = encode_password(&payload.new_password, &salt);

    if client
        .execute(
            "UPDATE users SET psw_hash = $1, salt = $2 WHERE id = $3;",
            &[&hashed_password, &salt, &user.id],
        )
        .await
        .is_ok()
    {
        //Delete the used code
        let _ = client
            .execute("DELETE FROM change_psw_codes WHERE id = $1;", &[&user.id])
            .await;

        return (
            StatusCode::OK,
            Json(json!({"message": "Password changed successfully"})),
        )
            .into_response();
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to change the password"})),
        )
            .into_response();
    }
}

pub async fn logout(State(state): State<AppState>, Json(payload): Json<Access>) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    match client
        .execute(
            "UPDATE users SET refresh = NULL
        WHERE id = $1;",
            &[&claims.user_id],
        )
        .await
    {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({"message": "Logout successful"})),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to delete the user's refresh token"})),
            )
                .into_response()
        }
    };
}

pub async fn check_changepsw_code(
    State(state): State<AppState>,
    Json(payload): Json<CheckChangePswCode>,
) -> Response {
    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    match client
        .query(
            "SELECT * FROM change_psw_codes WHERE code = $1;",
            &[&payload.code],
        )
        .await
    {
        Ok(rows) => {
            if rows.len() == 1 {
                return (StatusCode::OK, Json(json!({"valid": true}))).into_response();
            } else {
                return (StatusCode::OK, Json(json!({"valid": false}))).into_response();
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to search the code"})),
            )
                .into_response()
        }
    }
}

pub async fn get_employees(State(state): State<AppState>, Json(payload): Json<Access>) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role != 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "The user is not an admin"})),
        )
            .into_response();
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    match client
        .query(
            "SELECT * FROM users JOIN user_info ON users.id = user_info.id
        WHERE users.role = 1 AND status = 'active';",
            &[],
        )
        .await
    {
        Ok(rows) => {
            let employees: Vec<PubUserWithInfo> = rows
                .iter()
                .map(|row| PubUserWithInfo {
                    id: row.get("id"),
                    email: row.get("email"),
                    name: row.get("name"),
                    surname: row.get("surname"),
                    role: row.get("role"),
                    birthdate: row.get("birthdate"),
                    id_card: row.get("id_card"),
                    phone: row.get("phone"),
                })
                .collect();
            return (StatusCode::OK, Json(json!({"employees": employees}))).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to get the employees"})),
            )
                .into_response()
        }
    };
}

pub async fn delete_employee(
    State(state): State<AppState>,
    Json(payload): Json<DeleteEmployee>,
) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role != 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "The user is not an admin"})),
        )
            .into_response();
    }

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    match client
        .execute(
            "UPDATE users SET status = 'deleted' WHERE id = $1 AND role = 1",
            &[&payload.id],
        )
        .await
    {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({"message": "Employee deleted successfully"})),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to delete employee"})),
            )
                .into_response()
        }
    };
}

pub async fn register_employee(
    State(state): State<AppState>,
    Json(payload): Json<CreateEmployee>,
) -> Response {
    if let Err(_) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid input data"})),
        )
            .into_response();
    }

    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    if claims.role != 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "The user is not an admin"})),
        )
            .into_response();
    }

    let mut client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    let transaction = match client.transaction().await {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to create a DB transaction",})),
            )
                .into_response()
        }
    };

    let password = generate_random_string(8);
    let salt = generate_random_string(16);
    let hashed_password = encode_password(&password, &salt);

    let birthdate = match chrono::NaiveDate::parse_from_str(&payload.birthdate, "%d-%m-%Y") {
        Ok(date) => date,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid birth date format"})),
            )
                .into_response()
        }
    };

    if !is_adult(birthdate) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"message": "User age is less than 18"})),
        )
            .into_response();
    }

    let row = match transaction
        .query_one(
            "INSERT INTO users (email, name, surname, psw_hash, salt, role, refresh, status)
        VALUES ($1, $2, $3, $4, $5, 1, NULL, 'active') RETURNING id;",
            &[
                &payload.email,
                &payload.name,
                &payload.surname,
                &hashed_password,
                &salt,
            ],
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            if let Some(db_err) = e.as_db_error() {
                if db_err.code() == &SqlState::UNIQUE_VIOLATION
                    && db_err.message().contains("email")
                {
                    return (
                        StatusCode::CONFLICT,
                        Json(json!({"message": "A user with this information already exists"})),
                    )
                        .into_response();
                }
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to execute transaction"})),
            )
                .into_response();
        }
    };

    let user_id: i32 = row.get("id");

    if let Err(e) = transaction
        .execute(
            "INSERT INTO user_info (id, birthdate, id_card, phone)
        VALUES ($1, $2, $3, $4);",
            &[&user_id, &birthdate, &payload.id_card, &payload.phone],
        )
        .await
    {
        if let Some(db_err) = e.as_db_error() {
            if db_err.code() == &SqlState::UNIQUE_VIOLATION && db_err.message().contains("id_card")
            {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({"message": "A user with this information already exists"})),
                )
                    .into_response();
            }
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to execute transaction"})),
        )
            .into_response();
    }

    let subject = "Contraseña generada para sistema de Bob el Alquilador";
    let body = format!(
        "Hola, {}. Tu contraseña es: {}. \nSi desea, puede cambiarla luego de iniciar sesión.",
        payload.name, password
    );

    if send_mail(&payload.email, subject, &body).is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to send the email"})),
        )
            .into_response();
    }

    match transaction.commit().await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({"message": "Employee registered successfully"})),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to commit transaction"})),
            )
                .into_response()
        }
    };
}

pub async fn change_phone(
    State(state): State<AppState>,
    Json(payload): Json<ChangePhone>,
) -> Response {
    let claims = match validate_jwt(&payload.access) {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid access token"})),
            )
                .into_response()
        }
    }
    .claims;

    let client = match state.pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to connect to the DB"})),
            )
                .into_response()
        }
    };

    match client
        .execute(
            "UPDATE user_info SET phone = $1
        WHERE id = $2;",
            &[&payload.phone, &claims.user_id],
        )
        .await
    {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({"message": "Phone number changed successfully"})),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to change the phone number"})),
            )
                .into_response()
        }
    };
}
