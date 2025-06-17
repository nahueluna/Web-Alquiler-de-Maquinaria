use crate::custom_types::enums::RunningEnv;
use crate::helpers::auth::create_pool;
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

    // ---------- Employee gets a unit history with an event without description

    let unit_with_event_without_description_id = 6;

    let response_with_event_without_description = http_client
        .post(format!(
            "{}/{}/history",
            backend_url("/unit"),
            unit_with_event_without_description_id
        ))
        .json(&serde_json::json!({
            "access": jwt
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response_with_event_without_description.status(), 200);

    let response_with_event_without_description_json: serde_json::Value =
        response_with_event_without_description
            .json()
            .await
            .unwrap();

    //println!("{:#?}", response_with_event_without_description_json);

    let unit_history_with_event_without_description = response_with_event_without_description_json
        ["history"]
        .as_array()
        .unwrap();

    assert!(!unit_history_with_event_without_description.is_empty());

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

#[tokio::test]
async fn test_update_unit_history() {
    setup().await;
    let http_client = Client::new();

    let pool = create_pool(RunningEnv::Testing);
    let db_client = match pool.await.get().await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to the database: {}", e),
    };

    let jwt = get_test_jwt("bob@example.com", true).await;

    // ---------- Employee updates a unit history with description

    let valid_unit_id = 4;

    match db_client
        .query(
            "
        SELECT * 
        FROM machinery_units mu
        INNER JOIN unit_history_events uhe 
        ON mu.id = uhe.unit_id 
        WHERE mu.id = $1;",
            &[&&valid_unit_id],
        )
        .await
    {
        Ok(row) => {
            assert!(row.is_empty());
        }
        Err(e) => {
            panic!("Failed to query the database: {}", e);
        }
    }

    let update_payload = serde_json::json!({
        "access": jwt,
        "unit_id": valid_unit_id,
        "new_status": "maintenance",
        "description": "Unit underwent routine maintenance."
    });

    let update_response = http_client
        .post(backend_url("/unit/history/update"))
        .json(&update_payload)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status(), 201);

    let update_response_json: serde_json::Value = update_response.json().await.unwrap();

    assert_eq!(
        update_response_json["message"],
        "El evento se ha registrado correctamente"
    );

    match db_client
        .query(
            "
        SELECT * 
        FROM machinery_units mu
        INNER JOIN unit_history_events uhe 
        ON mu.id = uhe.unit_id 
        WHERE mu.id = $1;",
            &[&&valid_unit_id],
        )
        .await
    {
        Ok(row) => {
            assert!(!row.is_empty());
        }
        Err(e) => {
            panic!("Failed to query the database: {}", e);
        }
    }

    // ---------- Employee updates a unit history without description

    let valid_unit_id_without_description = 4;

    let update_payload_without_description = serde_json::json!({
        "access": jwt,
        "unit_id": valid_unit_id_without_description,
        "new_status": "available",
    });

    let update_response_without_description = http_client
        .post(backend_url("/unit/history/update"))
        .json(&update_payload_without_description)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response_without_description.status(), 201);

    let update_response_without_description_json: serde_json::Value =
        update_response_without_description.json().await.unwrap();

    assert_eq!(
        update_response_without_description_json["message"],
        "El evento se ha registrado correctamente"
    );

    // ---------- Employee tries to update a unit history with an invalid unit ID

    let invalid_unit_id = 9999;

    let invalid_update_payload = serde_json::json!({
        "access": jwt,
        "unit_id": invalid_unit_id,
        "new_status": "maintenance",
        "description": "This unit does not exist."
    });

    let invalid_update_response = http_client
        .post(backend_url("/unit/history/update"))
        .json(&invalid_update_payload)
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_update_response.status(), 404);

    let invalid_update_response_json: serde_json::Value =
        invalid_update_response.json().await.unwrap();

    assert_eq!(
        invalid_update_response_json["message"],
        "La unidad indicada no existe"
    );

    // ---------- Employee tries to update a unit history with an invalid status

    let invalid_status_payload = serde_json::json!({
        "access": jwt,
        "unit_id": valid_unit_id,
        "new_status": "invalid_status",
    });

    let invalid_status_response = http_client
        .post(backend_url("/unit/history/update"))
        .json(&invalid_status_payload)
        .send()
        .await
        .unwrap();

    assert_eq!(invalid_status_response.status(), 422);

    // ---------- Client tries to update a unit history

    let client_jwt = get_test_jwt("dave@example.com", false).await;

    let client_update_payload = serde_json::json!({
        "access": client_jwt,
        "unit_id": valid_unit_id,
        "new_status": "maintenance",
    });

    let client_update_response = http_client
        .post(backend_url("/unit/history/update"))
        .json(&client_update_payload)
        .send()
        .await
        .unwrap();

    assert_eq!(client_update_response.status(), 403);
}
