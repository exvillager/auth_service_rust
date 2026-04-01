use serde::Serialize;
use tracing::{self};
use uuid::Uuid;

use crate::{
    models::user::User,
    repository::auth as auth_repo,
    utils::{
        auth_error::AuthError,
        util::{self, Token},
    },
};

#[derive(Serialize)]
pub struct LoginResult {
    pub user_id: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(username: String, password: String) -> Result<LoginResult, AuthError> {
    let user = auth_repo::find_user(&username)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err,"DB error during finding user {:?}", err);
            AuthError::DatabaseError
        })?
        .ok_or(AuthError::UserNotFound)?;

    let is_valid = util::verify_pass(password, user.password).map_err(|err| {
        tracing::error!(error = ?err,"error during verifying password {:?}", err);
        AuthError::VerifyPasswordError
    })?;

    if !is_valid {
        return Err(AuthError::InvalidCredentials);
    }

    let Token {
        access_token,
        refresh_token,
    } = util::generate_access_and_refresh_token(&user.id).map_err(|err| {
        tracing::error!(error = ?err,"error during generating access token: {:?}", err);
        AuthError::TokenGenerationError
    })?;

    Ok(LoginResult {
        user_id: user.id.to_string(),
        username: user.username,
        access_token: access_token,
        refresh_token: refresh_token,
    })
}

#[derive(Serialize)]
pub struct RegisterResult {
    pub id: String,
    pub email: String,
}

#[tracing::instrument(skip(password))]
pub async fn register(
    email: String,
    username: String,
    password: String,
) -> Result<RegisterResult, AuthError> {
    let existing_user = auth_repo::check_user_exists(&username, &email)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err,"DB error during checking existing user {:?}", err);
            AuthError::DatabaseError
        })?;

    if existing_user.username_exists {
        tracing::warn!("Username already taken: {}", username);
        return Err(AuthError::UsernameTaken);
    }

    if existing_user.email_exists {
        tracing::warn!("Email already taken: {}", email);
        return Err(AuthError::EmailTaken);
    }

    let hashed_pass = util::hash_password(password).map_err(|err| {
        tracing::error!(error = ?err,"error during hashing password {:?}", err);
        AuthError::HashingError
    })?;

    let user = auth_repo::create_user(&email, &username, &hashed_pass)
        .await
        .map_err(|err| {
            if let sqlx::Error::Database(db_err) = &err {
                if db_err.code() == Some("23505".into()) {
                    let msg = db_err.message();

                    if msg.contains("username") {
                        tracing::error!(error = ?err , "UserName already exists for {} {:?}",username,err);
                        return AuthError::UsernameTaken;
                    }
                    if msg.contains("email") {
                        tracing::error!(error = ?err , "email already exists for {} {:?}",email,err);
                        return AuthError::EmailTaken;
                    }
                }
            }

            tracing::error!(error = ?err,"DB error during creating user: {:?}", err);
            AuthError::DatabaseError
        })?;

    Ok(RegisterResult {
        id: user.id.to_string(),
        email: user.email,
    })
}

#[derive(Serialize)]
pub struct RefreshResult {
    pub access_token: String,
    pub refresh_token: String,
}
pub async fn refresh_token(old_token: String) -> Result<RefreshResult, AuthError> {
    let claims = util::decode_token(&old_token).await.map_err(|err| {
        tracing::error!(error = ?err,"error during decoding refresh token: {:?}", err);
        AuthError::RefreshToken
    })?;

    let Token {
        access_token,
        refresh_token,
    } = util::generate_access_and_refresh_token(&claims.sub).map_err(|err| {
        tracing::error!(error = ?err, "Token generation failed");
        AuthError::TokenGenerationError
    })?;

    // update user's refresh token
    auth_repo::rotate_refresh_token(&claims.sub, &old_token, &refresh_token)
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::RowNotFound => AuthError::InvalidToken,
                _ => {
                    tracing::error!(%err, "DB error during rotation");
                    AuthError::DatabaseError
                }
            }
        });

    Ok(RefreshResult {
        access_token,
        refresh_token,
    })
}

pub async fn delete_user(user_id: Uuid) -> Result<User, AuthError> {
    let deleted_user = auth_repo::delete_user(&user_id).await.map_err(|err| {
        if matches!(err, sqlx::Error::RowNotFound) {
            tracing::warn!(%user_id, "Delete failed: User does not exist");
            return AuthError::UserNotFound;
        }

        tracing::error!(%err, "Unexpected DB failure during deletion");
        AuthError::DatabaseError
    })?;

    Ok(deleted_user)
}

pub async fn list_users() -> Result<Vec<User>, AuthError> {
    let users = auth_repo::list_users().await.map_err(|err| {
        tracing::error!(%err, "DB error during getting all users");
        AuthError::DatabaseError
    })?;

    Ok(users)
}
