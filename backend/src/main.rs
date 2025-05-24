use crate::custom_types::enums::RunningEnv;
use crate::custom_types::structs::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use handlers::{auth::*, machinery_mgmt::explore_catalog};
use helpers::auth::create_pool;
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

mod custom_types;
mod handlers;
mod helpers;
mod tests;

#[tokio::main]
async fn main() {
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
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /register` goes to `client_sign_up`
        .route("/signup", post(client_sign_up))
        // `POST /login` goes to `client_login`
        .route("/login", post(client_login))
        .route("/explore", get(explore_catalog))
        .layer(
            CorsLayer::new()
                .allow_origin(vec!["http://localhost:5173".parse().unwrap()])
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
