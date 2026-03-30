mod app;
mod controller;
mod models;
mod repository;
mod routes;
mod service;
mod utils;
mod config;

use app::create_app;

use crate::config::db::connect_db;

#[tokio::main]
async fn main() {
    let pool = connect_db().await.expect("DB connection failed");
    crate::config::db::init(pool);

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("error while binding to the port");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("error during creating the server")
}
