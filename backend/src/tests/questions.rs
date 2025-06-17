use crate::custom_types::{enums::RunningEnv, structs::*};
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

#[tokio::test]
async fn test_vote_question() {
    setup().await;
    let client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    //Get the access token needed for new_question
    let jwt = get_test_jwt("newquestion@example.com", false).await;
    //Vote question
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "upvote": true
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Vote saved successfully");

    //Check the vote was saved
    db_client.query_one("SELECT * FROM question_votes JOIN users
        ON users.id = question_votes.user_id WHERE email = $1 AND question_id = $2;",
            &[&"newquestion@example.com", &&1]).await.unwrap();

    //Vote positive again
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "upvote": true
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The vote was already saved");

    //Check the vote is still there
    db_client.query_one("SELECT * FROM question_votes JOIN users
        ON users.id = question_votes.user_id WHERE email = $1 AND question_id = $2;",
            &[&"newquestion@example.com", &&1]).await.unwrap();

    //Vote negative
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "upvote": false
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Vote saved successfully");

    //Check the vote was deleted
    assert!(db_client.query_one("SELECT * FROM question_votes JOIN users
        ON users.id = question_votes.user_id WHERE email = $1 AND question_id = $2;",
            &[&"newquestion@example.com", &&1]).await.is_err());

    //Vote negative again
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "upvote": false
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The vote was already saved");

    //Check the vote is still deleted
    assert!(db_client.query_one("SELECT * FROM question_votes JOIN users
        ON users.id = question_votes.user_id WHERE email = $1 AND question_id = $2;",
            &[&"newquestion@example.com", &&1]).await.is_err());

    //Invalid question_id with positive vote
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": i32::MAX,
            "upvote": true
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "question_id is invalid");

    //Invalid question_id with negative vote
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": i32::MAX,
            "upvote": false
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "question_id is invalid");

    //Get admin token
    let jwt = get_test_jwt("newanswer@example.com", false).await;
    //Invalid role
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": jwt,
            "question_id": 1,
            "upvote": true
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid role");

    //Invalid token
    let res = client
        .post(backend_url("/votequestion"))
        .json(&serde_json::json!({
            "access": "no",
            "question_id": 1,
            "upvote": true
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");
}

#[tokio::test]
async fn test_get_unanswered_questions() {
    setup().await;
    let client = Client::new();

    //Get the access token
    let jwt = get_test_jwt("admin@example.com", false).await;
    //Get the questions
    let res = client
        .post(backend_url("/getunansweredquestions"))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["questions"].clone();
    let mut questions: Vec<UnansweredQuestion> = serde_json::from_value(value).unwrap();
    assert!(questions.len() > 1);
    //Question 1 is answered in a test above, we cant count on it
    if questions[0].question_id == 1 {
        questions.remove(0);
    }
    assert_eq!(questions[0].question_id, 3);
    assert_eq!(questions[0].content, "pregunta 3");
    assert_eq!(questions[0].model_id, 1);
    assert_eq!(questions[0].user_name, "user19");

    assert_eq!(questions[1].question_id, 5);
    assert_eq!(questions[1].content, "pregunta 5");
    assert_eq!(questions[1].model_id, 1);
    assert_eq!(questions[1].user_name, "user19");

    //Get client token
    let jwt = get_test_jwt("user@example.com", false).await;
    //Invalid role
    let res = client
        .post(backend_url("/getunansweredquestions"))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Not enough permissions");

    //Invalid token
    let res = client
        .post(backend_url("/getunansweredquestions"))
        .json(&serde_json::json!({
            "access": "no"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");
}

#[tokio::test]
async fn test_get_questions() {
    setup().await;
    let client = Client::new();

    //Get the access token
    let jwt = get_test_jwt("user@example.com", false).await;
    //Get the questions ordered by most recent
    let res = client
        .post(backend_url("/getquestions"))
        .json(&serde_json::json!({
            "access": jwt,
            "model_id": 1,
            "order_by_recent": true
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["questions"].clone();
    let mut questions: Vec<Question> = serde_json::from_value(value).unwrap();
    assert!(questions.len() > 4);
    //Question 6 is created in a test above
    if questions[0].question_id == 6 {
        questions.remove(0);
    }
    //As recent as Q4 but has more votes
    assert_eq!(questions[0].question_id, 5);
    assert_eq!(questions[0].content, "pregunta 5");
    assert_eq!(questions[0].user_name, "user19");
    assert_eq!(questions[0].upvoted, false);
    assert_eq!(questions[0].upvotes, 2);
    assert!(questions[0].answer.is_none());

    assert_eq!(questions[1].question_id, 4);
    assert_eq!(questions[1].content, "pregunta 4");
    assert_eq!(questions[1].user_name, "user19");
    assert_eq!(questions[1].upvoted, false);
    assert_eq!(questions[1].upvotes, 0);
    assert_eq!(questions[1].answer.as_ref().unwrap().content, "r2 a p4");

    //As recent as Q1 and Q2 but has more votes
    assert_eq!(questions[2].question_id, 3);
    assert_eq!(questions[2].content, "pregunta 3");
    assert_eq!(questions[2].user_name, "user19");
    assert_eq!(questions[2].upvoted, false);
    assert_eq!(questions[2].upvotes, 2);
    assert!(questions[2].answer.is_none());

    assert_eq!(questions[3].question_id, 1);
    assert_eq!(questions[3].content, "pregunta 1");
    assert_eq!(questions[3].user_name, "user19");
    assert_eq!(questions[3].upvoted, true);
    assert_eq!(questions[3].upvotes, 1);
    assert!(questions[3].answer.is_none());

    assert_eq!(questions[4].question_id, 2);
    assert_eq!(questions[4].content, "pregunta 2");
    assert_eq!(questions[4].user_name, "user19");
    assert_eq!(questions[4].upvoted, false);
    assert_eq!(questions[4].upvotes, 0);
    assert_eq!(questions[4].answer.as_ref().unwrap().content, "r1 a p2");

    //Get the questions ordered by most votes
    let res = client
        .post(backend_url("/getquestions"))
        .json(&serde_json::json!({
            "access": jwt,
            "model_id": 1,
            "order_by_recent": false
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["questions"].clone();
    let mut questions: Vec<Question> = serde_json::from_value(value).unwrap();
    assert!(questions.len() > 4);
    //Question 6 is created in a test above
    if questions[0].question_id == 6 {
        questions.remove(0);
    }
    //As many votes as Q3 but more recent
    assert_eq!(questions[0].question_id, 5);
    assert_eq!(questions[0].upvotes, 2);

    assert_eq!(questions[1].question_id, 3);
    assert_eq!(questions[1].upvotes, 2);

    assert_eq!(questions[2].question_id, 1);
    assert_eq!(questions[2].upvotes, 1);

    //As many votes as Q2 but more recent
    assert_eq!(questions[3].question_id, 4);
    assert_eq!(questions[3].upvotes, 0);

    assert_eq!(questions[4].question_id, 2);
    assert_eq!(questions[4].upvotes, 0);

    //Test without access token
    let res = client
        .post(backend_url("/getquestions"))
        .json(&serde_json::json!({
            "model_id": 1,
            "order_by_recent": false
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let value = res.json::<serde_json::Value>().await.unwrap()["questions"].clone();
    let mut questions: Vec<Question> = serde_json::from_value(value).unwrap();
    assert!(questions.len() > 4);
    //Question 6 is created in a test above
    if questions[0].question_id == 6 {
        questions.remove(0);
    }
    //As many votes as Q3 but more recent
    assert_eq!(questions[0].question_id, 5);
    assert_eq!(questions[0].upvotes, 2);

    assert_eq!(questions[1].question_id, 3);
    assert_eq!(questions[1].upvotes, 2);

    assert_eq!(questions[2].question_id, 1);
    assert_eq!(questions[2].upvotes, 1);

    //As many votes as Q2 but more recent
    assert_eq!(questions[3].question_id, 4);
    assert_eq!(questions[3].upvotes, 0);

    assert_eq!(questions[4].question_id, 2);
    assert_eq!(questions[4].upvotes, 0);

    //Invalid token
    let res = client
        .post(backend_url("/getquestions"))
        .json(&serde_json::json!({
            "access": "guagua",
            "model_id": 1,
            "order_by_recent": false
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");
}
