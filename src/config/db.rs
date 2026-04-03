    use std::sync::OnceLock;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::config::envs::get_env;

static POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

pub async fn connect_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = &get_env().database_url;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub fn init(pool: Pool<Postgres>) {
    POOL.set(pool).expect("DB pool already initialized");
}

pub fn get() -> &'static Pool<Postgres> {
    POOL.get().expect("DB pool not initialized")
}
