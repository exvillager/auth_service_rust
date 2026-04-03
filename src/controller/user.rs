use axum::{Extension, Json};
use axum::http::StatusCode;

use crate::dto::user::{GetUserResponse, UpdateUserRequest, UpdatedUserResponse};
use crate::service::user as user_service;
use crate::utils::util::Claims;
use crate::utils::{api_response::ApiResponse, auth_error::ApiErr};

pub async fn get_me(
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<GetUserResponse>, ApiErr> {
    let user = user_service::get_user(&claims.sub)
        .await
        .map_err(ApiErr::from)?;

    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "retrieved user",
        data: user,
    })
}

pub async fn update(
    Extension(claims): Extension<Claims>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<ApiResponse<UpdatedUserResponse>, ApiErr> {
    let updated_user = user_service::update_user(&claims.sub, body)
        .await
        .map_err(ApiErr::from)?;
    Ok(ApiResponse {
        status: StatusCode::OK,
        message: "user updated successfully",
        data: updated_user,
    })
}
