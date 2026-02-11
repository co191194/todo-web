use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::todo::{
        CreateTodoRequest, TodoListResponse, TodoPriority, TodoQuery, TodoResponse, TodoStatus,
        UpdateTodoRequest, UpdateTodoStatusRequest,
    },
    repositories::todo_repository::TodoRepository,
};

#[derive(Clone)]
pub struct TodoService {
    todo_repo: TodoRepository,
}

impl TodoService {
    pub fn new(todo_repo: TodoRepository) -> Self {
        Self { todo_repo }
    }

    /// ToDoの作成
    pub async fn create(&self, user_id: Uuid, req: CreateTodoRequest) -> AppResult<TodoResponse> {
        let status = req.status.unwrap_or(TodoStatus::Pending);
        let priority = req.priority.unwrap_or(TodoPriority::Medium);

        let todo = self
            .todo_repo
            .create(
                user_id,
                &req.title,
                req.description.as_deref(),
                req.due_date,
                &status,
                &priority,
            )
            .await?;

        Ok(todo.into())
    }

    /// ToDo一覧を取得
    pub async fn list(&self, user_id: Uuid, query: TodoQuery) -> AppResult<TodoListResponse> {
        let per_page = query.per_page.clamp(1, 100);
        let page = query.page.max(1);

        let (todos, total) = self.todo_repo.find_by_user_id(user_id, &query).await?;

        Ok(TodoListResponse {
            items: todos.into_iter().map(|t| t.into()).collect(),
            total,
            page,
            per_page,
        })
    }

    /// ToDo詳細を取得
    /// 認可チェックも実施
    pub async fn get_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<TodoResponse> {
        let todo = self
            .todo_repo
            .find_by_id_and_user_id(id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".into()))?;

        Ok(todo.into())
    }

    /// ToDoを更新
    /// 認可チェックも実施
    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        req: UpdateTodoRequest,
    ) -> AppResult<TodoResponse> {
        let todo = self
            .todo_repo
            .update(
                id,
                user_id,
                req.title.as_deref(),
                req.description.as_deref(),
                req.due_date,
                req.status.as_ref(),
                req.priority.as_ref(),
            )
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".into()))?;
        
        Ok(todo.into())
    }
    
    /// ステータスの更新
    pub async fn update_status(
        &self,
        id: Uuid,
        user_id: Uuid,
        req: UpdateTodoStatusRequest,
    ) -> AppResult<TodoResponse> {
        let todo = self
            .todo_repo
            .update_status(id, user_id, &req.status)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".into()))?;
        
        Ok(todo.into())
    }
    
    /// ToDoの削除
    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let deleted = self.todo_repo.delete(id, user_id).await?;
        if !deleted {
            return Err(AppError::NotFound("Todo not found".into()));
        }
        Ok(())
    }
}
