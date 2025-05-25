use crate::custom_types::enums::RunningEnv;
use crate::custom_types::structs::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use handlers::{auth::*, machinery_mgmt::explore_catalog, machinery_mgmt::select_machine};
use helpers::auth::create_pool;
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

mod custom_types;
mod handlers;
mod helpers;
mod tests;

#[tokio::main]
async fn main() {
    dotenv().ok(); //Load .env
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set in the .env file");
    let socket_addr = env::var("SOCKET_ADDR").expect("SOCKET_ADDR must be set in the .env file");

    // Get the first CLI argument (after the executable name)
    let db_env = env::args()
        .nth(1)
        .expect("Missing environment parameter: Usage cargo run -- <prod|test>");

    // Create the pool
    let pool = match db_env.as_str() {
        "test" => create_pool(RunningEnv::Testing).await,
        "prod" => create_pool(RunningEnv::Production).await,
        other => {
            panic!(
                "Invalid environment parameter '{}': Usage cargo run -- <prod|test>",
                other
            );
        }
    };

    let shared_state = AppState {
        pool: Arc::new(pool),
    };

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/signup", post(client_sign_up))
        .route("/login", post(login))
        .route("/explore", get(explore_catalog))
        .route("/machinery/{id}", get(select_machine))
        .layer(
            CorsLayer::new()
                .allow_origin(vec![frontend_url.parse().unwrap()])
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
        .with_state(shared_state);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
