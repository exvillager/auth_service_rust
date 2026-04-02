use axum::Extension;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::service::user as user_service;
use crate::utils::util::Claims;
use crate::utils::{api_response::ApiResponse, auth_error::ApiErr};

#[derive(Deserialize, Serialize, FromRow)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub refresh_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
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

pub async fn update() {}
