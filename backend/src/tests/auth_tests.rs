#[cfg(test)]
mod tests {
    use crate::custom_types::{enums::RunningEnv, structs::{UserInfo, User}};
    use crate::helpers::auth::{create_pool, send_mail, validate_jwt};
    use crate::tests::helpers::*;
    use chrono::Datelike;
    use reqwest::Client;

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
        let value = res.json::<serde_json::Value>().await.unwrap();
        let jwt = value["access"].as_str().unwrap();
        let claims = validate_jwt(&jwt.to_string()).unwrap().claims;
        assert_eq!(10, claims.user_id);
        assert_eq!(2, claims.role);
        let user: User = serde_json::from_value(value["user"].clone()).unwrap();
        let user_info: Option<UserInfo> = serde_json::from_value(value["user_info"].clone()).unwrap();
        assert!(user_info.is_none());
        assert_eq!(user.id, 10);
        assert_eq!(user.role, 2);

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
        let value = res.json::<serde_json::Value>().await.unwrap();
        let jwt = value["access"].as_str().unwrap();
        let claims = validate_jwt(&jwt.to_string()).unwrap().claims;
        assert_eq!(11, claims.user_id);
        assert_eq!(0, claims.role);
        let user: User = serde_json::from_value(value["user"].clone()).unwrap();
        let user_info: Option<UserInfo> = serde_json::from_value(value["user_info"].clone()).unwrap();
        let user_info = user_info.unwrap();
        assert_eq!(user_info.id, 11);
        assert_eq!(user.id, 11);
        assert_eq!(user.role, 0);

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
        assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The password is invalid");

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
        assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The user does not exist");
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
}
