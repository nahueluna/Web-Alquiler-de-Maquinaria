#[cfg(test)]
mod tests {
    use crate::tests::helpers::setup;
    use reqwest::Client;
    use validator::ValidateLength;

    #[tokio::test]
    async fn test_explore_catalog() {
        setup().await;
        let http_client = Client::new();

        // ----------- Page 1, Page size 2, valid request without filters

        let basic_page1_res = http_client
            .get("http://localhost:8000/explore")
            .query(&[("page", "1"), ("page_size", "2")])
            .send()
            .await
            .unwrap();

        assert_eq!(basic_page1_res.status(), 200);

        let response_body = basic_page1_res.json::<serde_json::Value>().await.unwrap();

        assert_eq!(response_body["items"].as_array().length(), Some(2));

        let machine1 = &response_body["items"][0];
        let machine2 = &response_body["items"][1];

        assert_eq!(machine1.as_object().unwrap().get("id").unwrap(), 1);
        assert_eq!(machine2.as_object().unwrap().get("model").unwrap(), "310SL");

        // ----------- Page 2, Page size 2, valid request with filters

        let basic_page2_res = http_client
            .get("http://localhost:8000/explore")
            .query(&[("page", "2"), ("page_size", "2")])
            .send()
            .await
            .unwrap();

        assert_eq!(basic_page2_res.status(), 200);

        let response_body = basic_page2_res.json::<serde_json::Value>().await.unwrap();

        assert_eq!(response_body["items"].as_array().length(), Some(2));

        let machine3 = &response_body["items"][0];
        let machine4 = &response_body["items"][1];

        assert_eq!(machine3.as_object().unwrap().get("id").unwrap(), 3);
        assert_eq!(machine4.as_object().unwrap().get("model").unwrap(), "E35");

        // ----------- Page default, Page size 5, valid request with search term, minimum price and order

        let basic_page2_res = http_client
            .get("http://localhost:8000/explore")
            .query(&[
                ("page", "1"),
                ("page_size", "5"),
                ("search", "excavadora"),
                ("min_price", "80000"),
                ("order_by", "price"),
                ("order_dir", "asc"),
            ])
            .send()
            .await
            .unwrap();

        assert_eq!(basic_page2_res.status(), 200);

        let response_body = basic_page2_res.json::<serde_json::Value>().await.unwrap();

        assert_eq!(response_body["items"].as_array().length(), Some(2));

        let machine5 = &response_body["items"][0];
        let machine6 = &response_body["items"][1];

        assert_eq!(machine5.as_object().unwrap().get("id").unwrap(), 2);
        assert_eq!(
            machine6.as_object().unwrap().get("model").unwrap(),
            "CAT320D"
        );

    }
}
