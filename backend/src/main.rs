use axum::{
    routing::{get, post},
    Router,
};

use handlers::auth::{client_login, create_user, root};

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
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        // `POST /login` goes to `client_login`
        .route("/login", post(client_login));

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
