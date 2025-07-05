use crate::custom_types::structs::MachineModel;
use crate::custom_types::{enums::RunningEnv, structs::MyRentalInfo};
use crate::helpers::auth::create_pool;
use crate::tests::helpers::*;
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{NaiveDate, Utc};
use reqwest::Client;
use std::{env, fs::File, io::Read};
use validator::ValidateLength;

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

    assert_eq!(response_body["all_categories"].as_array().unwrap().len(), 5);

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
    assert!(!machine2
        .as_object()
        .unwrap()
        .get("main_image")
        .unwrap()
        .as_str()
        .unwrap()
        .is_empty(),);

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
    let machines_categories = ["obras urbanas", "movimiento de tierra"];

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

    assert!(machines_categories.contains(
        &machine9
            .as_object()
            .unwrap()
            .get("categories")
            .unwrap()
            .as_array()
            .unwrap()[0]["name"]
            .as_str()
            .unwrap()
    ));

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

    // ----------- Request with spaces in search term

    let spaces_search_res = http_client
        .get(backend_url("/explore"))
        .query(&[
            ("page", "1"),
            ("page_size", "2"),
            ("search", "      excavadora        "),
        ])
        .send()
        .await
        .unwrap();

    assert_eq!(spaces_search_res.status(), 200);

    let response_body = spaces_search_res.json::<serde_json::Value>().await.unwrap();

    assert!(response_body["items"].as_array().unwrap().len() > 0);

    // ----------- Request with empty search term

    let empty_search_res = http_client
        .get(backend_url("/explore"))
        .query(&[("page", "1"), ("page_size", "2"), ("search", " ")])
        .send()
        .await
        .unwrap();

    assert_eq!(empty_search_res.status(), 200);

    let response_body = empty_search_res.json::<serde_json::Value>().await.unwrap();

    assert!(response_body["items"].as_array().unwrap().len() > 0);

    // ----------- Request with special characters in search term

    let special_chars_search_res = http_client
        .get(backend_url("/explore"))
        .query(&[
            ("page", "1"),
            ("page_size", "2"),
            ("search", "exca!@va#$do%^&ra*() hidráuli*ca"),
        ])
        .send()
        .await
        .unwrap();

    assert_eq!(special_chars_search_res.status(), 200);

    let response_body = special_chars_search_res
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert!(response_body["items"].as_array().unwrap().len() > 0);
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
    assert_eq!(
        machine
            .get("extra_images")
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        2
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

    // ----------- Get locations with a JWT of an admin user

    let non_client_jwt = get_test_jwt("alice@example.com", false).await;

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
async fn test_get_units_unavailable_dates() {
    setup().await;
    let http_client = Client::new();

    let jwt = get_test_jwt("hank@example.com", false).await;

    // ----------- Check availability for a valid machine ID and date range

    let valid_model_id = 1;
    let valid_location_id = 1;
    let valid_response = http_client
        .post(backend_url("/rental/availability"))
        .query(&[
            ("model_id", &valid_model_id.to_string()),
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

    let units_info = response_body["units_and_their_unavailable_dates"]
        .as_array()
        .unwrap();

    assert_eq!(units_info.len(), 2);

    let dates_from_unit1 = &units_info[0]["periods"].as_array().unwrap();
    let dates_from_unit2 = &units_info[1]["periods"].as_array().unwrap();

    assert_eq!(dates_from_unit1.len(), 0);
    assert_eq!(dates_from_unit2.len(), 2);

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

    assert_eq!(invalid_response.status(), 404);

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
            "extra_images": [img1, img2],
            "image": img1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Model created successfully"
    );

    // Check that the model exists in the DB
    let row = db_client
        .query_one(
            "SELECT * FROM machinery_models WHERE name = $1",
            &[&"Bulldozer X1"],
        )
        .await
        .unwrap();

    let model_id: i32 = row.get("id");

    // Check that categories were linked
    let cats = db_client
        .query(
            "SELECT * FROM machinery_categories WHERE model_id = $1",
            &[&model_id],
        )
        .await
        .unwrap();
    assert_eq!(cats.len(), 2);

    // Check that the images are registered
    let imgs = db_client
        .query(
            "SELECT * FROM model_extra_images WHERE id = $1",
            &[&model_id],
        )
        .await
        .unwrap();
    assert_eq!(imgs.len(), 2);

    //Try to create a model with the same brand, model and year
    let res = client
        .post(backend_url("/newmodel"))
        .json(&serde_json::json!({
            "access": jwt,
            "name": "Bulldozer X1",
            "brand": "Caterpillar",
            "model": "X1 2024",
            "year": 2024,
            "policy": "New Policy",
            "description": "No",
            "price": 123123,
            "categories": ["Construction"],
            "extra_images": [img1, img2],
            "image": img1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "A model with this same name, brand, model and year already exists"
    );

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
            "extra_images": [img2, img2, img2, img2, img2, img2,
                        img2, img2, img2, img2, img2, img2],
            "image":img1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Cannot upload more than 10 images"
    );

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
            "extra_images": [img2],
            "image": img2
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Invalid access token"
    );

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
            "extra_images": [img1, img2],
            "image":img1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Not enough permissions"
    );
}

#[tokio::test]
async fn test_new_rental() {
    setup().await;
    let http_client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    let jwt = get_test_jwt("hank@example.com", false).await;

    // ----------- Create a new rental with valid data

    let start_date = Utc::now()
        .checked_add_signed(chrono::Duration::days(5))
        .unwrap()
        .date_naive();
    let end_date = start_date
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap();

    let new_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": start_date,
        "end_date": end_date,
        "total_price": 1_050_000.0,
        "access": jwt
    });

    let valid_response = http_client
        .post(backend_url("/rental/new"))
        .json(&new_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response.status(), 201);

    let response_body = valid_response.json::<serde_json::Value>().await.unwrap();

    let rental_id = response_body["rental_id"].as_i64().unwrap() as i32;

    match db_client
        .query_one("SELECT * FROM rentals r WHERE r.id = $1;", &[&rental_id])
        .await
    {
        Ok(row) => {
            assert_eq!(row.get::<_, i32>("machine_id"), 1);
            assert_eq!(row.get::<_, i32>("user_id"), 8);
            assert_eq!(row.get::<_, f32>("total_price"), 1_050_000.0);
            assert_eq!(row.get::<_, chrono::NaiveDate>("start_date"), start_date);
            assert_eq!(row.get::<_, chrono::NaiveDate>("end_date"), end_date);
        }
        Err(e) => {
            panic!("Failed to query the database: {}", e);
        }
    }

    // ----------- Create a new rental with valid data but different user and machine

    let new_user_jwt = get_test_jwt("ivy@example.com", false).await;

    let new_rental_user = serde_json::json!({
        "machine_id": 4,
        "start_date": start_date,
        "end_date": end_date,
        "total_price": 665_000.0,
        "access": new_user_jwt
    });

    let new_user_response = http_client
        .post(backend_url("/rental/new"))
        .json(&new_rental_user)
        .send()
        .await
        .unwrap();

    assert_eq!(new_user_response.status(), 201);

    let new_user_response_body = new_user_response.json::<serde_json::Value>().await.unwrap();

    let new_user_rental_id = new_user_response_body["rental_id"].as_i64().unwrap() as i32;

    match db_client
        .query_one(
            "SELECT * FROM rentals r WHERE r.id = $1;",
            &[&new_user_rental_id],
        )
        .await
    {
        Ok(row) => {
            assert_eq!(row.get::<_, i32>("machine_id"), 4);
            assert_eq!(row.get::<_, i32>("user_id"), 9);
            assert_eq!(row.get::<_, f32>("total_price"), 665_000.0);
            assert_eq!(row.get::<_, chrono::NaiveDate>("start_date"), start_date);
            assert_eq!(row.get::<_, chrono::NaiveDate>("end_date"), end_date);
        }
        Err(e) => {
            panic!("Failed to query the database: {}", e);
        }
    }

    // ----------- Create a new rental with missing parameters

    let missing_params_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": start_date,
        "end_date": end_date,
        // "total_price" is missing
        "access": jwt
    });

    let missing_params_response = http_client
        .post(backend_url("/rental/new"))
        .json(&missing_params_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(missing_params_response.status(), 422);

    // ----------- Create a new rental with invalid data (end date before start date)

    let invalid_end_date = start_date
        .checked_sub_signed(chrono::Duration::days(1))
        .unwrap();

    let invalid_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": start_date,
        "end_date": invalid_end_date,
        "total_price": 1_050_000.0,
        "access": jwt
    });

    let invalid_response = http_client
        .post(backend_url("/rental/new"))
        .json(&invalid_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_response.status(), 400);

    // ----------- Create a new rental with period minor than 7 days

    let short_rental_end_date = start_date
        .checked_add_signed(chrono::Duration::days(6))
        .unwrap();

    let short_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": start_date,
        "end_date": short_rental_end_date,
        "total_price": 1_050_000.0,
        "access": jwt
    });

    let short_rental_response = http_client
        .post(backend_url("/rental/new"))
        .json(&short_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(short_rental_response.status(), 400);

    // ----------- Create a new rental with an invalid JWT

    let invalid_jwt = "invalid.jwt.token";

    let invalid_jwt_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": start_date,
        "end_date": end_date,
        "total_price": 1_050_000.0,
        "access": invalid_jwt
    });

    let invalid_jwt_response = http_client
        .post(backend_url("/rental/new"))
        .json(&invalid_jwt_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_jwt_response.status(), 401);

    // ----------- Create a new rental with a JWT of a non-client user

    let non_client_jwt = get_test_jwt("bob@example.com", false).await;

    let non_client_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": start_date,
        "end_date": end_date,
        "total_price": 1_050_000.0,
        "access": non_client_jwt
    });

    let non_client_response = http_client
        .post(backend_url("/rental/new"))
        .json(&non_client_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(non_client_response.status(), 403);

    // ----------- Create a new rental with a machine ID that does not exist

    let non_existing_machine_id = 9999;

    let non_existing_machine_rental = serde_json::json!({
        "machine_id": non_existing_machine_id,
        "start_date": start_date,
        "end_date": end_date,
        "total_price": 1_050_000.0,
        "access": jwt
    });

    let non_existing_machine_response = http_client
        .post(backend_url("/rental/new"))
        .json(&non_existing_machine_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(non_existing_machine_response.status(), 404);

    // ----------- Create a new rental with negative total price

    let negative_price_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": start_date,
        "end_date": end_date,
        "total_price": -1000.0,
        "access": jwt
    });

    let negative_price_response = http_client
        .post(backend_url("/rental/new"))
        .json(&negative_price_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(negative_price_response.status(), 400);

    // ----------- Create a new rental with a total price that does not match the expected price

    let another_start_date = start_date
        .checked_add_signed(chrono::Duration::days(20))
        .unwrap();

    let another_end_date = another_start_date
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap();

    let wrong_price_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": another_start_date,
        "end_date": another_end_date,
        "total_price": 500_000.0, // This should be 1_050_000.0 based on the rental period
        "access": jwt
    });

    let wrong_price_response = http_client
        .post(backend_url("/rental/new"))
        .json(&wrong_price_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(wrong_price_response.status(), 400);

    // ----------- Create a new rental with a machine that is already rented for the requested period

    let overlapping_start_date = start_date
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap();
    let overlapping_end_date = end_date
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap();

    let overlapping_rental = serde_json::json!({
        "machine_id": 1,
        "start_date": overlapping_start_date,
        "end_date": overlapping_end_date,
        "total_price": 1_050_000.0,
        "access": jwt
    });

    let overlapping_response = http_client
        .post(backend_url("/rental/new"))
        .json(&overlapping_rental)
        .send()
        .await
        .unwrap();

    assert_eq!(overlapping_response.status(), 409);
}

