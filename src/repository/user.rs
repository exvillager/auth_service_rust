use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    config::db,
    dto::user::{GetUserResponse, UpdateUserRequest, UpdatedUserResponse},
};

pub async fn get_user_by_id(user_id: &Uuid) -> Result<GetUserResponse, sqlx::Error> {
    let pool = db::get();

    let user = sqlx::query_as::<_, GetUserResponse>("SELECT * FROM users WHERE id = ?1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn update_user(user_id: &Uuid, body: UpdateUserRequest) -> Result<UpdatedUserResponse, sqlx::Error> {
    let pool = db::get();

    let mut builder = QueryBuilder::new("UPDATE users SET ");
    let mut separated = builder.separated(", ");

    if let Some(username) = body.username {
        separated.push("username = ");
        separated.push_bind_unseparated(username);
    }

    builder.push(" WHERE id = ");
    builder.push_bind(user_id);
    builder.push(" RETURNING id, email, username, created_at, updated_at");

    builder.build_query_as::<UpdatedUserResponse>()
        .fetch_one(pool)
        .await
}
