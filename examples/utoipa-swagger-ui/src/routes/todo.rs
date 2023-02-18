use std::sync::Arc;

use crate::swagger::{Todo, TodoError};
use graphul::{
    extract::{Json, Path, Query},
    Context, ContextPart, IntoResponse,
};
use hyper::{HeaderMap, StatusCode};
use serde::Deserialize;
use tokio::sync::Mutex;
use utoipa::IntoParams;

/// In-memory todo store
pub type Store = Mutex<Vec<Todo>>;

/// List all Todo items
///
/// List all Todo items from in-memory storage.
#[utoipa::path(
    get,
    path = "/todo",
    responses(
        (status = 200, description = "List all todos successfully", body = [Todo])
    )
)]
pub async fn list_todos(ctx: Context<Arc<Store>>) -> Json<Vec<Todo>> {
    let todos = ctx.state().lock().await.clone();

    Json(todos)
}

/// Todo search query
#[derive(Deserialize, IntoParams)]
pub struct TodoSearchQuery {
    /// Search by value. Search is incase sensitive.
    value: String,
    /// Search by `done` status.
    done: bool,
}

/// Search Todos by query params.
///
/// Search `Todo`s by query params and return matching `Todo`s.
#[utoipa::path(
    get,
    path = "/todo/search",
    params(
        TodoSearchQuery
    ),
    responses(
        (status = 200, description = "List matching todos by query", body = [Todo])
    )
)]
pub async fn search_todos(
    query: Query<TodoSearchQuery>,
    ctx: Context<Arc<Store>>,
) -> Json<Vec<Todo>> {
    Json(
        ctx.state()
            .lock()
            .await
            .iter()
            .filter(|todo| {
                todo.value.to_lowercase() == query.value.to_lowercase() && todo.done == query.done
            })
            .cloned()
            .collect(),
    )
}

/// Create new Todo
///
/// Tries to create a new Todo item to in-memory storage or fails with 409 conflict if already exists.
#[utoipa::path(
    post,
    path = "/todo",
    request_body = Todo,
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
        (status = 409, description = "Todo already exists", body = TodoError)
    )
)]
pub async fn create_todo(
    ctx: ContextPart<Arc<Store>>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    let mut todos = ctx.state().lock().await;

    todos
        .iter_mut()
        .find(|existing_todo| existing_todo.id == todo.id)
        .map(|found| {
            (
                StatusCode::CONFLICT,
                Json(TodoError::Conflict(format!(
                    "todo already exists: {}",
                    found.id
                ))),
            )
                .into_response()
        })
        .unwrap_or_else(|| {
            todos.push(todo.clone());

            (StatusCode::CREATED, Json(todo)).into_response()
        })
}

/// Mark Todo item done by id
///
/// Mark Todo item done by given id. Return only status 200 on success or 404 if Todo is not found.
#[utoipa::path(
    put,
    path = "/todo/{id}",
    responses(
        (status = 200, description = "Todo marked done successfully"),
        (status = 404, description = "Todo not found")
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        (), // <-- make optional authentication
        ("api_key" = [])
    )
)]
pub async fn mark_done(
    Path(id): Path<i32>,
    headers: HeaderMap,
    ctx: Context<Arc<Store>>,
) -> StatusCode {
    match check_api_key(false, headers) {
        Ok(_) => (),
        Err(_) => return StatusCode::UNAUTHORIZED,
    }

    let mut todos = ctx.state().lock().await;

    todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .map(|todo| {
            todo.done = true;
            StatusCode::OK
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Delete Todo item by id
///
/// Delete Todo item from in-memory storage by id. Returns either 200 success of 404 with TodoError if Todo is not found.
#[utoipa::path(
    delete,
    path = "/todo/{id}",
    responses(
        (status = 200, description = "Todo marked done successfully"),
        (status = 401, description = "Unauthorized to delete Todo", body = TodoError, example = json!(TodoError::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Todo not found", body = TodoError, example = json!(TodoError::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn delete_todo(
    Path(id): Path<i32>,
    headers: HeaderMap,
    ctx: Context<Arc<Store>>,
) -> impl IntoResponse {
    match check_api_key(true, headers) {
        Ok(_) => (),
        Err(error) => return error.into_response(),
    }

    let mut todos = ctx.state().lock().await;

    let len = todos.len();

    todos.retain(|todo| todo.id != id);

    if todos.len() != len {
        StatusCode::OK.into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(TodoError::NotFound(format!("id = {id}"))),
        )
            .into_response()
    }
}

// normally you should create a middleware for this but this is sufficient for sake of example.
fn check_api_key(require_api_key: bool, headers: HeaderMap) -> Result<(), impl IntoResponse> {
    match headers.get("todo_apikey") {
        Some(header) if header != "utoipa-rocks" => Err((
            StatusCode::UNAUTHORIZED,
            Json(TodoError::Unauthorized(String::from("incorrect api key"))),
        )
            .into_response()),
        None if require_api_key => Err((
            StatusCode::UNAUTHORIZED,
            Json(TodoError::Unauthorized(String::from("missing api key"))),
        )
            .into_response()),
        _ => Ok(()),
    }
}
