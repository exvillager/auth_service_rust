use axum::extract::Path;
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::models::user::User;
use crate::service::auth::{self as auth_service, LoginResult, RegisterResult};
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

#[derive(Serialize)]
pub struct DeleteUserResponse {
    id: String,
}

pub async fn delete_user(
    Path(id): Path<Uuid>,  
) -> Result<ApiResponse<DeleteUserResponse>, ApiErr> {
    let deleted_user = auth_service::delete_user(id)
        .await
        .map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "User deleted successfully",
        data: DeleteUserResponse {
            id: deleted_user.id.to_string(),
        },
    })
}

pub async fn list_users() -> Result<ApiResponse<Vec<User>>, ApiErr> {
    let all_users = auth_service::list_users().await.map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "Users retrieved successfully",
        data: all_users,
    })
}
