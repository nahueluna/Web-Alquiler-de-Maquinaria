use std::{env, fs};
use dotenvy::dotenv;
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;

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
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let mut sql = fs::read_to_string("createdb.sql").unwrap();
        client.batch_execute(&sql).await.unwrap();

        sql = fs::read_to_string("populate_rows.sql").unwrap();
        client.batch_execute(&sql).await.unwrap();

    }).await;
}
