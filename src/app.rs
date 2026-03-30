// Routes
// POST   /auth/signup
// POST   /auth/login
// POST   /auth/refresh

// GET    /user/me        (protected)
// PUT    /user/update    (protected)

// GET    /health
// GET    /metrics

use axum::Router;

use crate::routes;

pub fn create_app() -> Router {
    Router::new().merge(routes::create_routes())
}
