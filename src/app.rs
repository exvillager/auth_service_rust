
// POST   /auth/signup ✅
// POST   /auth/login ✅
// POST   /auth/refresh ✅

// GET    /user/me ✅       (protected)
// PUT    /user/update ✅   (protected)

// GET    /health ✅
// GET    /metrics

use std::time::Duration;

use axum::{Router, http::Request, response::Response};
use tower_http::trace::TraceLayer;

use crate::routes;

pub fn create_app() -> Router {
    Router::new().merge(routes::create_routes()).layer(
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
