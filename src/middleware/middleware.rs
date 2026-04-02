use axum::{
    extract::Request,
    http::{HeaderMap, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::utils::{
    auth_error::{ApiErr, AuthError},
    util,
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

    let decoded_token = util::decode_token(&token).await.map_err(|err| {
        tracing::error!(error = ?err,"error during decoding token: {:?}", err);
        AuthError::RefreshToken
    })?;

    req.extensions_mut().insert(decoded_token);

    Ok(next.run(req).await)
}
