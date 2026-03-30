use axum::Router;

pub mod auth;

pub fn create_routes() -> Router {
    Router::new()
        .merge(auth::routes())
}
