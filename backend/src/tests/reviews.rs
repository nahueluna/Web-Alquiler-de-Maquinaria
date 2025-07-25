use crate::custom_types::enums::RunningEnv;
use crate::custom_types::structs::{ServiceReview, MachineReview};
use crate::helpers::auth::create_pool;
use crate::tests::helpers::*;
use reqwest::Client;
use chrono::NaiveDateTime;

#[tokio::test]
async fn test_new_machine_review() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    //Get the access token needed
    let jwt = get_test_jwt("user@example.com", false).await;
    //Max size content (256)
    let max_size_content = "1234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678";
    assert_eq!(max_size_content.len(), 256);
    //New review
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 5,
            "content": max_size_content
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Review saved successfully");
    //Check the answer was saved
    db_client.query_one("SELECT * FROM machine_reviews JOIN users
        ON users.id = machine_reviews.user_id
        WHERE email = $1 AND rental_id = $2
        AND user_id = $3 AND model_id = $4
        AND rating = $5 AND content = $6;",
            &[&"user@example.com", &&60, &&21, &&8, &&5i16, &max_size_content]).await.unwrap();

    //Try again with required content (rating < 5)
    db_client.execute("DELETE FROM machine_reviews WHERE rental_id = $1;",
            &[&&60]).await.unwrap();
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 1,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Review saved successfully");
    //Check the answer was saved
    db_client.query_one("SELECT * FROM machine_reviews JOIN users
        ON users.id = machine_reviews.user_id
        WHERE email = $1 AND rental_id = $2
        AND user_id = $3 AND model_id = $4
        AND rating = $5 AND content = $6;",
            &[&"user@example.com", &&60, &&21, &&8, &&1i16, &"a"]).await.unwrap();

    //Rental already reviewed
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "This rental has already been reviewed");

    //Rating < 1
    db_client.execute("DELETE FROM machine_reviews WHERE rental_id = $1;",
            &[&&60]).await.unwrap();
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 0,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rating must be between 1 and 5 inclusive");

    //Rating > 5
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 6,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rating must be between 1 and 5 inclusive");


    //Missing content
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Content is required on reviews with less than 5 stars");

    //Content too short
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": ""
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Content must be between 1 and 256 characters inclusive");

    //Content too large
    let oversize_content = format!("{}a", max_size_content);
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": oversize_content 
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Content must be between 1 and 256 characters inclusive");


    //Invalid rental id
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": i32::MAX,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rental not found, does not belong to user, or is not completed");

    //Rental does not belong to user
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 1,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rental not found, does not belong to user, or is not completed");

    //Rental is not completed
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 59,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rental not found, does not belong to user, or is not completed");

    //Invalid role
    let jwt = get_test_jwt("admin@example.com", false).await;
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid role");

    //Invalid token
    let res = client
        .post(backend_url("/reviews/machines/new"))
        .json(&serde_json::json!({
            "access": "hihihiha",
            "rental_id": 60,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");
}

#[tokio::test]
async fn test_new_service_review() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    //Get the access token needed
    let jwt = get_test_jwt("user@example.com", false).await;
    //Max size content (256)
    let max_size_content = "1234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678";
    assert_eq!(max_size_content.len(), 256);
    //New review
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 5,
            "content": max_size_content
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Review saved successfully");
    //Check the answer was saved
    db_client.query_one("SELECT * FROM service_reviews JOIN users
        ON users.id = service_reviews.user_id
        WHERE email = $1 AND rental_id = $2
        AND user_id = $3
        AND rating = $4 AND content = $5;",
            &[&"user@example.com", &&60, &&21,&&5i16, &max_size_content]).await.unwrap();

    //Try again with required content (rating < 5)
    db_client.execute("DELETE FROM service_reviews WHERE rental_id = $1;",
            &[&&60]).await.unwrap();
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 1,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Review saved successfully");
    //Check the answer was saved
    db_client.query_one("SELECT * FROM service_reviews JOIN users
        ON users.id = service_reviews.user_id
        WHERE email = $1 AND rental_id = $2
        AND user_id = $3
        AND rating = $4 AND content = $5;",
            &[&"user@example.com", &&60, &&21, &&1i16, &"a"]).await.unwrap();

    //Rental already reviewed
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "This rental has already been reviewed");

    //Rating < 1
    db_client.execute("DELETE FROM service_reviews WHERE rental_id = $1;",
            &[&&60]).await.unwrap();
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 0,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rating must be between 1 and 5 inclusive");

    //Rating > 5
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 6,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rating must be between 1 and 5 inclusive");


    //Missing content
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Content is required on reviews with less than 5 stars");

    //Content too short
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": ""
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Content must be between 1 and 256 characters inclusive");

    //Content too large
    let oversize_content = format!("{}a", max_size_content);
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": oversize_content 
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Content must be between 1 and 256 characters inclusive");


    //Invalid rental id
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": i32::MAX,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rental not found, does not belong to user, or is not completed");

    //Rental does not belong to user
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 1,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rental not found, does not belong to user, or is not completed");

    //Rental is not completed
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 59,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Rental not found, does not belong to user, or is not completed");

    //Invalid role
    let jwt = get_test_jwt("admin@example.com", false).await;
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": jwt,
            "rental_id": 60,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid role");

    //Invalid token
    let res = client
        .post(backend_url("/reviews/service/new"))
        .json(&serde_json::json!({
            "access": "hihihiha",
            "rental_id": 60,
            "rating": 3,
            "content": "a"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");
}

#[tokio::test]
async fn test_get_service_reviews() {
    setup().await;
    let client = Client::new();

    //Get the access token
    let jwt = get_test_jwt("admin@example.com", false).await;
    //Get the reviews - ordered by recent by default
    let res = client
        .post(backend_url("/reviews/service/get"))
        .json(&serde_json::json!({
            "access": jwt,
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["reviews"].clone();
    let reviews: Vec<ServiceReview> = serde_json::from_value(value).unwrap();
    assert!(reviews.len() >= 5);
    let mut previous = NaiveDateTime::MAX;
    for r in reviews {
        assert!(r.created_at < previous);
        previous = r.created_at;
    }

    //Get the reviews - ordered by recent
    let res = client
        .post(backend_url("/reviews/service/get"))
        .json(&serde_json::json!({
            "access": jwt,
            "order":"recent"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["reviews"].clone();
    let reviews: Vec<ServiceReview> = serde_json::from_value(value).unwrap();
    assert!(reviews.len() >= 5);
    let mut previous = NaiveDateTime::MAX;
    for r in reviews {
        assert!(r.created_at < previous);
        previous = r.created_at;
    }

    //Get the reviews - ordered by more rating
    let res = client
        .post(backend_url("/reviews/service/get"))
        .json(&serde_json::json!({
            "access": jwt,
            "order":"more_rating"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["reviews"].clone();
    let reviews: Vec<ServiceReview> = serde_json::from_value(value).unwrap();
    assert!(reviews.len() >= 5);
    let mut previous = i16::MAX;
    for r in reviews {
        assert!(r.rating < previous);
        previous = r.rating;
    }

    //Get the reviews - ordered by less rating
    let res = client
        .post(backend_url("/reviews/service/get"))
        .json(&serde_json::json!({
            "access": jwt,
            "order":"less_rating"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["reviews"].clone();
    let reviews: Vec<ServiceReview> = serde_json::from_value(value).unwrap();
    assert!(reviews.len() >= 5);
    let mut previous = i16::MIN;
    for r in reviews {
        assert!(r.rating > previous);
        previous = r.rating;
    }

    //Get the reviews - filter by rating
    let res = client
        .post(backend_url("/reviews/service/get"))
        .json(&serde_json::json!({
            "access": jwt,
            "rating":3
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["reviews"].clone();
    let reviews: Vec<ServiceReview> = serde_json::from_value(value).unwrap();
    assert!(reviews.len() >= 1);
    for r in reviews {
        assert_eq!(r.rating, 3);
    }

    //Invalid role
    let jwt = get_test_jwt("user@example.com", false).await;
    let res = client
        .post(backend_url("/reviews/service/get"))
        .json(&serde_json::json!({
            "access": jwt,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Not enough permissions");

    //Invalid token
    let res = client
        .post(backend_url("/reviews/service/get"))
        .json(&serde_json::json!({
            "access": "anything at all",
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");
}

#[tokio::test]
async fn test_get_machine_reviews() {
    setup().await;
    let client = Client::new();

    //Get the reviews - ordered by recent by default
    let res = client
        .post(backend_url("/reviews/machines/get"))
        .json(&serde_json::json!({
            "model_id" : 1
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let reviews: Vec<MachineReview> = serde_json::from_value(value["reviews"].clone()).unwrap();
    let avg: f32 = serde_json::from_value(value["average_rating"].clone()).unwrap();
    assert_eq!(avg, 3.0);
    assert!(reviews.len() >= 5);
    let mut previous = NaiveDateTime::MAX;
    for r in reviews {
        assert!(r.created_at < previous);
        previous = r.created_at;
    }

    //Get the reviews - ordered by recent
    let res = client
        .post(backend_url("/reviews/machines/get"))
        .json(&serde_json::json!({
            "order":"recent",
            "model_id" : 1
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let reviews: Vec<MachineReview> = serde_json::from_value(value["reviews"].clone()).unwrap();
    let avg: f32 = serde_json::from_value(value["average_rating"].clone()).unwrap();
    assert_eq!(avg, 3.0);
    assert!(reviews.len() >= 5);
    let mut previous = NaiveDateTime::MAX;
    for r in reviews {
        assert!(r.created_at < previous);
        previous = r.created_at;
    }

    //Get the reviews - ordered by more rating
    let res = client
        .post(backend_url("/reviews/machines/get"))
        .json(&serde_json::json!({
            "order":"more_rating",
            "model_id":1
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let reviews: Vec<MachineReview> = serde_json::from_value(value["reviews"].clone()).unwrap();
    let avg: f32 = serde_json::from_value(value["average_rating"].clone()).unwrap();
    assert_eq!(avg, 3.0);
    assert!(reviews.len() >= 5);
    let mut previous = i16::MAX;
    for r in reviews {
        assert!(r.rating < previous);
        previous = r.rating;
    }

    //Get the reviews - ordered by less rating
    let res = client
        .post(backend_url("/reviews/machines/get"))
        .json(&serde_json::json!({
            "order":"less_rating",
            "model_id":1
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let reviews: Vec<MachineReview> = serde_json::from_value(value["reviews"].clone()).unwrap();
    let avg: f32 = serde_json::from_value(value["average_rating"].clone()).unwrap();
    assert_eq!(avg, 3.0);
    assert!(reviews.len() >= 5);
    let mut previous = i16::MIN;
    for r in reviews {
        assert!(r.rating > previous);
        previous = r.rating;
    }

    //Get the reviews - filter by rating
    let res = client
        .post(backend_url("/reviews/machines/get"))
        .json(&serde_json::json!({
            "rating":3,
            "model_id":1
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap();
    let reviews: Vec<MachineReview> = serde_json::from_value(value["reviews"].clone()).unwrap();
    let avg: f32 = serde_json::from_value(value["average_rating"].clone()).unwrap();
    assert_eq!(avg, 3.0);
    assert!(reviews.len() >= 1);
    for r in reviews {
        assert_eq!(r.rating, 3);
    }
}
