use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub struct ApiErr {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiErr {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(json!({
                "error": self.message,
            })),
        )
            .into_response()
    }
}

pub enum AuthError {
    InvalidCredentials,
    UserNotFound,
    UserAlreadyExists,
    HashingError,
    DatabaseError,
    VerifyPasswordError(err),
    TokenGenerationError,
}

impl From<AuthError> for ApiErr {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidCredentials => ApiErr {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid credentials".to_string(),
            },
            AuthError::UserNotFound => ApiErr {
                status: StatusCode::NOT_FOUND,
                message: "User not found".to_string(),
            },
            AuthError::UserAlreadyExists => ApiErr {
                status: StatusCode::CONFLICT,
                message: "User already exists".to_string(),
            },
            AuthError::HashingError => ApiErr {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Password hashing failed".to_string(),
            },
            AuthError::DatabaseError => ApiErr {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Database error".to_string(),
            },
            AuthError::VerifyPasswordError(_) => ApiErr {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Password verification failed".to_string(),
            },
            AuthError::TokenGenerationError => ApiErr {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Token generation failed".to_string(),
            },
        }
    }
}
