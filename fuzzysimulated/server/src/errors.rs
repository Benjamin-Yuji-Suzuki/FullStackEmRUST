use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("External service error: {0}")]
    External(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::External(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    fn check_status(err: AppError, expected: StatusCode) {
        let resp = err.into_response();
        assert_eq!(resp.status(), expected);
    }

    #[test]
    fn test_validation_status() {
        check_status(AppError::Validation("x".into()), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_not_found_status() {
        check_status(AppError::NotFound("x".into()), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_database_status() {
        check_status(AppError::Database(sqlx::Error::PoolTimedOut), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_external_status() {
        check_status(AppError::External("x".into()), StatusCode::BAD_GATEWAY);
    }
}
