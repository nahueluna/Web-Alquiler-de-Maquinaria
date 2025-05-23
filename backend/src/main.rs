use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use handlers::auth::{client_login, client_sign_up, root};
use lettre::transport::smtp::client;

mod custom_types;
mod handlers;
mod tests;

#[tokio::main]
async fn main() {
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
        .layer(
            CorsLayer::new()
                .allow_origin(vec!["http://localhost:5173".parse().unwrap()]) // Allow all origins; restrict in production!
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        );

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
