use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    models::{
        auth::Claims,
        todo::{
            CreateTodoRequest, TodoListResponse, TodoQuery, TodoResponse, UpdateTodoRequest,
            UpdateTodoStatusRequest,
        },
    },
    AppState,
};

/// ToDo一覧の取得
#[utoipa::path(
    get,
    path = "/api/todos",
    params(
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("priority" = Option<String>, Query, description = "Filter by priority"),
        ("sort" = Option<String>, Query, description = "Sort field"),
        ("order" = Option<String>, Query, description = "Sort order"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "Todo list", body = TodoListResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "todos"
)]
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<TodoQuery>,
) -> AppResult<impl IntoResponse> {
    let response = state.todo_service.list(claims.sub, query).await?;
    Ok(Json(response))
}

/// ToDoの登録
#[utoipa::path(
    post,
    path = "/api/todos",
    request_body = CreateTodoRequest,
    responses(
        (status = 201, description = "Todo created", body = TodoResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "todos"
)]
pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateTodoRequest>,
) -> AppResult<impl IntoResponse> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let response = state.todo_service.create(claims.sub, req).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// ToDoの詳細を取得
#[utoipa::path(
    get,
    path = "/api/todos/{id}",
    params(("id" = Uuid, Path, description = "Todo ID")),
    responses(
        (status = 200, description = "Todo detail", body = TodoResponse),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "todos"
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let response = state.todo_service.get_by_id(id, claims.sub).await?;
    Ok(Json(response))
}

/// ToDoの更新
#[utoipa::path(
    put,
    path = "/api/todos/{id}",
    params(("id" = Uuid, Path, description = "Todo ID")),
    request_body = UpdateTodoRequest,
    responses(
        (status = 200, description = "Todo updated", body = TodoResponse),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "todos"
)]
pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTodoRequest>,
) -> AppResult<impl IntoResponse> {
    if let Some(ref title) = req.title {
        if title.is_empty() || title.len() > 255 {
            return Err(AppError::Validation(
                "Title must be between 1 and 255 characters".into(),
            ));
        }
    }

    let response = state.todo_service.update(id, claims.sub, req).await?;
    Ok(Json(response))
}

/// ToDoを削除
#[utoipa::path(
    delete,
    path = "/api/todos/{id}",
    params(("id" = Uuid, Path, description = "Todo ID")),
    responses(
        (status = 204, description = "Todo deleted"),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "todos"
)]
pub async fn delete(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    state.todo_service.delete(id, claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// ToDoのステータスを更新
#[utoipa::path(
    patch,
    path = "/api/todos/{id}/status",
    params(("id" = Uuid, Path, description = "Todo ID")),
    request_body = UpdateTodoStatusRequest,
    responses(
        (status = 200, description = "Status updated", body = TodoResponse),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "todos"
)]
pub async fn update_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTodoStatusRequest>,
) -> AppResult<impl IntoResponse> {
    let response = state
        .todo_service
        .update_status(id, claims.sub, req)
        .await?;
    Ok(Json(response))
}
