use axum::{
    Extension, extract::{Path, Request}, http::{HeaderMap, StatusCode, header::AUTHORIZATION}, middleware::Next, response::Response
};
use uuid::Uuid;

use crate::{
    config::envs::get_env, repository::auth as auth_repo, utils::{
        auth_error::{ApiErr, AuthError},
        util::{self, Claims},
    }
};

pub async fn auth_check(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiErr> {
    let bearer_token = headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .filter(|v| v.starts_with("Bearer "))
        .map(|v| v[7..].to_string());

    let cookie_token = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str.split(';').find_map(|cookie| {
                let cookie = cookie.trim();
                if cookie.starts_with("access_token=") {
                    Some(cookie["access_token=".len()..].to_string())
                } else {
                    None
                }
            })
        });

    let token = bearer_token
        .or(cookie_token)
        .ok_or(AuthError::Unauthorized)?;

    let decoded_token = util::decode_token(&token, get_env().access_token_secret.as_bytes())
        .map_err(|err| {
            tracing::error!(error = ?err,"error during decoding token: {:?}", err);
            AuthError::AccessTokenError
        })?;

    req.extensions_mut().insert(decoded_token);

    Ok(next.run(req).await)
}

pub async fn require_admin(
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<Response, ApiErr> {
    // check if the requested user and the id that needs to be deleted match
    let user = auth_repo::_find_user_by_id(&id).await.map_err(|err| {
        tracing::error!(error = ?err, "error during finding user by id: {:?}", err);
        AuthError::UserNotFound
    })?;
    
    if user.id != claims.sub {
        return Err(AuthError::Forbidden.into());
    }

    Ok(next.run(req).await)
}
