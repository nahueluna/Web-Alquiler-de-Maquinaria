#[cfg(test)]
mod tests {
    use reqwest::Client;
    use crate::handlers::auth::send_mail;

    #[tokio::test]
    async fn test_create_user() {
        let client = Client::new();
        let res = client
            .post("http://localhost:8000/users")
            .json(&serde_json::json!({ "username": "alice" }))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 201);
        let json: serde_json::Value = res.json().await.unwrap();
        assert_eq!(json["username"], "alice");
        assert_eq!(json["id"], 1337);
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
