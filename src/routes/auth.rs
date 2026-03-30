use crate::controller::auth;

use axum::{Router, routing::{get, post}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
}



async fn home() -> &'static str {
    "Hello /auth service"
}
