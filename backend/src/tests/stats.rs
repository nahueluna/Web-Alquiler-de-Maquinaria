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

#[tokio::test]
async fn test_get_stats_by_employee() {
    setup().await;
    let client = Client::new();

    //Get the access token needed
    let jwt = get_test_jwt("admin2@example.com", false).await;
    //Get rentals by employee - all time
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "rentals",
            "group_by": "employee"
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: Vec<NameValue> = serde_json::from_value(value).unwrap();
    assert!(stats.len() >= 2);
    assert_eq!(stats[0].name, "user22 u22");
    assert_eq!(stats[0].value, 6.0);
    assert_eq!(stats[1].name, "user23 u23");
    assert_eq!(stats[1].value, 3.0);

    //Get rentals by employee - all time - asc order
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "rentals",
            "group_by": "employee",
            "order": "asc"
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: Vec<NameValue> = serde_json::from_value(value).unwrap();
    assert!(stats.len() >= 2);
    assert_eq!(stats[0].name, "user23 u23");
    assert_eq!(stats[0].value, 3.0);
    assert_eq!(stats[1].name, "user22 u22");
    assert_eq!(stats[1].value, 6.0);

    //Get rentals by employee - 2024 - explicit desc order
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "rentals",
            "group_by": "employee",
            "order": "desc",
            "period": ["2024-01-01", "2024-12-31"]
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: Vec<NameValue> = serde_json::from_value(value).unwrap();
    assert!(stats.len() >= 2);
    assert_eq!(stats[0].name, "user22 u22");
    assert_eq!(stats[0].value, 2.0);
    assert_eq!(stats[1].name, "user23 u23");
    assert_eq!(stats[1].value, 1.0);

    //Get income by employee - all time
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "income",
            "group_by": "employee"
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: Vec<NameValue> = serde_json::from_value(value).unwrap();
    assert!(stats.len() >= 2);
    assert_eq!(stats[0].name, "user22 u22");
    assert_eq!(stats[0].value, 6000.0);
    assert_eq!(stats[1].name, "user23 u23");
    assert_eq!(stats[1].value, 3000.0);

    //Get income by employee - all time - asc order
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "income",
            "group_by": "employee",
            "order": "asc"
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: Vec<NameValue> = serde_json::from_value(value).unwrap();
    assert!(stats.len() >= 2);
    assert_eq!(stats[0].name, "user23 u23");
    assert_eq!(stats[0].value, 3000.0);
    assert_eq!(stats[1].name, "user22 u22");
    assert_eq!(stats[1].value, 6000.0);

    //Get income by employee - 2024 - explicit desc order
    let res = client
        .post(backend_url("/stats"))
        .json(&serde_json::json!({
            "access": jwt,
            "stat_type": "income",
            "group_by": "employee",
            "order": "desc",
            "period": ["2024-01-01", "2024-12-31"]
        })).send().await.unwrap();
    assert_eq!(res.status(), 200);

    let value = res.json::<serde_json::Value>().await.unwrap()["stats"].clone();
    let stats: Vec<NameValue> = serde_json::from_value(value).unwrap();
    assert!(stats.len() >= 2);
    assert_eq!(stats[0].name, "user22 u22");
    assert_eq!(stats[0].value, 2000.0);
    assert_eq!(stats[1].name, "user23 u23");
    assert_eq!(stats[1].value, 1000.0);
}
