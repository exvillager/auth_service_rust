use std::sync::OnceLock;
use sqlx::{Pool, Sqlite, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use std::str::FromStr;

use crate::config::envs::get_env;

static POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub async fn connect_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = &get_env().database_url;

    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub fn init(pool: Pool<Sqlite>) {
    POOL.set(pool).expect("DB pool already initialized");
}

pub fn get() -> &'static Pool<Sqlite> {
    POOL.get().expect("DB pool not initialized")
}
