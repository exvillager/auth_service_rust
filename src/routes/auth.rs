use crate::{controller::auth, middleware::middleware};

use axum::{
    middleware::from_fn,
    routing::{delete, get, post},
    Router,
};

pub fn routes() -> Router {
    let public = Router::new()
        .route("/", get(home))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/refresh", post(auth::refresh_token));

    let protected = Router::new()
        .route("/list", get(auth::list_users))
        .layer(from_fn(middleware::auth_check));

    let user_protected_rouet = Router::new()
        .route("/delete/:id", delete(auth::delete_user))
        .layer(from_fn(middleware::require_admin))
        .layer(from_fn(middleware::auth_check));

    public
        .merge(protected)
        .merge(user_protected_rouet)
}

async fn home() -> &'static str {
    "Hello /auth service"
}
