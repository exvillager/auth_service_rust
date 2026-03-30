use std::env;
use std::sync::OnceLock;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

static POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

pub async fn connect_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

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
