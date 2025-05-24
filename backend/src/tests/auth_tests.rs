#[cfg(test)]
mod tests {
    use crate::custom_types::enums::RunningEnv;
    use crate::handlers::auth::{create_pool, send_mail};
    use crate::tests::helpers::setup;
    use chrono::Datelike;
    use reqwest::Client;

    #[tokio::test]
    async fn test_pool() {
        setup().await;
        // Successful client user creation
        let client = Client::new();
        let res = client.get("http://localhost:8000/").send().await.unwrap();
        println!("{:?}", res);
    }

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

        let succesful_res = http_client
            .post("http://localhost:8000/signup")
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

        assert_eq!(succesful_res.status(), 201);

        let user_info = successful_rows.get(0).unwrap();

        // Uses indexation by column name (&str) and gets a String value
        let email = user_info.get::<&str, String>("email");

        assert_eq!(email, "user@example.com");

        // ----------- Conflict due to existing email

        let repeated_res = http_client
            .post("http://localhost:8000/signup")
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
            .post("http://localhost:8000/signup")
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
            .post("http://localhost:8000/signup")
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
            .post("http://localhost:8000/signup")
            .json(&serde_json::json!({}))
            .send()
            .await
            .unwrap();

        assert_eq!(unprocessable_res.status(), 422);

        // ----------- Bad request due to invalid birth date format

        let invalid_date_res = http_client
            .post("http://localhost:8000/signup")
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

        // Correct login
        let res = client
            .post("http://localhost:8000/login")
            .json(&serde_json::json!({
                "email": "user@example.com",
                "password": "password123"
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), 200);

        // Unauthorized login due to wrong password
        let res = client
            .post("http://localhost:8000/login")
            .json(&serde_json::json!({
                "email": "user@example.com",
                "password": "notapassword"
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), 401);

        // Unauthorized login due to wrong email
        let res = client
            .post("http://localhost:8000/login")
            .json(&serde_json::json!({
                "email": "notauser@example.com",
                "password": "password123"
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), 401);
    }

    #[tokio::test]
    async fn test_send_email() {
        setup().await;
        send_mail(
            "recipient@example.com",
            "Hello from Rust!",
            "This email was sent securely using dotenv.",
        );
    }
}
