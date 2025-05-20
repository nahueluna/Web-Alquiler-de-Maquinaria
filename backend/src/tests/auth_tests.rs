#[cfg(test)]
mod tests {
    use crate::handlers::auth::{connect_db, send_mail};
    use reqwest::Client;

    #[tokio::test]
    async fn test_create_client() {
        let db_client = match connect_db().await {
            Ok(c) => c,
            Err(e) => panic!("Failed to connect to the database: {}", e),
        };

        // Successful client user creation
        let client = Client::new();
        let res = client
            .post("http://localhost:8000/register")
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

        let rows = db_client
            .query(
                "SELECT * FROM users WHERE email = $1;",
                &[&"user@example.com"],
            )
            .await
            .unwrap();

        db_client
            .query("DELETE FROM users WHERE email='user@example.com';", &[])
            .await
            .unwrap();

        assert_eq!(res.status(), 201);

        let json: serde_json::Value = res.json().await.unwrap();
        assert_eq!(json["message"], "Client user successfully registered");

        let user_info = rows.get(0).unwrap();

        // Uses indexation by column name (&str) and gets a String value
        let email = user_info.get::<&str, String>("email");

        assert_eq!(email, "user@example.com");
    }

    #[tokio::test]
    async fn test_client_login() {
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
        send_mail();
    }
}