#[tokio::test]
async fn test_check_rental_payment() {
    setup().await;
    let http_client = Client::new();

    let jwt = get_test_jwt("hank@example.com", false).await;

    // ----------- Check payment for a valid rental ID

    let valid_rental_id = 7;
    let valid_response = http_client
        .post(backend_url("/payment/check"))
        .query(&[("payment_id", "2424235352"), ("status", "approved")])
        .json(&serde_json::json!({
            "rental_id": valid_rental_id,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response.status(), 200);

    let response_body = valid_response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        response_body["message"].as_str().unwrap(),
        "El alquiler ha sido aprobado y el usuario ha sido notificado"
    );

    // ---------- Check payment for an invalid rental ID

    let invalid_rental_id = 9999;

    let invalid_response = http_client
        .post(backend_url("/payment/check"))
        .query(&[("payment_id", "2424235352"), ("status", "approved")])
        .json(&serde_json::json!({
            "rental_id": invalid_rental_id,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_response.status(), 404);

    // ----------- Check payment with failed status

    let failed_rental_id = 8;

    let failed_response = http_client
        .post(backend_url("/payment/check"))
        .query(&[("payment_id", "2424235352"), ("status", "rejected")])
        .json(&serde_json::json!({
            "rental_id": failed_rental_id,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(failed_response.status(), 502);

    // ----------- Check payment with another status

    let another_status_rental_id = 9;
    let another_status_response = http_client
        .post(backend_url("/payment/check"))
        .query(&[("payment_id", "2424235352"), ("status", "pending")])
        .json(&serde_json::json!({
            "rental_id": another_status_rental_id,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(another_status_response.status(), 409);
}

#[tokio::test]
async fn test_new_unit() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Get an admin token
    let jwt = get_test_jwt("admin@example.com", true).await;

    let res = client
        .post(backend_url("/newunit"))
        .json(&serde_json::json!({
            "access": jwt,
            "serial_number": "AAAA1234",
            "model_id": 1,
            "location_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Unit created successfully"
    );

    // Check that the model exists in the DB
    db_client
        .query_one(
            "SELECT * FROM machinery_units WHERE serial_number = $1 AND model_id = $2 AND location_id = $3",
            &[&"AAAA1234", &&1, &&1],
        )
        .await
        .unwrap();

    //Used serial_number
    let res = client
        .post(backend_url("/newunit"))
        .json(&serde_json::json!({
            "access": jwt,
            "serial_number": "AAAA1234",
            "model_id": 1,
            "location_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "The serial_number is already registered"
    );

    //Invalid model_id
    let res = client
        .post(backend_url("/newunit"))
        .json(&serde_json::json!({
            "access": jwt,
            "serial_number": "NEWSERIAL",
            "model_id": 10000,
            "location_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "model_id is invalid"
    );

    //Invalid location_id
    let res = client
        .post(backend_url("/newunit"))
        .json(&serde_json::json!({
            "access": jwt,
            "serial_number": "NEWSERIAL",
            "model_id": 1,
            "location_id": 10000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "location_id is invalid"
    );

    //Invalid token
    let res = client
        .post(backend_url("/newunit"))
        .json(&serde_json::json!({
            "access": "thisisnotavalidtoken",
            "serial_number": "AAAA1234",
            "model_id": 1,
            "location_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Invalid access token"
    );

    //Try to access as an user
    let jwt = get_test_jwt("login@example.com", false).await;
    let res = client
        .post(backend_url("/newunit"))
        .json(&serde_json::json!({
            "access": jwt,
            "serial_number": "AAAA1234",
            "model_id": 1,
            "location_id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Not enough permissions"
    );
}

#[tokio::test]
async fn test_get_my_rentals() {
    setup().await;
    let client = Client::new();

    //Get the access token (id 8)
    let jwt = get_test_jwt("hank@example.com", false).await;
    //get_employees
    let res = client
        .post(backend_url("/myrentals"))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["rentals"].clone();
    let rentals: Vec<MyRentalInfo> = serde_json::from_value(value).unwrap();

    assert_eq!(rentals.len(), 1);
    let rental = &rentals[0];

    assert_eq!(rental.rental_id, 2);
    assert_eq!(
        rental.return_date,
        Some(NaiveDate::from_ymd_opt(2025, 1, 12).unwrap())
    );
    assert_eq!(
        rental.retirement_date,
        Some(NaiveDate::from_ymd_opt(2025, 1, 3).unwrap())
    );
    assert_eq!(
        rental.start_date,
        NaiveDate::from_ymd_opt(2025, 1, 3).unwrap()
    );
    assert_eq!(
        rental.end_date,
        NaiveDate::from_ymd_opt(2025, 1, 12).unwrap()
    );
    assert_eq!(rental.total_price, 1000.0);
    assert_eq!(rental.status, "completed");
    assert_eq!(rental.unit_id, 5);
    assert_eq!(rental.unit_serial_number, "JD-002");
    assert_eq!(rental.model_id, 2);
    assert_eq!(rental.model_name, "Retroexcavadora");
    assert_eq!(rental.model_brand, "John Deere");
    assert_eq!(rental.model_model, "310SL");
    assert_eq!(rental.model_year, 2019);
    assert_eq!(
        rental.model_policy,
        "No se realizan reembolsos por cancelaciones."
    );
    assert_eq!(rental.model_description, "Ideal para zonas urbanas");
    let nginx_url = env::var("NGINX_URL").expect("NGINX_URL must be set in the .env file");
    assert_eq!(
        rental.model_image,
        format!("{}/media/machines/imagecode.webp", nginx_url)
    );

    let jwt = get_test_jwt("admin@example.com", false).await;
    let res = client
        .post(backend_url("/myrentals"))
        .json(&serde_json::json!({
            "access": jwt,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "The user is not a client"
    );

    //Invalid token
    let res = client
        .post(backend_url("/myrentals"))
        .json(&serde_json::json!({
            "access": "thisisnotavalidtoken"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Invalid access token"
    );
}

#[tokio::test]
async fn test_load_retirement() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Get an admin token
    let jwt = get_test_jwt("admin@example.com", true).await;

    let res = client
        .post(backend_url("/loadretirement"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 18,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Retirement loaded successfully"
    );

    // Check that the rental was updated
    db_client
        .query_one(
            "SELECT * FROM rentals WHERE id = $1 AND
            machine_id = $2 AND
            retirement_employee_id = $3 AND
            retirement_date = CURRENT_DATE;",
            &[&&18, &&2, &&11],
        )
        .await
        .unwrap();
    // Check that the unit was updated
    db_client
        .query_one(
            "SELECT * FROM machinery_units WHERE id = $1 AND status = 'rented';",
            &[&&2],
        )
        .await
        .unwrap();

    //Invalid rental_id
    let res = client
        .post(backend_url("/loadretirement"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 10000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "rental_id is invalid"
    );

    //Rental expired
    let res = client
        .post(backend_url("/loadretirement"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 20
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "The rental has expired"
    );

    //Rental not active
    let res = client
        .post(backend_url("/loadretirement"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 19
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "The rental is not active"
    );

    //Invalid token
    let res = client
        .post(backend_url("/loadretirement"))
        .json(&serde_json::json!({
            "access": "invalidtoken",
            "rental_id": 10000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Invalid access token"
    );

    //Try to access as an user
    let jwt = get_test_jwt("login@example.com", false).await;
    let res = client
        .post(backend_url("/loadretirement"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 10000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Not enough permissions"
    );
}

#[tokio::test]
async fn test_load_return() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Get an admin token
    let jwt = get_test_jwt("admin@example.com", true).await;

    let res = client
        .post(backend_url("/loadreturn"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 3,
            "location_id": 3,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    let json = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(
        json["message"].as_str().unwrap(),
        "Return loaded successfully"
    );
    assert!(json["days_late"].as_u64().unwrap() > 0);
    assert!(json["fine"].as_f64().unwrap() > 0.0);

    // Check that the rental was updated
    db_client
        .query_one(
            "SELECT * FROM rentals WHERE id = $1 AND
            machine_id = $2 AND
            return_employee_id = $3 AND
            status = 'completed' AND
            return_date = CURRENT_DATE;",
            &[&&3, &&16, &&11],
        )
        .await
        .unwrap();
    // Check that the unit was updated
    db_client
        .query_one(
            "SELECT * FROM machinery_units WHERE
            id = $1 AND
            status = 'maintenance' AND
            location_id = $2;",
            &[&&16, &&3],
        )
        .await
        .unwrap();
    // Check that the location history was updated
    db_client
        .query_one(
            "SELECT * FROM machinery_location_history WHERE
            unit_id = $1 AND
            location_id = $2;",
            &[&&16, &&1],
        )
        .await
        .unwrap();

    //Invalid rental_id
    let res = client
        .post(backend_url("/loadreturn"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 10000,
            "location_id": 3
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "rental_id is invalid"
    );

    //Invalid location_id
    let res = client
        .post(backend_url("/loadreturn"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 4,
            "location_id": 10000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "location_id is invalid"
    );

    //Invalid token
    let res = client
        .post(backend_url("/loadreturn"))
        .json(&serde_json::json!({
            "access": "invalidtoken",
            "rental_id": 4,
            "location_id": 10000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Invalid access token"
    );

    //Try to access as an user
    let jwt = get_test_jwt("login@example.com", false).await;
    let res = client
        .post(backend_url("/loadreturn"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 4,
            "location_id": 10000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(
        res.json::<serde_json::Value>().await.unwrap()["message"]
            .as_str()
            .unwrap(),
        "Not enough permissions"
    );
}

#[tokio::test]
async fn test_cancel_rental() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    // Get an client token
    let jwt = get_test_jwt("ivy@example.com", false).await;

    // ----------- Client user cancels a rental with valid data

    let rental_id = 15;

    let cancel_response = client
        .post(backend_url("/rental/cancel"))
        .json(&serde_json::json!({
            "rental_id": rental_id,
            "access": jwt,
            "reason": null,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(cancel_response.status(), 200);

    let response_body = cancel_response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        response_body["message"].as_str().unwrap(),
        "El alquiler ha sido cancelado exitosamente"
    );

    let row = db_client
        .query_one(
            "SELECT id, status::TEXT as status FROM rentals WHERE id = $1 AND status = 'cancelled';",
            &[&&rental_id],
        )
        .await
        .unwrap();

    assert_eq!(row.get::<_, i32>("id"), rental_id);
    assert_eq!(row.get::<_, String>("status"), "cancelled");

    // ---------- Client user tries to cancel a rental that does not exist

    let invalid_rental_id = 9999;
    let invalid_cancel_response = client
        .post(backend_url("/rental/cancel"))
        .json(&serde_json::json!({
            "rental_id": invalid_rental_id,
            "access": jwt,
            "reason": null,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_cancel_response.status(), 404);

    let invalid_response_body = invalid_cancel_response
        .json::<serde_json::Value>()
        .await
        .unwrap();
    assert_eq!(
        invalid_response_body["message"].as_str().unwrap(),
        "El alquiler no se ha encontrado o ya ha sido cancelado"
    );

    // ---------- Client user tries to cancel a rental that has already been retired

    let past_rental_id = 22;

    let past_cancel_response = client
        .post(backend_url("/rental/cancel"))
        .json(&serde_json::json!({
            "rental_id": past_rental_id,
            "access": jwt,
            "reason": null,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(past_cancel_response.status(), 400);

    let past_response_body = past_cancel_response
        .json::<serde_json::Value>()
        .await
        .unwrap();
    assert_eq!(
        past_response_body["message"].as_str().unwrap(),
        "No se puede cancelar un alquiler que ya ha sido retirado"
    );

    // ---------- Client user tries to cancel a rental that he has not rented

    let another_user_rental_id = 6;
    let another_user_cancel_response = client
        .post(backend_url("/rental/cancel"))
        .json(&serde_json::json!({
            "rental_id": another_user_rental_id,
            "access": jwt,
            "reason": null,
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(another_user_cancel_response.status(), 404);

    let another_user_response_body = another_user_cancel_response
        .json::<serde_json::Value>()
        .await
        .unwrap();
    assert_eq!(
        another_user_response_body["message"].as_str().unwrap(),
        "El alquiler no se ha encontrado o no puede ser cancelado"
    );

    // ---------- Client user tries to cancel a rental with start date in the past

    let past_start_date_rental_id = 18;

    let past_start_date_cancel_response = client
        .post(backend_url("/rental/cancel"))
        .json(&serde_json::json!({
            "rental_id": past_start_date_rental_id,
            "access": jwt,
            "reason": null,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(past_start_date_cancel_response.status(), 400);

    let past_start_date_response_body = past_start_date_cancel_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        past_start_date_response_body["message"].as_str().unwrap(),
        "No se puede cancelar un alquiler que ya ha comenzado y no ha finalizado"
    );

    // ---------- Employee cancels a rental with valid data

    let employee_jwt = get_test_jwt("bob@example.com", true).await;

    let another_rental_id = 16;

    let employee_cancel_response = client
        .post(backend_url("/rental/cancel"))
        .json(&serde_json::json!({
            "rental_id": another_rental_id,
            "access": employee_jwt,
            "reason": "Maintenance required",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(employee_cancel_response.status(), 200);

    let employee_response_body = employee_cancel_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        employee_response_body["message"].as_str().unwrap(),
        "El alquiler ha sido cancelado exitosamente y el cliente ha sido notificado"
    );

    let employee_row = db_client
        .query_one(
            "SELECT id, status::TEXT as status, notes FROM rentals WHERE id = $1 AND status = 'cancelled';",
            &[&&another_rental_id],
        )
        .await
        .unwrap();

    assert_eq!(employee_row.get::<_, i32>("id"), another_rental_id);
    assert_eq!(employee_row.get::<_, String>("status"), "cancelled");
    assert_eq!(
        employee_row.get::<_, Option<String>>("notes"),
        Some("Maintenance required".to_string())
    );
}

#[tokio::test]
async fn test_get_staff_rentals() {
    setup().await;
    let client = Client::new();

    let jwt = get_test_jwt("bob@example.com", true).await;

    // ---------- Employee retrieves all rentals

    let res = client
        .post(backend_url("/staff/rentals"))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap();

    assert!(value["rentals"].as_array().unwrap().len() > 1);

    // ---------- Employee retrieves specific rental by ID

    let rental_id = 1;

    let res = client
        .post(backend_url("/staff/rentals"))
        .json(&serde_json::json!({
            "access": jwt,
        }))
        .query(&[("id", "1")])
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    let rental = res.json::<serde_json::Value>().await.unwrap();

    let machine = rental["rentals"].as_array().unwrap().get(0);

    assert_eq!(rental["rentals"].as_array().unwrap().len(), 1);
    assert_eq!(machine.unwrap()["rental_id"], rental_id);

    assert!(machine.unwrap()["days_late"].is_null());
    assert!(machine.unwrap()["percentage_per_day_late"].is_null());

    // ---------- Employee retrieves rentals with invalid ID

    let res = client
        .post(backend_url("/staff/rentals"))
        .json(&serde_json::json!({
            "access": jwt,
        }))
        .query(&[("id", "9999")])
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 404);

    let res_json = res.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        res_json["message"].as_str().unwrap(),
        "No se encontraron alquileres"
    );

    // ---------- Not staff member tries to access rentals

    let user_jwt = get_test_jwt("hank@example.com", false).await;

    let res = client
        .post(backend_url("/staff/rentals"))
        .json(&serde_json::json!({
            "access": user_jwt,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);

    let res_json = res.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        res_json["message"].as_str().unwrap(),
        "Solo empleados y administradores pueden acceder a esta información"
    );

    // ---------- Retrieves late rental

    let late_rental_id = 17;

    let res = client
        .post(backend_url("/staff/rentals"))
        .json(&serde_json::json!({
            "access": jwt,
        }))
        .query(&[("id", &late_rental_id.to_string())])
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    let late_rental = res.json::<serde_json::Value>().await.unwrap();

    let late_machine = late_rental["rentals"].as_array().unwrap().get(0);

    assert_eq!(late_machine.unwrap()["days_late"].as_u64().unwrap(), 3);
    assert!(!late_machine.unwrap()["percentage_per_late_day"].is_null());
}

#[tokio::test]
async fn test_get_locations() {
    setup().await;
    let client = Client::new();

    // Get an admin token
    let jwt = get_test_jwt("alice@example.com", true).await;

    // ----------- Admin retrieves all locations

    let res = client
        .post(backend_url("/locations"))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap();

    assert!(value["locations"].as_array().unwrap().len() > 0);

    // ----------- Client user tries to retrieve locations

    let user_jwt = get_test_jwt("dave@example.com", false).await;

    let res = client
        .post(backend_url("/locations"))
        .json(&serde_json::json!({
            "access": user_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);

    let res_json = res.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        res_json["message"].as_str().unwrap(),
        "Solo empleados y administradores pueden acceder a esta información"
    );
}

#[tokio::test]
async fn test_get_models() {
    setup().await;
    let client = Client::new();

    //Get the access token needed for get_employees
    let jwt = get_test_jwt("admin2@example.com", false).await;
    //get_employees
    let res = client
        .post(backend_url("/getmodels"))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["models"].clone();
    let models: Vec<MachineModel> = serde_json::from_value(value).unwrap();
    assert!(models.len() > 5);
}

#[tokio::test]
async fn test_verify_client() {
    setup().await;
    let client = Client::new();

    // Get an employee token
    let jwt = get_test_jwt("bob@example.com", true).await;

    // ----------- Employee verifies a valid client

    let valid_client_email = "dave@example.com";

    let valid_client_response = client
        .post(backend_url("/staff/rental/verifyclient"))
        .json(&serde_json::json!({
            "email": valid_client_email,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_client_response.status(), 200);

    let valid_client_body = valid_client_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(valid_client_body["user_id"].as_i64().unwrap(), 4);

    // ----------- Employee tries to verify an unregistered email

    let invalid_client_email = "notaregistereduser@example.com";

    let invalid_client_response = client
        .post(backend_url("/staff/rental/verifyclient"))
        .json(&serde_json::json!({
            "email": invalid_client_email,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_client_response.status(), 404);

    let invalid_client_body = invalid_client_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        invalid_client_body["message"].as_str().unwrap(),
        "El email no corresponde a un cliente registrado"
    );

    // ----------- Employee tries to verify a user that is not a client

    let non_client_email = "alice@example.com";

    let non_client_response = client
        .post(backend_url("/staff/rental/verifyclient"))
        .json(&serde_json::json!({
            "email": non_client_email,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(non_client_response.status(), 404);

    let non_client_body = non_client_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        non_client_body["message"].as_str().unwrap(),
        "El email no corresponde a un cliente registrado"
    );

    // ----------- Client tries to use the verify endpoint

    let user_jwt = get_test_jwt("dave@example.com", false).await;

    let user_response = client
        .post(backend_url("/staff/rental/verifyclient"))
        .json(&serde_json::json!({
            "email": valid_client_email,
            "access": user_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(user_response.status(), 403);

    let user_body = user_response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        user_body["message"].as_str().unwrap(),
        "Solo empleados pueden acceder a esta funcionalidad"
    );
}

#[tokio::test]
async fn test_get_units_by_model_and_location() {
    setup().await;
    let client = Client::new();

    // Get an employee token
    let jwt = get_test_jwt("bob@example.com", true).await;

    // ----------- Employee retrieves units by valid location ID

    let valid_model_id = 1;
    let valid_location_id = 1;

    let valid_units_response = client
        .post(backend_url("/staff/rental/getunits"))
        .json(&serde_json::json!({
            "model_id": valid_model_id,
            "location_id": valid_location_id,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_units_response.status(), 200);

    let valid_units_body = valid_units_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let units = valid_units_body["units_id"].as_array().unwrap();

    assert!(!units.is_empty());

    // ----------- Employee tries to retrieve units not availables for that model and location

    let invalid_model_id = 1;
    let invalid_location_id = 3;

    let invalid_units_response = client
        .post(backend_url("/staff/rental/getunits"))
        .json(&serde_json::json!({
            "model_id": invalid_model_id,
            "location_id": invalid_location_id,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_units_response.status(), 404);

    let invalid_units_body = invalid_units_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        invalid_units_body["message"].as_str().unwrap(),
        "No se encontraron unidades disponibles en la ubicación especificada"
    );

    // ----------- Employee tries to retrieve units with invalid model ID and location ID

    let invalid_model_id = 9999;
    let invalid_location_id = 9999;

    let invalid_request_response = client
        .post(backend_url("/staff/rental/getunits"))
        .json(&serde_json::json!({
            "model_id": invalid_model_id,
            "location_id": invalid_location_id,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_request_response.status(), 404);

    // ---------- Not employee tries to access units

    let user_jwt = get_test_jwt("dave@example.com", false).await;

    let user_response = client
        .post(backend_url("/staff/rental/getunits"))
        .json(&serde_json::json!({
            "model_id": valid_model_id,
            "location_id": valid_location_id,
            "access": user_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(user_response.status(), 403);

    let user_body = user_response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(
        user_body["message"].as_str().unwrap(),
        "Solo empleados pueden acceder a esta funcionalidad"
    );
}

#[tokio::test]
async fn test_validate_rental_dates() {
    setup().await;
    let client = Client::new();

    // Get an employee token
    let jwt = get_test_jwt("bob@example.com", true).await;

    // ----------- Employee validates valid rental dates

    let valid_unit_id = 18;
    let valid_start_date = Utc::now()
        .checked_add_signed(chrono::Duration::days(20))
        .unwrap()
        .date_naive();
    let valid_end_date = valid_start_date
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap();

    let valid_dates_response = client
        .post(backend_url("/staff/rental/validatedates"))
        .json(&serde_json::json!({
            "unit_id": valid_unit_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_dates_response.status(), 200);

    let valid_dates_body = valid_dates_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        valid_dates_body["message"].as_str().unwrap(),
        "Las fechas de alquiler son válidas"
    );

    // ----------- Employee tries to validate rental dates with end date before start date

    let invalid_start_date = Utc::now()
        .checked_add_signed(chrono::Duration::days(20))
        .unwrap()
        .date_naive();

    let invalid_end_date = invalid_start_date
        .checked_sub_signed(chrono::Duration::days(7))
        .unwrap();

    let invalid_dates_response = client
        .post(backend_url("/staff/rental/validatedates"))
        .json(&serde_json::json!({
            "unit_id": valid_unit_id,
            "start_date": invalid_start_date,
            "end_date": invalid_end_date,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_dates_response.status(), 400);

    let invalid_dates_body = invalid_dates_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        invalid_dates_body["message"].as_str().unwrap(),
        "El período indicado no es válido. Debe ser al menos 7 días y la fecha de fin no puede ser anterior a la de inicio.");

    // ----------- Employee tries to validate rental dates with period lower than 7 days

    let short_start_date = Utc::now()
        .checked_add_signed(chrono::Duration::days(20))
        .unwrap()
        .date_naive();
    let short_end_date = short_start_date
        .checked_add_signed(chrono::Duration::days(6))
        .unwrap();

    let short_dates_response = client
        .post(backend_url("/staff/rental/validatedates"))
        .json(&serde_json::json!({
            "unit_id": valid_unit_id,
            "start_date": short_start_date,
            "end_date": short_end_date,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(short_dates_response.status(), 400);

    let short_dates_body = short_dates_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
            short_dates_body["message"].as_str().unwrap(),
            "El período indicado no es válido. Debe ser al menos 7 días y la fecha de fin no puede ser anterior a la de inicio.");

    // ----------- Employee tries to validate rental dates where start date overlaps with an existing rental

    let overlapping_start_date = Utc::now()
        .checked_add_signed(chrono::Duration::days(18))
        .unwrap()
        .date_naive();

    let overlapping_end_date = overlapping_start_date
        .checked_add_signed(chrono::Duration::days(10))
        .unwrap();

    let overlapping_dates_response = client
        .post(backend_url("/staff/rental/validatedates"))
        .json(&serde_json::json!({
            "unit_id": valid_unit_id,
            "start_date": overlapping_start_date,
            "end_date": overlapping_end_date,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(overlapping_dates_response.status(), 409);

    let overlapping_dates_body = overlapping_dates_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        overlapping_dates_body["message"].as_str().unwrap(),
        "Las fechas de inicio y fin se superponen con un alquiler existente, considerando el período de mantenimiento planificado",
    );
    assert!(overlapping_dates_body["overlaped_date"]
        .as_object()
        .is_some());

    // ---------- Employee tries to validate rental dates where end date overlaps with an existing rental

    let another_overlapping_start_date = Utc::now().date_naive();
    let another_overlapping_end_date = another_overlapping_start_date
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap();

    let another_overlapping_end_dates_response = client
        .post(backend_url("/staff/rental/validatedates"))
        .json(&serde_json::json!({
            "unit_id": valid_unit_id,
            "start_date": another_overlapping_start_date,
            "end_date": another_overlapping_end_date,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(another_overlapping_end_dates_response.status(), 409);

    let another_overlapping_end_dates_body = another_overlapping_end_dates_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        another_overlapping_end_dates_body["message"].as_str().unwrap(),
        "Las fechas de inicio y fin se superponen con un alquiler existente, considerando el período de mantenimiento planificado",
    );

    // ---------- Employee tries to validate rental dates with an invalid unit ID

    let invalid_unit_id = 9999;

    let invalid_unit_dates_response = client
        .post(backend_url("/staff/rental/validatedates"))
        .json(&serde_json::json!({
            "unit_id": invalid_unit_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_unit_dates_response.status(), 200);
}

#[tokio::test]
async fn test_new_in_person_rental() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    let jwt = get_test_jwt("frank@example.com", true).await;

    // ----------- Employee creates a new in-person rental with valid data

    let valid_machine_id = 4;
    let valid_user_id = 4;

    let valid_start_date = Utc::now()
        .checked_add_signed(chrono::Duration::days(40))
        .unwrap()
        .date_naive();
    let valid_end_date = valid_start_date
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap();

    let new_rental_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": valid_machine_id,
            "user_id": valid_user_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "total_price": 665_000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(new_rental_response.status(), 201);

    let new_rental_body = new_rental_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        new_rental_body["message"].as_str().unwrap(),
        "El alquiler ha sido registrado exitosamente y se le ha notificado al cliente"
    );

    let new_rental_query = "
    SELECT * 
    FROM rentals 
    WHERE user_id = $1 AND machine_id = $2 
        AND start_date = $3 AND status = 'active';
    ";

    match db_client
        .query_one(
            new_rental_query,
            &[&valid_user_id, &valid_machine_id, &valid_start_date],
        )
        .await
    {
        Ok(row) => {
            assert_eq!(row.get::<_, i32>("machine_id"), valid_machine_id);
            assert_eq!(row.get::<_, i32>("user_id"), valid_user_id);
            assert_eq!(row.get::<_, f32>("total_price"), 665_000.0);
            assert_eq!(
                row.get::<_, chrono::NaiveDate>("start_date"),
                valid_start_date
            );
            assert_eq!(row.get::<_, chrono::NaiveDate>("end_date"), valid_end_date);
        }
        Err(e) => {
            panic!("Failed to query the database: {}", e);
        }
    }

    // ---------- Not employee tries to create a new in-person rental

    let user_jwt = get_test_jwt("dave@example.com", false).await;

    let another_valid_machine_id = 5;

    let user_rental_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": another_valid_machine_id,
            "user_id": valid_user_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "total_price": 665_000,
            "access": user_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(user_rental_response.status(), 403);

    let user_rental_body = user_rental_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        user_rental_body["message"].as_str().unwrap(),
        "Solo empleados pueden acceder a esta funcionalidad"
    );

    // ---------- Total price below zero

    let invalid_price_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": another_valid_machine_id,
            "user_id": valid_user_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "total_price": -1000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_price_response.status(), 400);

    let invalid_price_body = invalid_price_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        invalid_price_body["message"].as_str().unwrap(),
        "Ingreso de información inválida"
    );

    // ---------- End date before start date

    let invalid_end_date = valid_start_date
        .checked_sub_signed(chrono::Duration::days(1))
        .unwrap();

    let invalid_dates_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": another_valid_machine_id,
            "user_id": valid_user_id,
            "start_date": valid_start_date,
            "end_date": invalid_end_date,
            "total_price": 665_000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_dates_response.status(), 400);

    let invalid_dates_body = invalid_dates_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        invalid_dates_body["message"].as_str().unwrap(),
        "El período indicado no es válido. Debe ser al menos 7 días y la fecha de fin no puede ser anterior a la de inicio."
    );

    // ---------- Rental period is less than 7 days

    let short_end_date = valid_start_date
        .checked_add_signed(chrono::Duration::days(6))
        .unwrap();

    let short_period_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": another_valid_machine_id,
            "user_id": valid_user_id,
            "start_date": valid_start_date,
            "end_date": short_end_date,
            "total_price": 665_000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(short_period_response.status(), 400);

    let short_period_body = short_period_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        short_period_body["message"].as_str().unwrap(),
        "El período indicado no es válido. Debe ser al menos 7 días y la fecha de fin no puede ser anterior a la de inicio."
    );

    // ---------- Rental overlaps with an existing rental

    let overlapping_start_date = Utc::now()
        .checked_add_signed(chrono::Duration::days(53))
        .unwrap()
        .date_naive();

    let overlapping_end_date = overlapping_start_date
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap();

    let overlapping_rental_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": valid_machine_id,
            "user_id": valid_user_id,
            "start_date": overlapping_start_date,
            "end_date": overlapping_end_date,
            "total_price": 665_000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(overlapping_rental_response.status(), 409);

    let overlapping_rental_body = overlapping_rental_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        overlapping_rental_body["message"].as_str().unwrap(),
        "Las fechas de inicio y fin se superponen con un alquiler existente, considerando el período de mantenimiento planificado"
    );

    // ---------- Rental with an invalid machine ID

    let invalid_machine_id = 9999;

    let invalid_machine_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": invalid_machine_id,
            "user_id": valid_user_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "total_price": 665_000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_machine_response.status(), 404);

    let invalid_machine_body = invalid_machine_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        invalid_machine_body["message"].as_str().unwrap(),
        "No se ha encontrado la máquina solicitada"
    );

    // ---------- Rental total price does not match the expected price

    let mismatched_price_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": another_valid_machine_id,
            "user_id": valid_user_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "total_price": 100_000, // Mismatched price
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(mismatched_price_response.status(), 400);

    let mismatched_price_body = mismatched_price_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        mismatched_price_body["message"].as_str().unwrap(),
        "El precio total no es correcto"
    );

    // ---------- Rental with a user that is not a client

    let non_client_user_id = 1;

    let non_client_rental_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": another_valid_machine_id,
            "user_id": non_client_user_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "total_price": 665_000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(non_client_rental_response.status(), 404);

    let non_client_rental_body = non_client_rental_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        non_client_rental_body["message"].as_str().unwrap(),
        "No se ha encontrado al usuario o no es un cliente"
    );

    // ---------- Rental with a user that does not exist

    let non_existent_user_id = 9999;

    let non_existent_user_rental_response = client
        .post(backend_url("/staff/rental/new"))
        .json(&serde_json::json!({
            "machine_id": another_valid_machine_id,
            "user_id": non_existent_user_id,
            "start_date": valid_start_date,
            "end_date": valid_end_date,
            "total_price": 665_000,
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(non_existent_user_rental_response.status(), 404);

    let non_existent_user_rental_body = non_existent_user_rental_response
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert_eq!(
        non_existent_user_rental_body["message"].as_str().unwrap(),
        "No se ha encontrado al usuario o no es un cliente"
    );
}
