use crate::tests::helpers::*;
use reqwest::Client;

#[tokio::test]
async fn test_get_machine_unit() {
    setup().await;
    let http_client = Client::new();

    let jwt = get_test_jwt("bob@example.com", true).await;

    // ---------- Employee get a valid machine unit

    let valid_serial_number = "CAT-001";
    let valid_response = http_client
        .post(format!("{}/{}", backend_url("/unit"), valid_serial_number))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response.status(), 200);

    let valid_response_json: serde_json::Value = valid_response.json().await.unwrap();

    let valid_machine_unit = valid_response_json["unit_info"].as_object().unwrap();
    assert_eq!(valid_machine_unit["serial_number"], valid_serial_number);

    // ---------- Employee get a valid machine unit with lowcase serial number

    let valid_serial_number_lowcase = "cat-001";
    let valid_response_lowcase = http_client
        .post(format!(
            "{}/{}",
            backend_url("/unit"),
            valid_serial_number_lowcase
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response_lowcase.status(), 200);

    let valid_response_json_lowcase: serde_json::Value =
        valid_response_lowcase.json().await.unwrap();

    let valid_machine_unit_lowcase = valid_response_json_lowcase["unit_info"]
        .as_object()
        .unwrap();

    assert_eq!(
        valid_machine_unit_lowcase["serial_number"],
        valid_serial_number
    );

    // ---------- Employee get a valid machine unit with spaces in serial number
    let valid_serial_number_with_spaces = "  CAT-001  ";

    let valid_response_with_spaces = http_client
        .post(format!(
            "{}/{}",
            backend_url("/unit"),
            valid_serial_number_with_spaces
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response_with_spaces.status(), 200);

    let valid_response_json_with_spaces: serde_json::Value =
        valid_response_with_spaces.json().await.unwrap();

    let valid_machine_unit_with_spaces = valid_response_json_with_spaces["unit_info"]
        .as_object()
        .unwrap();

    assert_eq!(
        valid_machine_unit_with_spaces["serial_number"],
        valid_serial_number
    );

    // ---------- Employee get a non-existing machine unit

    let non_existing_serial_number = "NON-EXISTING-001";

    let non_existing_response = http_client
        .post(format!(
            "{}/{}",
            backend_url("/unit"),
            non_existing_serial_number
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(non_existing_response.status(), 404);

    let non_existing_response_json: serde_json::Value = non_existing_response.json().await.unwrap();

    assert_eq!(
        non_existing_response_json["message"],
        "El número de serie ingresado no existe"
    );

    // ---------- Client tries to get a machine unit

    let client_jwt = get_test_jwt("dave@example.com", false).await;

    let client_response = http_client
        .post(format!("{}/{}", backend_url("/unit"), valid_serial_number))
        .json(&serde_json::json!({
            "access": client_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(client_response.status(), 403);

    let client_response_json: serde_json::Value = client_response.json().await.unwrap();

    assert_eq!(
        client_response_json["message"],
        "Solo empleados y administradores pueden acceder a esta información"
    );
}

#[tokio::test]
async fn test_get_unit_history() {
    setup().await;
    let http_client = Client::new();

    let jwt = get_test_jwt("bob@example.com", true).await;

    // ---------- Employee gets a valid unit history

    let valid_unit_id = 1;
    let valid_response = http_client
        .post(format!(
            "{}/{}/history",
            backend_url("/unit"),
            valid_unit_id
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(valid_response.status(), 200);

    let valid_response_json: serde_json::Value = valid_response.json().await.unwrap();

    let valid_unit_history = valid_response_json["history"].as_array().unwrap();

    assert!(!valid_unit_history.is_empty());

    // ---------- Employee gets a valid unit history with an invalid unit ID

    let invalid_unit_id = 9999;

    let invalid_response = http_client
        .post(format!(
            "{}/{}/history",
            backend_url("/unit"),
            invalid_unit_id
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_response.status(), 404);

    let invalid_response_json: serde_json::Value = invalid_response.json().await.unwrap();

    assert_eq!(
        invalid_response_json["message"],
        "La unidad solicitada no existe"
    );

    // ---------- Employee tries to get a history for a unit without it

    let no_history_unit_id = 11;

    let no_history_response = http_client
        .post(format!(
            "{}/{}/history",
            backend_url("/unit"),
            no_history_unit_id
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(no_history_response.status(), 200);

    let no_history_response_json: serde_json::Value = no_history_response.json().await.unwrap();

    let no_history_unit_history = no_history_response_json["history"].as_array().unwrap();

    assert!(no_history_unit_history.is_empty());

    // ---------- Client tries to get a unit history

    let client_jwt = get_test_jwt("dave@example.com", false).await;

    let client_response = http_client
        .post(format!(
            "{}/{}/history",
            backend_url("/unit"),
            valid_unit_id
        ))
        .json(&serde_json::json!({
            "access": client_jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(client_response.status(), 403);

    let client_response_json: serde_json::Value = client_response.json().await.unwrap();

    assert_eq!(
        client_response_json["message"],
        "Solo empleados y administradores pueden acceder a esta información"
    );
}
