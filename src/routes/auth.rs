use std::time::Duration;

use crate::controller::auth;

use axum::{
    Router,
    http::{Request, Response},
    routing::{delete, get, post},
};
use tower_http::trace::TraceLayer;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/list", get(auth::list_users))
        .route("/delete/:id", delete(auth::delete_user))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri()
                    )
                })
                .on_request(|_request: &Request<_>, _span: &tracing::Span| {
                    tracing::info!("Incoming Request");
                })
                .on_response(
                    |response: &Response<_>, latency: Duration, _span: &tracing::Span| {
                        tracing::info!(
                            status = %response.status(),
                            latency = ?latency,
                            "response sent"
                        );
                    },
                ),
        )
}

async fn home() -> &'static str {
    "Hello /auth service"
}
