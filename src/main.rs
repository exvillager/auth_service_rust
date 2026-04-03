mod app;
mod config;
mod controller;
mod middleware;
mod models;
mod repository;
mod routes;
mod service;
mod utils;

use app::create_app;
use axum::Json;
use serde::Serialize;

use crate::{
    config::db::{self, connect_db},
    utils::{api_response::ApiResponse, auth_error::ApiErr},
};

use tracing_subscriber::{EnvFilter, fmt};

fn init_tracing() {
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_target(false)
        .compact()
        .init();
}

// POST   /auth/signup ✅
// POST   /auth/login ✅
// POST   /auth/refresh ✅

// GET    /user/me ✅       (protected)
// PUT    /user/update ✅   (protected)

// GET    /health ✅
// GET    /metrics

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub db: &'static str,
    pub version: &'static str,
}
pub async fn health_check() -> Json<HealthResponse> {
    let db_status = match db::get().acquire().await {
        Ok(_) => "up",
        Err(_) => "down"
    };
    
    let status = if db_status == "up" {
        "ok"
    } else {
        "degraded"
    };
    
    Json(HealthResponse {
        status,
        db: db_status,
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_tracing();
    let pool = connect_db().await.expect("DB connection failed");
    crate::config::db::init(pool);

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("error while binding to the port");

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("error during creating the server")
}
