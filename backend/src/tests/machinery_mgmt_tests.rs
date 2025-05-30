use crate::tests::helpers::*;
use crate::helpers::auth::create_pool;
use crate::custom_types::enums::RunningEnv;
use reqwest::Client;
use validator::ValidateLength;
use std::{fs::File, io::Read};
use base64::{engine::general_purpose::STANDARD, Engine};

#[tokio::test]
async fn test_explore_catalog() {
    setup().await;
    let http_client = Client::new();

    // ----------- Page 1, Page size 2, valid request without filters

    let basic_page1_res = http_client
        .get(backend_url("/explore"))
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
    assert_eq!(
        machine2
            .as_object()
            .unwrap()
            .get("categories")
            .unwrap()
            .as_array()
            .unwrap()[0]["name"],
        "obras urbanas"
    );

    // ----------- Page 2, Page size 2, valid request without filters

    let basic_page2_res = http_client
        .get(backend_url("/explore"))
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
        .get(backend_url("/explore"))
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

    // ----------- Page 1, Page size 2, valid request with categories filter

    let categories_filter = vec![
        ("category", "obras urbanas"),
        ("category", "movimiento de tierra"),
    ];

    let categories_res = http_client
        .get(backend_url("/explore"))
        .query(&[("page", "1"), ("page_size", "3")])
        .query(&categories_filter)
        .send()
        .await
        .unwrap();

    assert_eq!(categories_res.status(), 200);

    let response_body = categories_res.json::<serde_json::Value>().await.unwrap();

    assert_eq!(response_body["items"].as_array().length(), Some(3));

    let machine7 = &response_body["items"][0];
    let machine8 = &response_body["items"][1];
    let machine9 = &response_body["items"][2];

    let machine9_price = machine9
        .as_object()
        .unwrap()
        .get("price")
        .unwrap()
        .as_f64()
        .unwrap();

    assert_eq!(machine7.as_object().unwrap().get("id").unwrap(), 2);
    assert_eq!(
        machine8.as_object().unwrap().get("brand").unwrap(),
        "Komatsu"
    );
    assert_eq!(machine9_price, 75000.0);
    assert_eq!(
        machine9
            .as_object()
            .unwrap()
            .get("categories")
            .unwrap()
            .as_array()
            .unwrap()[0]["name"],
        "movimiento de tierra"
    );

    // ----------- Page 1, Page size 2, valid request with categories filter, search term, maximum price and order

    let categories_filter = vec![
        ("category", "compactacion"),
        ("category", "elevacion"),
        ("category", "movimiento de tierra"),
    ];

    let categories_res = http_client
        .get(backend_url("/explore"))
        .query(&[
            ("page", "1"),
            ("page_size", "3"),
            ("search", "excavadora"),
            ("max_price", "90000"),
            ("order_by", "price"),
            ("order_dir", "desc"),
        ])
        .query(&categories_filter)
        .send()
        .await
        .unwrap();

    assert_eq!(categories_res.status(), 200);

    let response_body = categories_res.json::<serde_json::Value>().await.unwrap();

    assert_eq!(response_body["items"].as_array().length(), Some(1));

    let machine10 = &response_body["items"][0];

    assert_eq!(machine10.as_object().unwrap().get("id").unwrap(), 4);
    assert_eq!(
        machine10.as_object().unwrap().get("brand").unwrap(),
        "Bobcat"
    );

    // ----------- Page 1, Page size 2, invalid request with non-existing search term

    let invalid_categories_filter = vec![("category", "non-existing-category")];

    let invalid_categories_res = http_client
        .get(backend_url("/explore"))
        .query(&[
            ("page", "1"),
            ("page_size", "2"),
            ("search", "non-existing"),
        ])
        .query(&invalid_categories_filter)
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_categories_res.status(), 200);

    let response_body = invalid_categories_res
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(response_body["total_items"].as_i64().unwrap(), 0);

    // ----------- Page 999, Page size 2, invalid request with no results

    let invalid_page_res = http_client
        .get(backend_url("/explore"))
        .query(&[("page", "999"), ("page_size", "2")])
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_page_res.status(), 200);

    let response_body = invalid_page_res.json::<serde_json::Value>().await.unwrap();

    assert_eq!(response_body["items"].as_array().unwrap().len(), 0);

    // ----------- Invalid request with non-existing category

    let invalid_category_res = http_client
        .get(backend_url("/explore"))
        .query(&[("page", "1"), ("page_size", "2")])
        .query(&[("category", "non-existing-category")])
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_category_res.status(), 200);

    let response_body = invalid_category_res
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(response_body["items"].as_array().unwrap().len(), 0);

    // ----------- Invalid request with non-existing order_by field

    let invalid_order_res = http_client
        .get(backend_url("/explore"))
        .query(&[
            ("page", "1"),
            ("page_size", "2"),
            ("order_by", "non_existing_field"),
        ])
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_order_res.status(), 400);

    // ----------- Invalid request with negative minimun price

    let invalid_min_price_res = http_client
        .get(backend_url("/explore"))
        .query(&[("page", "1"), ("page_size", "2"), ("min_price", "-1000")])
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_min_price_res.status(), 400);

    // ----------- Invalid request with minimum price greater than maximum price

    let invalid_price_range_res = http_client
        .get(backend_url("/explore"))
        .query(&[
            ("page", "1"),
            ("page_size", "2"),
            ("min_price", "100000"),
            ("max_price", "50000"),
        ])
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_price_range_res.status(), 400);
}

