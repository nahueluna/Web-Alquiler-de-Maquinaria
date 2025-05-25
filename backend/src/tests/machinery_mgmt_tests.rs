#[cfg(test)]
mod tests {
    use crate::tests::helpers::setup;
    use reqwest::Client;
    use validator::ValidateLength;

    #[tokio::test]
    async fn test_explore_catalog() {
        setup().await;
        let http_client = Client::new();

        // ----------- Page 1, Page size 3, valid request without filters

        let basic_page1_res = http_client
            .get("http://localhost:8000/explore")
            .query(&[("page", "1"), ("page_size", "3")])
            .send()
            .await
            .unwrap();

        assert_eq!(basic_page1_res.status(), 200);

        let response_body = basic_page1_res.json::<serde_json::Value>().await.unwrap();

        assert_eq!(response_body["items"].as_array().length(), Some(3));

        let machine1 = &response_body["items"][0];
        let machine2 = &response_body["items"][1];

        assert_eq!(machine1.as_object().unwrap().get("id").unwrap(), 1);
        assert_eq!(
            machine2.as_object().unwrap().get("serial_number").unwrap(),
            "SN1002"
        );

        // ----------- Page 2, Page size 3, valid request with filters

        let basic_page2_res = http_client
            .get("http://localhost:8000/explore")
            .query(&[("page", "2"), ("page_size", "3")])
            .send()
            .await
            .unwrap();

        assert_eq!(basic_page2_res.status(), 200);

        let response_body = basic_page2_res.json::<serde_json::Value>().await.unwrap();

        assert_eq!(response_body["items"].as_array().length(), Some(3));

        let machine1 = &response_body["items"][0];
        let machine2 = &response_body["items"][1];

        assert_eq!(machine1.as_object().unwrap().get("id").unwrap(), 4);
        assert_eq!(
            machine2.as_object().unwrap().get("serial_number").unwrap(),
            "SN1005"
        );
    }
}
