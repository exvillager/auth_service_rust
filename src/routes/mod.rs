use axum::{Router, routing::get};

use crate::health_check;

pub mod auth;
pub mod user;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/api/v1/users", user::routes())
        .nest("/api/v1/auth", auth::routes())
        .route("/health", get(health_check))
}
