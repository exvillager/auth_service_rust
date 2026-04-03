use axum::extract::Path;
use axum::{Json, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::dto::auth::{LoginResult, RefreshResult, RegisteredUser};
use crate::dto::user::{DeletedUser, ListUser};
use crate::service::auth as auth_service;
use crate::utils::api_response::ApiResponse;
use crate::utils::auth_error::ApiErr;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

    pub async fn login(Json(body): Json<LoginRequest>) -> Result<ApiResponse<LoginResult>, ApiErr> {
    let result = auth_service::login(body.username, body.password)
        .await
        .map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "user logged in sucessfully",
        data: result,
    })
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

pub async fn register(
    Json(body): Json<RegisterRequest>,
) -> Result<ApiResponse<RegisteredUser>, ApiErr> {
    let result = auth_service::register(body.email, body.username, body.password)
        .await
        .map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::CREATED,
        message: "user created successfully",
        data: result,
    })
}

#[derive(Deserialize)]
pub struct RefreshTokenBody {
    pub refresh_token: String,
}
pub async fn refresh_token(
    Json(body): Json<RefreshTokenBody>,
) -> Result<ApiResponse<RefreshResult>, ApiErr> {
    let refresh_token = body.refresh_token;

    let result = auth_service::refresh_token(refresh_token)
        .await
        .map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "refresh token refreshed successfully",
        data: result,
    })
}

pub async fn delete_user(Path(id): Path<Uuid>) -> Result<ApiResponse<DeletedUser>, ApiErr> {
    let deleted_user = auth_service::delete_user(id).await.map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "User deleted successfully",
        data: deleted_user,
    })
}

pub async fn list_users() -> Result<ApiResponse<Vec<ListUser>>, ApiErr> {
    let all_users = auth_service::list_users().await.map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "Users retrieved successfully",
        data: all_users,
    })
}
