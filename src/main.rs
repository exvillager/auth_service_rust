mod app;
mod config;
mod controller;
mod models;
mod repository;
mod routes;
mod service;
mod utils;

use app::create_app;

use crate::config::db::connect_db;

use tracing_subscriber::{EnvFilter, fmt};

fn init_tracing() {
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_target(false)
        .compact()
        .init();
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
