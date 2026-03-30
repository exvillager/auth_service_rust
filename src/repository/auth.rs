
use crate::{config::db, models::user::User};

pub async fn findUser(username: &str) -> Result<Option<User>, sqlx::Error> {
    let pool = db::get();
    let user = sqlx::query_as::<_, User>("")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    
    Ok(user)
}

pub async fn find_user_by_username_or_email(username: &str, email: &str) -> Result<Option<User>, sqlx::Error> {
    let pool = db::get();
    // TODO: sqlx query using pool
    None
}

pub async fn create_user(email: &str, username: &str, password: &str) -> Result<Option<User>, sqlx::Error> {
    let pool = db::get();
    // TODO: sqlx query using pool
    None
}
