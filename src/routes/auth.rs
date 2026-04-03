use crate::{controller::auth, middleware::middleware};

use axum::{
    Router, middleware::from_fn, routing::{delete, get, post}
};

pub fn routes() -> Router {
    let public = Router::new()
        .route("/", get(home))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/refresh", post(auth::refresh_token));

    let protected = Router::new()
        .route("/delete/:id", delete(auth::delete_user))
        .route("/list", get(auth::list_users))
        .layer(from_fn(middleware::auth_check));

    public.merge(protected)
}

async fn home() -> &'static str {
    "Hello /auth service"
}
