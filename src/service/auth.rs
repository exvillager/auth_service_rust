use tracing::{self};
use uuid::Uuid;

use crate::{
    config::envs::get_env,
    dto::{
        auth::{LoginResult, RefreshResult, RegisteredUser},
        user::{DeletedUser, ListUser},
    },
    repository::auth as auth_repo,
    utils::{
        auth_error::AuthError,
        util::{self, Token},
    },
};

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

    // update refresh token in db
    auth_repo::update_refresh_token(&user.id, &refresh_token)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err,"DB error during adding refresh token in users DB {:?}", err);
            AuthError::DatabaseError
        })?;
    
    tracing::info!("Updated refresh token for user {:?}", user.id);

    Ok(LoginResult {
        user_id: user.id.to_string(),
        username: user.username,
        access_token: access_token,
        refresh_token: refresh_token,
    })
}

#[tracing::instrument(skip(password))]
pub async fn register(
    email: String,
    username: String,
    password: String,
) -> Result<RegisteredUser, AuthError> {
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

    Ok(user)
}

pub async fn refresh_token(old_token: String) -> Result<RefreshResult, AuthError> {
    let claims = util::decode_token(&old_token, get_env().refresh_token_secret.as_bytes())
        .map_err(|err| {
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
          .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                tracing::warn!(user_id = %claims.sub, "Refresh token mismatch — possible token reuse");
                AuthError::RefreshTokenMismatch
            }
            _ => {
                tracing::error!(%err, "DB error during token rotation");
                AuthError::DatabaseError
            }
        })?;

    Ok(RefreshResult {
        access_token,
        refresh_token,
    })
}

pub async fn delete_user(user_id: Uuid) -> Result<DeletedUser, AuthError> {
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

pub async fn list_users() -> Result<Vec<ListUser>, AuthError> {
    let users = auth_repo::list_users().await.map_err(|err| {
        tracing::error!(%err, "DB error during getting all users");
        AuthError::DatabaseError
    })?;

    Ok(users)
}
