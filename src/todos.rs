use axum::{
  extract::{self, Path, State},
  http::StatusCode,
  response::IntoResponse,
  routing::{get, put},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::errors::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Todo {
  id: String,
  text: String,
  done: bool,
}

type Store = Mutex<Vec<Todo>>;
type MainState = State<Arc<Store>>;

pub(crate) fn todos_service() -> Router {
  let initial_todos: Vec<Todo> = vec![
    Todo {
      id: Uuid::new_v4().to_string(),
      text: "Learn React".to_string(),
      done: false,
    },
    Todo {
      id: Uuid::new_v4().to_string(),
      text: "Learn Vim".to_string(),
      done: true,
    },
  ];

  let store = Arc::new(Mutex::new(initial_todos));

  Router::new()
    .route("/", get(get_todos).post(create_todo))
    .route("/:id", put(toggle_todo).delete(delete_todo).post(edit_todo))
    .with_state(store)
}

pub(crate) async fn get_todos(State(store): MainState) -> Json<Vec<Todo>> {
  tracing::info!("fetching todos from in-memory store");

  let todos = store.lock().await.clone();
  Json(todos)
}

pub(crate) async fn toggle_todo(
  Path(id): Path<String>,
  State(store): MainState,
) -> impl IntoResponse {
  let mut todos = store.lock().await;

  tracing::info!("trying to toggle todo: {id}");

  todos
    .iter_mut()
    .find(|todo| todo.id == id)
    .map(|todo| {
      todo.done = !todo.done;
      StatusCode::OK.into_response()
    })
    .unwrap_or(ApiError::TodoNotFound(id).into_response())
}

pub(crate) async fn delete_todo(
  Path(id): Path<String>,
  State(store): MainState,
) -> impl IntoResponse {
  let mut todos = store.lock().await;
  let len = todos.len();

  tracing::info!("trying to delete todo: {id}");

  todos.retain(|todo| todo.id != id);

  if todos.len() != len {
    StatusCode::OK.into_response()
  } else {
    ApiError::TodoNotFound(id).into_response()
  }
}

#[derive(Deserialize)]
pub(crate) struct CreateTodo {
  text: String,
}

pub(crate) async fn create_todo(
  State(store): MainState,
  extract::Json(body): extract::Json<CreateTodo>,
) -> impl IntoResponse {
  let mut todos = store.lock().await;
  tracing::info!("creating todo: {:?}", body.text);

  let new_todo = Todo {
    id: Uuid::new_v4().to_string(),
    text: body.text,
    done: false,
  };

  todos.push(new_todo.clone());
  Json(new_todo).into_response()
}

pub(crate) async fn edit_todo(
  State(store): MainState,
  Path(id): Path<String>,
  extract::Json(body): extract::Json<CreateTodo>,
) -> impl IntoResponse {
  let mut todos = store.lock().await;

  tracing::info!("trying to edit todo: {id}");

  todos
    .iter_mut()
    .find(|todo| todo.id == id)
    .map(|todo| {
      todo.text = body.text;
      Json(todo.clone()).into_response()
    })
    .unwrap_or(ApiError::TodoNotFound(id).into_response())
}
