use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    controller::{auth, user as user_controller},
    health_check,
    middleware::middleware,
};

// auth routes
fn auth_routes() -> Router {
    let public = Router::new()
        .route("/", get(auth_home))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/refresh", post(auth::refresh_token));

    let protected = Router::new()
        .route("/list", get(auth::list_users))
        .layer(from_fn(middleware::auth_check));

    let user_protected_route = Router::new()
        .route("/delete/:id", delete(auth::delete_user))
        .layer(from_fn(middleware::require_admin))
        .layer(from_fn(middleware::auth_check));

    public.merge(protected).merge(user_protected_route)
}

async fn auth_home() -> &'static str {
    "Hello /auth service"
}

// user routes
fn user_routes() -> Router {
    Router::new()
        .route("/me", get(user_controller::get_me))
        .route("/update", put(user_controller::update))
        .layer(from_fn(middleware::auth_check))
}

pub fn create_routes() -> Router {
    Router::new()
        .nest("/api/v1/users", user_routes())
        .nest("/api/v1/auth", auth_routes())
        .route("/health", get(health_check))
}
