use uuid::Uuid;

use crate::{config::db, dto::{auth::RegisteredUser, user::{DeletedUser, ListUser}}, models::user::User};

pub async fn find_user(username: &str) -> Result<Option<User>, sqlx::Error> {
    let pool = db::get();
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
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
            EXISTS (SELECT 1 FROM "users" WHERE username = $1) AS username_exists,
            EXISTS (SELECT 1 FROM "users" WHERE email = $2) AS email_exists
        "#,
    )
    .bind(username)
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn create_user(email: &str, username: &str, password: &str) -> Result<RegisteredUser, sqlx::Error> {
    let pool = db::get();
    let created_user = sqlx::query_as::<_, RegisteredUser>(
        r#"
            INSERT INTO "users" (email, username, password)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, created_at
        "#,
    )
    .bind(email)
    .bind(username)
    .bind(password)
    .fetch_one(pool)
    .await?;

    Ok(created_user)
}


pub async fn delete_user(user_id: &Uuid) -> Result<DeletedUser, sqlx::Error> {
    let pool = db::get();
    let deleted_user = sqlx::query_as::<_, DeletedUser>(
        r#"
                DELETE FROM "users"
                WHERE id = $1
                RETURNING id, username
                "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(deleted_user)
}

pub async fn list_users() -> Result<Vec<ListUser>, sqlx::Error> {
    let pool = db::get();
    let users = sqlx::query_as::<_, ListUser>(
        r#"
                SELECT
                    *
                FROM "users";
                "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}


pub async fn rotate_refresh_token(
    user_id: &Uuid,
    old_token: &str,
    new_token: &str,
) -> Result<User, sqlx::Error> {
    let pool = db::get();

    let updated = sqlx::query_as(
        r#"
                UPDATE users
                SET refresh_token = $3
                WHERE id = $1 AND refresh_token = $2
                RETURNING *
                "#,
    )
    .bind(user_id)
    .bind(old_token)
    .bind(new_token)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

pub async fn _find_user_by_id(user_id: &Uuid) -> Result<User, sqlx::Error> {
    let pool = db::get();

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn _check_refresh_token_valid(
    user_id: &Uuid,
    refresh_token: &str,
) -> Result<bool, sqlx::Error> {
    let pool = db::get();
    let exists: (bool,) = sqlx::query_as(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE id = $1 AND refresh_token = $2
            )
            "#,
    )
    .bind(user_id)
    .bind(refresh_token)
    .fetch_one(pool)
    .await?;

    Ok(exists.0)
}

pub async fn _update_user_refresh_token(
    user_id: &Uuid,
    refresh_token: &str,
) -> Result<User, sqlx::Error> {
    let pool = db::get();

    let updated = sqlx::query_as(
        r#"
            UPDATE users
            SET refresh_token = $2
            WHERE id = $1
            RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(refresh_token)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

#[derive(sqlx::FromRow)]
pub struct UpdatedUser {
    pub id: Uuid,
    pub username: String,
    pub refresh_token: Option<String>,
}
pub async fn update_refresh_tokem(
    user_id: &Uuid,
    new_refresh_token: &str,
) -> Result<UpdatedUser, sqlx::Error> {
    let pool = db::get();

    let updated = sqlx::query_as::<_, UpdatedUser>(
        r#"
            UPDATE users
            SET refresh_token = $2
            WHERE id = $1
            RETURNING id,username,refresh_token
        "#,
    )
    .bind(user_id)
    .bind(new_refresh_token)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}
