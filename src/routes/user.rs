use axum::middleware::from_fn; 
use axum::{Router, routing::get};

use crate::controller::user as user_controller;
use crate::middleware::middleware;

pub fn routes() -> Router {
    Router::new()
        .route("/me", get(user_controller::get_me))
        .route("/update", get(user_controller::update))
        .layer(from_fn(middleware::auth_check))
}
