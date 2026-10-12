use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde::Serialize;
use utoipa::ToSchema;

/// Failure while fetching or reading an upstream API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("upstream returned status {0}")]
    Status(u16),
    #[error("unexpected payload from {0}")]
    Payload(&'static str),
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub errors: Vec<ErrorItem>,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorItem {
    pub code: &'static str,
    pub message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!(error = %self, "upstream failure");
        let body = ErrorResponse {
            errors: vec![ErrorItem {
                code: "upstream_error",
                message: self.to_string(),
            }],
        };
        (StatusCode::BAD_GATEWAY, Json(body)).into_response()
    }
}
