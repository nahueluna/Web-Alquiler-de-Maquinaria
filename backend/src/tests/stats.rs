use crate::custom_types::{enums::RunningEnv, structs::*};
use crate::helpers::auth::{create_pool, send_mail, validate_jwt, encode_password};
use crate::tests::helpers::*;
use chrono::Datelike;
use reqwest::Client;
use tokio_postgres::Error;
use std::collections::HashMap;

#[tokio::test]
async fn test_get_stats_by_month() {
    setup().await;
    let client = Client::new();

    //Get the access token needed
    let jwt = get_test_jwt("admin2@example.com", false).await;
    //Get rentals by month in the current year
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "rentals",
            "group_by": "month"
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: HashMap<String, f64> = serde_json::from_value(value).unwrap();
    assert!(*stats.get("january").unwrap() >= 23.0);
    assert!(*stats.get("february").unwrap() >= 5.0);
    assert!(*stats.get("december").unwrap() >= 5.0);
    assert!(*stats.get("march").unwrap() <= 5.0);
    assert!(*stats.get("may").unwrap() <= 5.0);
    assert!(*stats.get("october").unwrap() <= 5.0);

    //Get rentals by month in 2024
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "rentals",
            "group_by": "month",
            "year": 2024
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: HashMap<String, f64> = serde_json::from_value(value).unwrap();
    assert_eq!(*stats.get("january").unwrap(), 0.0);
    assert_eq!(*stats.get("february").unwrap(), 0.0);
    assert_eq!(*stats.get("december").unwrap(), 0.0);
    assert_eq!(*stats.get("march").unwrap(), 5.0);
    assert_eq!(*stats.get("may").unwrap(), 0.0);
    assert_eq!(*stats.get("april").unwrap(), 5.0);

    //Get income by month in the current year
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "income",
            "group_by": "month"
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: HashMap<String, f64> = serde_json::from_value(value).unwrap();
    assert!(*stats.get("january").unwrap() >= 23000.0);
    assert!(*stats.get("february").unwrap() >= 5000.0);
    assert!(*stats.get("december").unwrap() >= 5000.0);
    assert!(*stats.get("march").unwrap() <= 5000.0);
    assert!(*stats.get("may").unwrap() <= 5000.0);
    assert!(*stats.get("october").unwrap() <= 5000.0);

    //Get income by month in 2024
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "income",
            "group_by": "month",
            "year": 2024
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: HashMap<String, f64> = serde_json::from_value(value).unwrap();
    println!("{:?}", stats);
    assert_eq!(*stats.get("january").unwrap(), 0.0);
    assert_eq!(*stats.get("february").unwrap(), 0.0);
    assert_eq!(*stats.get("december").unwrap(), 0.0);
    assert_eq!(*stats.get("march").unwrap(), 5000.0);
    assert_eq!(*stats.get("may").unwrap(), 0.0);
    assert_eq!(*stats.get("april").unwrap(), 5000.0);

    //Invalid role
    let jwt = get_test_jwt("user@example.com", false).await;
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "rentals",
            "group_by": "month"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 403);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "The user is not an admin");

    //Invalid token
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": "blablabla",
            "stat_type": "rentals",
            "group_by": "month"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    assert_eq!(res.json::<serde_json::Value>().await.unwrap()["message"], "Invalid access token");
}