#[tokio::test]
async fn test_select_machine() {
    setup().await;
    let http_client = Client::new();

    // ----------- Select a machine with a valid ID

    let valid_machine_id = 1;
    let valid_response = http_client
        .get(format!("{}/{}", backend_url("/explore"), valid_machine_id))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response.status(), 200);

    let response_body = valid_response.json::<serde_json::Value>().await.unwrap();

    let machine = response_body["machine"].as_object().unwrap();

    assert_eq!(machine.get("id").unwrap(), 1);
    assert_eq!(machine.get("model").unwrap(), "CAT320D");
    assert_eq!(machine.get("brand").unwrap(), "Caterpillar");
    assert_eq!(
        machine.get("categories").unwrap().as_array().unwrap()[0]["name"],
        "construccion pesada"
    );

    // ----------- Select a machine with an invalid ID

    let invalid_machine_id = 9999;
    let invalid_response = http_client
        .get(format!(
            "{}/{}",
            backend_url("/explore"),
            invalid_machine_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_response.status(), 404);
}

#[tokio::test]
async fn get_machine_locations() {
    setup().await;
    let http_client = Client::new();

    // ----------- Get locations for a valid machine ID

    let jwt = get_test_jwt("hank@example.com", false).await;

    let valid_machine_id = 1;
    let valid_response = http_client
        .post(format!(
            "{}/{}/locations",
            backend_url("/explore"),
            valid_machine_id
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response.status(), 200);

    let response_body = valid_response.json::<serde_json::Value>().await.unwrap();

    assert!(response_body["locations"].is_array());
    assert!(!response_body["locations"].as_array().unwrap().is_empty());

    assert_eq!(
        response_body["locations"].as_array().unwrap()[0]["city"],
        "Buenos Aires"
    );

    // ----------- Get locations for an invalid machine ID

    let invalid_machine_id = 9999;
    let invalid_response = http_client
        .post(format!(
            "{}/{}/locations",
            backend_url("/explore"),
            invalid_machine_id
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_response.status(), 404);

    // ----------- Get locations with an invalid JWT

    let invalid_jwt = "invalid.jwt.token";

    let invalid_jwt_response = http_client
        .post(format!(
            "{}/{}/locations",
            backend_url("/explore"),
            valid_machine_id
        ))
        .json(&serde_json::json!({
            "access": invalid_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_jwt_response.status(), 401);

    // ----------- Get locations with a JWT of a non-client user

    let non_client_jwt = get_test_jwt("frank@example.com", false).await;

    let non_client_response = http_client
        .post(format!(
            "{}/{}/locations",
            backend_url("/explore"),
            valid_machine_id
        ))
        .json(&serde_json::json!({
            "access": non_client_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(non_client_response.status(), 403);
}

#[tokio::test]
async fn test_check_availability() {
    setup().await;
    let http_client = Client::new();

    let jwt = get_test_jwt("hank@example.com", false).await;

    // ----------- Check availability for a valid machine ID and date range

    let valid_machine_id = 4;
    let valid_location_id = 2;
    let valid_response = http_client
        .post(backend_url("/rental/availability"))
        .query(&[
            ("model_id", &valid_machine_id.to_string()),
            ("location_id", &valid_location_id.to_string()),
        ])
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response.status(), 200);

    let response_body = valid_response.json::<serde_json::Value>().await.unwrap();

    let dates = response_body["not_available_dates"].as_array().unwrap();

    assert_eq!(dates.len(), 2);

    // ----------- Check availability for an invalid machine ID

    let invalid_machine_id = 9999;
    let invalid_response = http_client
        .post(backend_url("/rental/availability"))
        .query(&[
            ("model_id", &invalid_machine_id.to_string()),
            ("location_id", &valid_location_id.to_string()),
        ])
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_response.status(), 200);

    let response_body = invalid_response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        response_body["not_available_dates"]
            .as_array()
            .unwrap()
            .len(),
        0
    );

    // ----------- Check availability with missing parameters

    let missing_params_response = http_client
        .post(backend_url("/rental/availability"))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(missing_params_response.status(), 400);
}

#[tokio::test]
async fn test_new_model() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Get an admin token
    let jwt = get_test_jwt("admin@example.com", true).await;

    // Read image file from disk
    let mut file = File::open("media/test/test_image1.png").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    // Base64 encode the image
    let img1 = STANDARD.encode(&buffer);

    let mut file = File::open("media/test/test_image2.png").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let img2 = STANDARD.encode(&buffer);


    let res = client
        .post(backend_url("/newmodel"))
        .json(&serde_json::json!({
            "access": jwt,
            "name": "Bulldozer X1",
            "brand": "Caterpillar",
            "model": "X1 2024",
            "year": 2024,
            "policy": "Basic Warranty",
            "description": "Powerful bulldozer for rough terrain",
            "price": 99000,
            "categories": ["Heavy", "Construction"],
            "images": [img1, img2]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Model created successfully");

    // Check that the model exists in the DB
    let row = db_client.query_one(
        "SELECT * FROM machinery_models WHERE name = $1",
        &[&"Bulldozer X1"],
    ).await.unwrap();

    let model_id: i32 = row.get("id");

    // Check that categories were linked
    let cats = db_client.query(
        "SELECT * FROM machinery_categories WHERE model_id = $1",
        &[&model_id],
    ).await.unwrap();
    assert_eq!(cats.len(), 2);

    // Check that the images are registered
    let imgs = db_client.query(
        "SELECT * FROM model_images WHERE id = $1",
        &[&model_id],
    ).await.unwrap();
    assert_eq!(imgs.len(), 2);

    //Try to send 12 images
    let res = client
        .post(backend_url("/newmodel"))
        .json(&serde_json::json!({
            "access": jwt,
            "name": "Bulldozer X1",
            "brand": "Caterpillar",
            "model": "X1 2024",
            "year": 2024,
            "policy": "Basic Warranty",
            "description": "Powerful bulldozer for rough terrain",
            "price": 99000,
            "categories": ["Heavy", "Construction"],
            "images": [img2, img2, img2, img2, img2, img2,
                        img2, img2, img2, img2, img2, img2]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Cannot upload more than 10 images");

    //Try an invalid jwt
    let res = client
        .post(backend_url("/newmodel"))
        .json(&serde_json::json!({
            "access": "hello",
            "name": "Bulldozer X1",
            "brand": "Caterpillar",
            "model": "X1 2024",
            "year": 2024,
            "policy": "Basic Warranty",
            "description": "Powerful bulldozer for rough terrain",
            "price": 99000,
            "categories": ["Heavy", "Construction"],
            "images": [img2]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Invalid access token");

    //Try to access as an user
    let jwt = get_test_jwt("login@example.com", false).await;
    let res = client
        .post(backend_url("/newmodel"))
        .json(&serde_json::json!({
            "access": jwt,
            "name": "Bulldozer X1",
            "brand": "Caterpillar",
            "model": "X1 2024",
            "year": 2024,
            "policy": "Basic Warranty",
            "description": "Powerful bulldozer for rough terrain",
            "price": 99000,
            "categories": ["Heavy", "Construction"],
            "images": [img1, img2]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"].as_str().unwrap(), "Not enough permissions");
}
