use std::{env, fs};
use dotenvy::dotenv;
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::helpers::auth::create_2fa_code;
use crate::custom_types::structs::Claims;

static INIT: OnceCell<()> = OnceCell::const_new();

pub async fn setup() {
    INIT.get_or_init(|| async {
        dotenv().ok();

        let data_base_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set in .env file");

        // Connect to the database.
        let (client, connection) = tokio_postgres::connect(&data_base_url, NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {connection.await.unwrap()});

        let mut sql = fs::read_to_string("createdb.sql").unwrap();
        client.batch_execute(&sql).await.unwrap();

        sql = fs::read_to_string("populate_rows.sql").unwrap();
        client.batch_execute(&sql).await.unwrap();

    }).await;
}

pub fn backend_url(append: &str) -> String {
    format!("{}{}",env::var("BACKEND_URL").expect("BACKEND_URL must be set in the .env file"), append)
}

pub async fn get_test_jwt(email: &str, is_refresh: bool) -> String {

    let data_base_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set in .env file");

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(&data_base_url, NoTls).await.unwrap();
    tokio::spawn(async move {connection.await.unwrap()});

    let row = client.query_one("SELECT * FROM users WHERE email = $1;",
        &[&email]).await.unwrap();

    let user_id = row.get("id");
    let role = row.get("role");

    let secret_key = env::var("JWT_SECRET_KEY").expect("JTW_SECRET_KEY must be set in the .env file");

    let nonce = create_2fa_code();

    let claims = Claims {
        user_id,
        exp: usize::MAX,
        role,
        is_refresh,
        nonce,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).unwrap()
}
