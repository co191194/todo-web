use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// Enum

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "todo_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "todo_priority", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TodoPriority {
    Low,
    Medium,
    High,
}

// Entity

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: TodoStatus,
    pub priority: TodoPriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request DTOs

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTodoRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: Option<TodoStatus>,
    pub priority: Option<TodoPriority>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTodoRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: Option<TodoStatus>,
    pub priority: Option<TodoPriority>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTodoStatusRequest {
    pub status: TodoStatus,
}

// Query DTO

#[derive(Debug, Deserialize, ToSchema)]
pub struct TodoQuery {
    pub status: Option<TodoStatus>,
    pub priority: Option<TodoPriority>,
    pub due_before: Option<DateTime<Utc>>,
    pub due_after: Option<DateTime<Utc>>,
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default = "default_order")]
    pub order: String,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_sort() -> String {
    "created_at".to_string()
}
fn default_order() -> String {
    "desc".to_string()
}
fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}

// Response DTO

#[derive(Debug, Serialize, ToSchema)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: TodoStatus,
    pub priority: TodoPriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            due_date: todo.due_date,
            status: todo.status,
            priority: todo.priority,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TodoListResponse {
    pub items:Vec<TodoResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
