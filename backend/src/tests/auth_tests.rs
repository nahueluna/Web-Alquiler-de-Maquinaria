use crate::custom_types::{enums::RunningEnv, structs::*};
use crate::helpers::auth::{create_pool, send_mail, validate_jwt, encode_password};
use crate::tests::helpers::*;
use chrono::Datelike;
use reqwest::Client;
use tokio_postgres::Error;

#[tokio::test]
async fn test_create_client() {
    setup().await;
    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    let http_client = Client::new();

    // ----------- Successful client user creation

    let successful_res = http_client
        .post(backend_url("/signup"))
        .json(&serde_json::json!({"email": "user@example.com",
        "name": "alice",
        "surname": "wonderland",
        "birth_date": "01-01-2000",
        "id_card": "12345678",
        "phone": "1234567898",
        }))
        .send()
        .await
        .unwrap();

    let successful_rows = db_client
        .query(
            "SELECT * FROM users WHERE email = $1;",
            &[&"user@example.com"],
        )
        .await
        .unwrap();

    assert_eq!(successful_res.status(), 201);

    let user_info = successful_rows.get(0).unwrap();

    // Uses indexation by column name (&str) and gets a String value
    let email = user_info.get::<&str, String>("email");

    assert_eq!(email, "user@example.com");


    // ----------- Used id_card

    let successful_res = http_client
        .post(backend_url("/signup"))
        .json(&serde_json::json!({"email": "user2@example.com",
        "name": "Jamie",
        "surname": "Kent",
        "birth_date": "02-01-2000",
        "id_card": "12345678",
        "phone": "1234567898",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(successful_res.status(), 409);
    assert_eq!(successful_res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "A user with this information already exists");

    // ----------- Conflict due to existing email

    let repeated_res = http_client
        .post(backend_url("/signup"))
        .json(&serde_json::json!({"email": "user@example.com",
        "name": "alice",
        "surname": "wonderland",
        "birth_date": "01-01-2000",
        "id_card": "12345678",
        "phone": "1234567898",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(repeated_res.status(), 409);

    let repeated_rows = db_client
        .query(
            "SELECT * FROM users WHERE email = $1;",
            &[&"user@example.com"],
        )
        .await
        .unwrap();

    assert_eq!(repeated_rows.len(), 1);

    // ----------- Forbidden due to underage

    let year = chrono::Utc::now().year() - 17;
    let birth_date = format!("01-01-{}", year);

    let forbidden_res = http_client
        .post(backend_url("/signup"))
        .json(&serde_json::json!({"email": "anotheruser@example.com",
        "name": "alice",
        "surname": "wonderland",
        "birth_date": birth_date,
        "id_card": "12345678",
        "phone": "1234567898",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(forbidden_res.status(), 403);

    // ----------- Bad request due to invalid email format

    let invalid_format_res = http_client
        .post(backend_url("/signup"))
        .json(&serde_json::json!({"email": "user@.com",
        "name": "alice",
        "surname": "wonderland",
        "birth_date": birth_date,
        "id_card": "12345678",
        "phone": "1234567898",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_format_res.status(), 400);

    // ----------- Unprocessable entity

    let unprocessable_res = http_client
        .post(backend_url("/signup"))
        .json(&serde_json::json!({}))
        .send()
        .await
        .unwrap();

    assert_eq!(unprocessable_res.status(), 422);

    // ----------- Bad request due to invalid birth date format

    let invalid_date_res = http_client
        .post(backend_url("/signup"))
        .json(&serde_json::json!({"email": "anotheruser@example.com",
        "name": "alice",
        "surname": "wonderland",
        "birth_date": "2000-01-01",
        "id_card": "12345678",
        "phone": "1234567898",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_date_res.status(), 400);
}

#[tokio::test]
async fn test_client_login() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Successful login
    let res = client
        .post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "login@example.com",
            "password": "0iRxP5lD"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let cookies = res.headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect::<Vec<_>>();
    let refresh_cookie = cookies.iter().find(|c| c.starts_with("refresh_token=")).unwrap()
        .split(';').next().and_then(|s| s.split('=').nth(1)).unwrap();
    let claims = validate_jwt(&refresh_cookie.to_string()).unwrap().claims;
    assert_eq!(10, claims.user_id);
    assert_eq!(2, claims.role);
    assert_eq!(true, claims.is_refresh);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let jwt = value["access"].as_str().unwrap();
    let claims = validate_jwt(&jwt.to_string()).unwrap().claims;
    assert_eq!(10, claims.user_id);
    assert_eq!(2, claims.role);
    let pub_user: PubUser = serde_json::from_value(value["pub_user"].clone()).unwrap();
    let user_info: Option<UserInfo> = serde_json::from_value(value["user_info"].clone()).unwrap();
    let user_info = user_info.unwrap();
    assert_eq!(user_info.id, 10);
    assert_eq!(pub_user.id, 10);
    assert_eq!(pub_user.role, 2);

    // Successful admin login
    let rows = db_client.query("SELECT * FROM codes_2fa WHERE id = $1;",
            &[&11i32]).await.unwrap();
    assert_eq!(rows.len(), 0);

    let res = client
        .post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "admin@example.com",
            "password": "password"
        })).send().await.unwrap();

    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "2FA email sent");

    let rows = db_client.query("SELECT * FROM codes_2fa WHERE id = $1;",
            &[&11i32]).await.unwrap();
    assert_eq!(rows.len(), 1);
    let code1: i32 = rows.get(0).unwrap().get("code");

    client.post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "admin@example.com",
            "password": "password"
        })).send().await.unwrap();

    let rows = db_client.query("SELECT * FROM codes_2fa WHERE id = $1;",
            &[&11i32]).await.unwrap();
    assert_eq!(rows.len(), 1);
    let code2: i32 = rows.get(0).unwrap().get("code");
    assert_ne!(code1, code2);

    let res = client
        .post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "admin@example.com",
            "password": "password",
            "code": code2
        })).send().await.unwrap();

    assert_eq!(res.status(), 200);
    let cookies = res.headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect::<Vec<_>>();
    let refresh_cookie = cookies.iter().find(|c| c.starts_with("refresh_token=")).unwrap()
        .split(';').next().and_then(|s| s.split('=').nth(1)).unwrap();
    let claims = validate_jwt(&refresh_cookie.to_string()).unwrap().claims;
    assert_eq!(11, claims.user_id);
    assert_eq!(0, claims.role);
    assert_eq!(true, claims.is_refresh);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let jwt = value["access"].as_str().unwrap();
    let claims = validate_jwt(&jwt.to_string()).unwrap().claims;
    assert_eq!(11, claims.user_id);
    assert_eq!(0, claims.role);
    let pub_user: PubUser = serde_json::from_value(value["pub_user"].clone()).unwrap();
    let user_info: Option<UserInfo> = serde_json::from_value(value["user_info"].clone()).unwrap();
    assert!(user_info.is_none());
    assert_eq!(pub_user.id, 11);
    assert_eq!(pub_user.role, 0);

    // Unauthorized login due to wrong password
    let res = client
        .post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "login@example.com",
            "password": "badpassword"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid credentials");

    // Unauthorized login due to wrong email
    let res = client
        .post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "notanuser@example.com",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid credentials");
}

#[tokio::test]
async fn test_send_email() {
    setup().await;
    send_mail(
        "recipient@example.com",
        "Hello from Rust!",
        "This email was sent securely using dotenv.",
    ).unwrap();
}

#[tokio::test]
async fn test_refresh() {
    setup().await;
    let client = Client::new();

    // Successful login
    let res = client
        .post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "refresh@example.com",
            "password": "password1"
        }))
        .send()
        .await
        .unwrap();
    //Get cookie
    let cookie_header1 = res.headers().get("set-cookie")
        .unwrap().to_owned();
    //Call refresh
    let refresh_res = client
        .post("http://localhost:8000/refresh")
        .header("Cookie", &cookie_header1)
        .send().await.unwrap();
    let cookie_header2 = refresh_res.headers().get("set-cookie")
        .unwrap().to_owned();
    assert_eq!(refresh_res.status(), 200);
    //Check the new refresh
    let cookies = refresh_res.headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect::<Vec<_>>();
    let refresh_cookie = cookies.iter().find(|c| c.starts_with("refresh_token=")).unwrap()
        .split(';').next().and_then(|s| s.split('=').nth(1)).unwrap();
    let claims = validate_jwt(&refresh_cookie.to_string()).unwrap().claims;
    assert_eq!(12, claims.user_id);
    assert_eq!(2, claims.role);
    assert_eq!(true, claims.is_refresh);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let jwt = value["access"].as_str().unwrap();
    let claims = validate_jwt(&jwt.to_string()).unwrap().claims;
    assert_eq!(12, claims.user_id);
    assert_eq!(2, claims.role);
    //Check the first refresh became invalid
    let refresh_res = client
        .post("http://localhost:8000/refresh")
        .header("Cookie", cookie_header1)
        .send().await.unwrap();
    assert_eq!(refresh_res.status(), 400);
    assert_eq!(refresh_res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Invalid refresh token");

    //When refresh is called with a valid token that is not the newest
    //It invalidates the newest, and the user need to login again
    let refresh_res = client
        .post("http://localhost:8000/refresh")
        .header("Cookie", cookie_header2)
        .send().await.unwrap();
    assert_eq!(refresh_res.status(), 400);
    assert_eq!(refresh_res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Invalid refresh token");
}

#[tokio::test]
async fn test_request_psw_change() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Successful request
    let rows = db_client.query("SELECT * FROM change_psw_codes WHERE id = $1;",
            &[&10i32]).await.unwrap();
    assert_eq!(rows.len(), 0);

    let res = client
        .post(backend_url("/requestpswchange"))
        .json(&serde_json::json!({
            "email": "login@example.com"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Password change email sent");

    db_client.query_one("SELECT * FROM change_psw_codes WHERE id = $1;",
            &[&10i32]).await.unwrap();
}

#[tokio::test]
async fn test_change_password() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    let res = client
        .post(backend_url("/changepsw"))
        .json(&serde_json::json!({
            "new_password": "short",
            "code": "code"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap().contains("Invalid input data: new_password: Validation error: length"));

    let res = client
        .post(backend_url("/changepsw"))
        .json(&serde_json::json!({
            "new_password": "password",
            "code": "change_psw_code"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Password changed successfully");

    let row = db_client.query_one("SELECT * FROM users WHERE email = $1;",
        &[&"pswchange@example.com"]).await.unwrap();

    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        name: row.get("name"),
        surname: row.get("surname"),
        psw_hash: row.get("psw_hash"),
        salt: row.get("salt"),
        role: row.get("role"),
    };

    assert_eq!(user.psw_hash, encode_password("password", &user.salt));
}

#[tokio::test]
async fn test_logout() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Successful login
    let res = client
        .post(backend_url("/login"))
        .json(&serde_json::json!({
            "email": "logout@example.com",
            "password": "password"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    //Check that the user now has a refresh token
    let row = db_client.query_one("SELECT refresh FROM users WHERE email = $1;",
            &[&"logout@example.com"]).await.unwrap();
    let refresh: String = row.get("refresh");
    assert!(!refresh.is_empty());

    //Get the access token needed for logout
    let value = res.json::<serde_json::Value>().await.unwrap();
    let jwt = value["access"].as_str().unwrap();
    //Logout
    let res = client
        .post(backend_url("/logout"))
        .json(&serde_json::json!({
            "access": jwt 
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Logout successful");

    //Check that the user's refresh token was deleted
    let row = db_client.query_one("SELECT refresh FROM users WHERE email = $1;",
            &[&"logout@example.com"]).await.unwrap();

    let refresh: Result<&str, Error> = row.try_get("refresh");
    assert!(refresh.is_err());
}

#[tokio::test]
async fn test_check_changepsw_code() {
    setup().await;
    let client = Client::new();

    //Valid code
    let res = client
        .post(backend_url("/checkchangepswcode"))
        .json(&serde_json::json!({
            "code": "change_psw_code"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["valid"].as_bool().unwrap(), true);

    //Invalid code
    let res = client
        .post(backend_url("/checkchangepswcode"))
        .json(&serde_json::json!({
            "code": "invalid_code"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["valid"].as_bool().unwrap(), false);
}

#[tokio::test]
async fn test_register_get_and_delete_employees() {
    setup().await;
    let client = Client::new();

    //Get the access token needed for get_employees
    let jwt = get_test_jwt("admin2@example.com", false).await;
    //get_employees
    let res = client
        .post(backend_url("/getemployees"))
        .json(&serde_json::json!({
            "access": jwt 
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["employees"].clone();
    let employees: Vec<PubUserWithInfo> = serde_json::from_value(value).unwrap();
    //Increment if more employees are added to the test DB
    assert_eq!(employees.len(), 2);
    assert_eq!(employees[0].email, "bob@example.com");
    assert_eq!(employees[0].id, 2);
    assert_eq!(employees[0].role, 1);
    assert_eq!(employees[0].id_card, "ID234567");
    assert_eq!(employees[1].email, "frank@example.com");
    assert_eq!(employees[1].role, 1);
    assert_eq!(employees[1].id_card, "ID678901");

    //Delete bob
    let res = client
        .post(backend_url("/deletemployee"))
        .json(&serde_json::json!({
            "access": jwt,
            "id": 2
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Employee deleted successfully");

    //get_employees
    let res = client
        .post(backend_url("/getemployees"))
        .json(&serde_json::json!({
            "access": jwt
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["employees"].clone();
    let employees: Vec<PubUserWithInfo> = serde_json::from_value(value).unwrap();
    //Increment if more employees are added to the test DB
    assert_eq!(employees.len(), 1);
    assert_eq!(employees[0].email, "frank@example.com");
    assert_eq!(employees[0].role, 1);
    assert_eq!(employees[0].id_card, "ID678901");

    //Register new employee
    let res = client
        .post(backend_url("/registeremployee"))
        .json(&serde_json::json!({
            "email": "emp1@example.com",
            "name": "James",
            "surname": "Bond",
            "birthdate": "02-02-2000",
            "id_card": "GAAAAAA",
            "phone": "GAAAAA",
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Employee registered successfully");

    //get_employees
    let res = client
        .post(backend_url("/getemployees"))
        .json(&serde_json::json!({
            "access": jwt
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["employees"].clone();
    let employees: Vec<PubUserWithInfo> = serde_json::from_value(value).unwrap();
    //Increment if more employees are added to the test DB
    assert_eq!(employees.len(), 2);
    assert_eq!(employees[1].email, "emp1@example.com");
    assert_eq!(employees[1].role, 1);
    assert_eq!(employees[1].id_card, "GAAAAAA");

    //Used email
    let res = client
        .post(backend_url("/registeremployee"))
        .json(&serde_json::json!({
            "email": "emp1@example.com",
            "name": "James",
            "surname": "Bond",
            "birthdate": "02-02-2000",
            "id_card": "NEWIDHELLO",
            "phone": "GAAAAA",
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 409);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "A user with this information already exists");

    //Used id_card
    let res = client
        .post(backend_url("/registeremployee"))
        .json(&serde_json::json!({
            "email": "emp1@example.com",
            "name": "James",
            "surname": "Bond",
            "birthdate": "02-02-2000",
            "id_card": "GAAAAAA",
            "phone": "GAAAAA",
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 409);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "A user with this information already exists");
}

#[tokio::test]
async fn test_change_phone() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    //Dave has no number
    db_client.query_one("SELECT * FROM users JOIN user_info
        ON users.id = user_info.id WHERE email = $1 AND phone IS NULL;",
            &[&"dave@example.com"]).await.unwrap();

    let jwt = get_test_jwt("dave@example.com", false).await;
    //Change phone number
    let res = client
        .post(backend_url("/changephone"))
        .json(&serde_json::json!({
            "access": jwt,
            "phone": "newnumber"
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Phone number changed successfully");

    //Dave now has a number
    db_client.query_one("SELECT * FROM users JOIN user_info
        ON users.id = user_info.id WHERE email = $1 AND phone = $2;",
            &[&"dave@example.com", &"newnumber"]).await.unwrap();
}

