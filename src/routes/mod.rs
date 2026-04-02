use axum::Router;

pub mod auth;
pub mod user;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/api/v1/users", user::routes())
        .nest("/api/v1/auth", auth::routes())
}
