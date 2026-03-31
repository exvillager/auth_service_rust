use uuid::Uuid;

use crate::{config::db, models::user::User};

pub async fn find_user(username: &str) -> Result<Option<User>, sqlx::Error> {
    let pool = db::get();
    let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

#[derive(sqlx::FromRow)]
pub struct UserExistence {
    pub username_exists: bool,
    pub email_exists: bool,
}

pub async fn check_user_exists(username: &str, email: &str) -> Result<UserExistence, sqlx::Error> {
    let pool = db::get();

    let result = sqlx::query_as::<_, UserExistence>(
        r#"
        SELECT
            EXISTS (SELECT 1 FROM "user" WHERE username = $1) AS username_exists,
            EXISTS (SELECT 1 FROM "user" WHERE email = $2) AS email_exists
        "#,
    )
    .bind(username)
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn create_user(email: &str, username: &str, password: &str) -> Result<User, sqlx::Error> {
    let pool = db::get();
    let created_user = sqlx::query_as::<_, User>(
        r#"
                    INSERT INTO "user" (email, username, password)
                    VALUES ($1, $2, $3)
                    RETURNING id, email, username, password, created_at
                    "#,
    )
    .bind(email)
    .bind(username)
    .bind(password)
    .fetch_one(pool)
    .await?;

    Ok(created_user)
}

pub async fn find_user_by_id(user_id: &Uuid) -> Result<User, sqlx::Error> {
    let pool = db::get();

    let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn delete_user(user_id: &Uuid) -> Result<User, sqlx::Error> {
    let pool = db::get();
    let deleted_user = sqlx::query_as::<_, User>(
        r#"
                DELETE FROM "user"
                WHERE id = $1
                RETURNING id, username, email, password, created_at
                "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(deleted_user)
}

pub async fn list_users() -> Result<Vec<User>, sqlx::Error> {
    let pool = db::get();
    let users = sqlx::query_as::<_, User>(
        r#"
                SELECT
                    id,
                    username,
                    email,
                    password,
                    created_at
                FROM "user"
                "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}
