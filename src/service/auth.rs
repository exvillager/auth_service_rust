use serde::Serialize;

use crate::{
    repository::auth as auth_repo,
    utils::{auth_error::AuthError, util},
};

#[derive(Serialize)]
pub struct LoginResult {
    pub user_id: String,
    pub username: String,
    pub access_token: String,
}

pub async fn login(username: String, password: String) -> Result<LoginResult, AuthError> {
    let user = auth_repo::findUser(&username)
        .await
        .map_err(|_err| AuthError::DatabaseError)?
        .ok_or(AuthError::UserNotFound)?;

    let is_valid =
        util::verify_pass(password, user.password).map_err(|err|AuthError::VerifyPasswordError(err))?; // map_err(Auth::VerifyPasswordError) works same

    if !is_valid {
        return Err(AuthError::InvalidCredentials);
    }

    let token = util::generate_access_token(user.id.clone())
        .map_err(|_err| AuthError::TokenGenerationError)?;

    Ok(LoginResult {
        user_id: user.id,
        username: user.username,
        access_token: token,
    })
}

#[derive(Serialize)]
pub struct RegisterResult {
    pub id: String,
    pub email: String,
}

pub async fn register(
    email: String,
    username: String,
    password: String,
) -> Result<RegisterResult, AuthError> {
    let existing_user = auth_repo::find_user_by_username_or_email(&username, &email)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

    if existing_user.is_some() {
        return Err(AuthError::UserAlreadyExists);
    }

    let hashed_pass = util::hash_password(password).map_err(|_| AuthError::HashingError)?;

    let user = auth_repo::create_user(&email, &username, &hashed_pass)
        .await
        .map_err(|_| AuthError::DatabaseError)?
        .ok_or(AuthError::DatabaseError)?;

    Ok(RegisterResult {
        id: user.id,
        email: user.email,
    })
}
