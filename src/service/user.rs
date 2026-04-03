use uuid::Uuid;

use crate::{
    controller::user::{GetUserResponse, UpdateUserRequest, UpdatedUserResponse},
    repository::user as user_repo,
    utils::auth_error::AuthError,
};

pub async fn get_user(user_id: &Uuid) -> Result<GetUserResponse, AuthError> {
    let user = user_repo::get_user_by_id(user_id).await.map_err(|err| {
        tracing::error!(%err,"Db error during finding user");
        AuthError::DatabaseError
    })?;

    Ok(user)
}

pub async fn update_user(
    user_id: &Uuid,
    body: UpdateUserRequest,
) -> Result<UpdatedUserResponse, AuthError> {
    if body.username.is_none() {
        return Err(AuthError::NothingToUpdate);
    }

    let updated_user = user_repo::update_user(user_id, body)
        .await
        .map_err(|err| {
            tracing::error!(%err, "DB error during user update");
            AuthError::DatabaseError
        })?;

    Ok(updated_user)
}
