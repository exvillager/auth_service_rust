use uuid::Uuid;

use crate::{
    controller::user::GetUserResponse, repository::user as user_repo, utils::auth_error::AuthError,
};

pub async fn get_user(user_id: &Uuid) -> Result<GetUserResponse, AuthError> {
    let user = user_repo::get_user_by_id(user_id)
        .await
        .map_err(|err| {
        tracing::error!(%err,"Db error during finding user");
        AuthError::DatabaseError
    })?;
    
    Ok(user)
}
