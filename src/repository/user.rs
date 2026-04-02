use uuid::Uuid;

use crate::{config::db, controller::user::GetUserResponse};

pub async fn get_user_by_id(user_id: &Uuid) -> Result<GetUserResponse, sqlx::Error> {
    let pool = db::get();

    let user = sqlx::query_as::<_, GetUserResponse>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}
