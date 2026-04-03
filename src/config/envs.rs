use std::env;
use std::sync::OnceLock;

pub struct ENV {
    pub database_url: String,
    pub access_token_secret: String,
    pub refresh_token_secret: String,
}

static ENV_INSTANCE: OnceLock<ENV> = OnceLock::new();

pub fn init_env() {
    ENV_INSTANCE.get_or_init(|| ENV {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        access_token_secret: env::var("ACCESS_TOKEN_SECRET")
            .expect("ACCESS_TOKEN_SECRET must be set"),
        refresh_token_secret: env::var("REFRESH_TOKEN_SECRET")
            .expect("REFRESH_TOKEN_SECRET must be set"),
    });
}

pub fn get_env() -> &'static ENV {
    ENV_INSTANCE
        .get()
        .expect("ENV not initialized. Call init_env() first in main.rs")
}
