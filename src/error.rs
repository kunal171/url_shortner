use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    // Short machine-readable error code.
    pub error: String,

    // Human-readable message.
    pub message: String,
}

pub fn error_response(
    status: StatusCode,
    error: &str,
    message: &str,
) -> (StatusCode, Json<ErrorResponse>) {
    // Build consistent JSON error response.
    (
        status,
        Json(ErrorResponse {
            error: error.to_string(),
            message: message.to_string(),
        }),
    )
}
