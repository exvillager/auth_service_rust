use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

pub struct ApiResponse<T: Serialize> {
    pub status: StatusCode,
    pub message: &'static str,
    pub data: T,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(json!({
                "message": self.message,
                "data": self.data,
            })),
        )
            .into_response()
    }
}
