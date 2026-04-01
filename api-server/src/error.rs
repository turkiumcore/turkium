use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("RPC Error: {0}")]
    RpcError(String),

    #[error("Node Connection Error: {0}")]
    NodeConnectionError(String),

    #[error("Invalid Request: {0}")]
    InvalidRequest(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Internal Server Error: {0}")]
    InternalError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Serialization Error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration Error: {0}")]
    ConfigError(String),

    #[error("Anyhow Error: {0}")]
    AnyhowError(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::AnyhowError(err.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::RpcError(msg) => (StatusCode::BAD_GATEWAY, msg),
            ApiError::NodeConnectionError(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            ApiError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Timeout(msg) => (StatusCode::GATEWAY_TIMEOUT, msg),
            ApiError::SerializationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            ApiError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::AnyhowError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
