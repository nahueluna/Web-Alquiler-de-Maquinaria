#[cfg(test)]
mod tests {
    use reqwest::Client;

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
}
