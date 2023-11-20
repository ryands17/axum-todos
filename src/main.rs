mod todos;

use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router};

use crate::todos::todos_service;

#[tokio::main]
async fn main() -> Result<()> {
  // initialise tracing
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_line_number(true)
    .init();

  let app = Router::new()
    .route("/", get(hello))
    .nest("/todos", todos_service());

  let addr = SocketAddr::from(([0, 0, 0, 0], 3001));

  tracing::info!("App running on http://{}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}

async fn hello() -> &'static str {
  "Hello Rust!"
}