use crate::custom_types::enums::RunningEnv;
use crate::helpers::auth::create_pool;
use crate::tests::helpers::*;
use reqwest::Client;

#[tokio::test]
async fn test_new_question() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    //Get the access token needed for new_question
    let jwt = get_test_jwt("newquestion@example.com", false).await;
    //Max size content (256)
    let max_size_content = "1234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678";
    assert_eq!(max_size_content.len(), 256);
    //New question
    let res = client
        .post(backend_url("/newquestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "model_id": 1,
            "content": max_size_content
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Question created successfully");
    //Check the question was saved
    db_client.query_one("SELECT * FROM questions JOIN users
        ON users.id = questions.user_id WHERE email = $1 AND model_id = $2
        AND content = $3;",
            &[&"newquestion@example.com", &&1, &max_size_content]).await.unwrap();

    //Empty content
    let res = client
        .post(backend_url("/newquestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "model_id": 1,
            "content": ""
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The question's content is empty");

    //Content too large
    let oversize_content = format!("{}a", max_size_content);
    let res = client
        .post(backend_url("/newquestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "model_id": 1,
            "content": oversize_content 
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The question's content is larger than 256 characters");

    //Invalid model_id
    let res = client
        .post(backend_url("/newquestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "model_id": i32::MAX,
            "content": "helloo"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "model_id is invalid");

    //Invalid role
    let jwt = get_test_jwt("admin@example.com", false).await;
    let res = client
        .post(backend_url("/newquestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "model_id": 1,
            "content": "Does it come with batteries?"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid role");

    //Invalid token
    let res = client
        .post(backend_url("/newquestion"))
        .json(&serde_json::json!({
            "access": "haloooo",
            "model_id": 1,
            "content": "Does it come with batteries?"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");

}

#[tokio::test]
async fn test_new_answer() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    //Get the access token needed for new_answer
    let jwt = get_test_jwt("newanswer@example.com", false).await;
    //Max size content (256)
    let max_size_content = "1234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678";
    assert_eq!(max_size_content.len(), 256);
    //New answer
    let res = client
        .post(backend_url("/newanswer"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "content": max_size_content
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Answer created successfully");
    //Check the answer was saved
    db_client.query_one("SELECT * FROM answers JOIN users
        ON users.id = answers.user_id WHERE email = $1 AND question_id = $2
        AND content = $3;",
            &[&"newanswer@example.com", &&1, &max_size_content]).await.unwrap();

    //Question already answered
    let res = client
        .post(backend_url("/newanswer"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "content": max_size_content
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The question has already been answered");

    //Empty content
    let res = client
        .post(backend_url("/newanswer"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "content": ""
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The answer's content is empty");

    //Content too large
    let oversize_content = format!("{}a", max_size_content);
    let res = client
        .post(backend_url("/newanswer"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "content": oversize_content 
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The answer's content is larger than 256 characters");

    //Invalid question_id
    let res = client
        .post(backend_url("/newanswer"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": i32::MAX,
            "content": "helloo"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "question_id is invalid");

    //Invalid role
    let jwt = get_test_jwt("newquestion@example.com", false).await;
    let res = client
        .post(backend_url("/newanswer"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "content": "Does it come with batteries?"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Not enough permissions");

    //Invalid token
    let res = client
        .post(backend_url("/newanswer"))
        .json(&serde_json::json!({
            "access": "haloooo",
            "question_id": 1,
            "content": "Does it come with batteries?"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");

}
