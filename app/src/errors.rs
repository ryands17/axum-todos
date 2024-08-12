use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ApiErrors {
  #[error("Todo with id {0} not found")]
  TodoNotFound(String),
}

impl IntoResponse for ApiErrors {
  fn into_response(self) -> Response {
    let (status_code, error_message) = match self {
      ApiErrors::TodoNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
    };

    tracing::error!("Error: {status_code:?} with message {error_message:?}");

    (
      status_code,
      Json(json!({
        "error": error_message
      })),
    )
      .into_response()
  }
}
