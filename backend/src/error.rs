use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
    pub details: Option<String>,
}

impl AppError {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        message: impl Into<String>,
        details: Option<String>,
    ) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            details,
        }
    }

    pub fn bad_request(code: &'static str, message: impl Into<String>, details: Option<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message, details)
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message, None)
    }

    pub fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, code, message, None)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal", message, None)
    }
}

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: ErrorBody<'a>,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    code: &'a str,
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<&'a str>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = ErrorResponse {
            error: ErrorBody {
                code: self.code,
                message: &self.message,
                details: self.details.as_deref(),
            },
        };
        (self.status, Json(body)).into_response()
    }
}
