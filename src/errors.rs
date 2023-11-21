use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;

// errors with anyhow and convert into response
pub(crate) enum ApiError {
  InternalServerError(anyhow::Error),
  TodoNotFound(String),
}

impl From<anyhow::Error> for ApiError {
  fn from(inner: anyhow::Error) -> Self {
    ApiError::InternalServerError(inner)
  }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      ApiError::InternalServerError(error) => {
        tracing::error!("stacktrace: {}", error.backtrace());
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "Something went wrong!".to_string(),
        )
      }
      ApiError::TodoNotFound(id) => (
        StatusCode::NOT_FOUND,
        format!("Todo with id: {id:?} not found!"),
      ),
    };

    let body = json!({
      "error": error_message
    });

    tracing::error!("Error: {status:?} with message {error_message:?}");

    (status, Json(body)).into_response()
  }
}
