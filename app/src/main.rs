mod errors;
mod todos;

use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use tokio::net::TcpListener;

pub(crate) fn router() -> Router {
  Router::new()
    .route("/", get(hello))
    .nest("/todos", todos::todos_service())
}

#[tokio::main]
async fn main() -> Result<()> {
  // initialise tracing
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .with_line_number(true)
    .init();

  let app = router();
  let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
  let addr = listener.local_addr().unwrap().to_string();

  tracing::info!("App running on http://{}", addr);

  axum::serve(listener, app.into_make_service()).await?;

  Ok(())
}

async fn hello() -> &'static str {
  "Hello Rust!"
}
