use axum::{Json, http::StatusCode};
use serde::Deserialize;

use crate::service::auth::{self as auth_service, LoginResult, RegisterResult};
use crate::utils::api_response::ApiResponse;
use crate::utils::auth_error::ApiErr;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    Json(body): Json<LoginRequest>,
) -> Result<ApiResponse<LoginResult>, ApiErr> {
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
) -> Result<ApiResponse<RegisterResult>, ApiErr> {
    let result = auth_service::register(body.email, body.username, body.password)
        .await
        .map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::CREATED,
        message: "user created successfully",
        data: result,
    })
}
