use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        todo::{Todo, TodoQuery},
    },
};

#[derive(Clone)]
pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// ToDo作成
    pub async fn create(
        &self,
        user_id: Uuid,
        title: &str,
        description: Option<&str>,
        due_date: Option<chrono::DateTime<chrono::Utc>>,
        status: &crate::models::todo::TodoStatus,
        priority: &crate::models::todo::TodoPriority,
    ) -> AppResult<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            insert into todos (user_id, title, description, due_date, status, priority)
            values ($1, $2, $3, $4, $5, $6)
            returning *
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(description)
        .bind(due_date)
        .bind(status)
        .bind(priority)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    /// ID + ユーザーIDで取得
    /// 認可チェックも行う
    pub async fn find_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> AppResult<Option<Todo>> {
        let todo = sqlx::query_as::<_, Todo>("select * from todos where id = $1 and user_id = $2")
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(todo)
    }

    /// フィルタ・ソート・ページネーション付き一覧取得
    pub async fn find_by_user_id(
        &self,
        user_id: Uuid,
        query: &TodoQuery,
    ) -> AppResult<(Vec<Todo>, i64)> {
        // where句の構築
        let mut where_clauses: Vec<String> = vec!["user_id = $1".to_string()];
        let mut param_index = 2u32;

        if query.status.is_some() {
            where_clauses.push(format!("status = ${}", param_index));
            param_index += 1;
        }
        if query.priority.is_some() {
            where_clauses.push(format!("priority = ${}", param_index));
            param_index += 1;
        }
        if query.due_before.is_some() {
            where_clauses.push(format!("due_date <= ${}", param_index));
            param_index += 1;
        }
        if query.due_after.is_some() {
            where_clauses.push(format!("due_date >= ${}", param_index));
            param_index += 1;
        }

        let where_clause = where_clauses.join(" and ");

        // ソート
        let sort_column = match query.sort.as_str() {
            "due_date" => "due_date",
            "priority" => "priority",
            _ => "created_at",
        };
        let sort_order = match query.order.as_str() {
            "asc" => "asc",
            _ => "desc",
        };

        // ページネーション
        let per_page = query.per_page.clamp(1, 100);
        let offset = (query.page.max(1) - 1) * per_page;

        // count クエリ
        let count_sql = format!("select count(*) from todos where {}", where_clause);
        let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql).bind(user_id);

        if let Some(ref status) = query.status {
            count_query = count_query.bind(status);
        }
        if let Some(ref priority) = query.priority {
            count_query = count_query.bind(priority);
        }
        if let Some(ref due_before) = query.due_before {
            count_query = count_query.bind(due_before);
        }
        if let Some(ref due_after) = query.due_after {
            count_query = count_query.bind(due_after);
        }

        let total = count_query.fetch_one(&self.pool).await?;

        // データ取得クエリ
        let data_sql = format!(
            "select * from todos where {} order by {} {} limit {} offset {}",
            where_clause, sort_column, sort_order, per_page, offset
        );
        let mut data_query = sqlx::query_as::<_, Todo>(&data_sql).bind(user_id);

        if let Some(ref status) = query.status {
            data_query = data_query.bind(status);
        }
        if let Some(ref priority) = query.priority {
            data_query = data_query.bind(priority);
        }
        if let Some(ref due_before) = query.due_before {
            data_query = data_query.bind(due_before);
        }
        if let Some(ref due_after) = query.due_after {
            data_query = data_query.bind(due_after);
        }

        let todos = data_query.fetch_all(&self.pool).await?;

        Ok((todos, total))
    }

    /// ToDoの更新
    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        title: Option<&str>,
        description: Option<&str>,
        due_date: Option<chrono::DateTime<chrono::Utc>>,
        status: Option<&crate::models::todo::TodoStatus>,
        priority: Option<&crate::models::todo::TodoPriority>,
    ) -> AppResult<Option<Todo>> {
        // set句を動的に構築
        let mut set_clauses: Vec<String> = Vec::new();
        let mut param_index = 3u32; // $1 = id, $2 = user_id

        if title.is_some() {
            set_clauses.push(format!("title = ${}", param_index));
            param_index += 1;
        }
        if description.is_some() {
            set_clauses.push(format!("description = ${}", param_index));
            param_index += 1;
        }
        if due_date.is_some() {
            set_clauses.push(format!("due_date = ${}", param_index));
            param_index += 1;
        }
        if status.is_some() {
            set_clauses.push(format!("status = ${}", param_index));
            param_index += 1;
        }
        if priority.is_some() {
            set_clauses.push(format!("priority = ${}", param_index));
            param_index += 1;
        }

        if set_clauses.is_empty() {
            // 更新するフィールドがない場合は現在の値を返す
            return self.find_by_id_and_user_id(id, user_id).await;
        }

        set_clauses.push("updated_at = now()".to_string());

        let sql = format!(
            "update todos set {} where id = $1 and user_id = $2 returning *",
            set_clauses.join(", ")
        );

        let mut query = sqlx::query_as::<_, Todo>(&sql).bind(id).bind(user_id);

        if let Some(title) = title {
            query = query.bind(title);
        }
        if let Some(description) = description {
            query = query.bind(description);
        }
        if let Some(due_date) = due_date {
            query = query.bind(due_date);
        }
        if let Some(status) = status {
            query = query.bind(status);
        }
        if let Some(priority) = priority {
            query = query.bind(priority);
        }
        
        let todo = query.fetch_optional(&self.pool).await?;
        Ok(todo)
    }

    /// ステータス更新
    pub async fn update_status(
        &self,
        id: Uuid,
        user_id: Uuid,
        status: &crate::models::todo::TodoStatus,
    ) -> AppResult<Option<Todo>> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            update todos 
            set status = $3, updated_at = now() 
            where id = $1 and user_id = $2 
            returning *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(status)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(todo)
    }
    
    /// Todoの削除
    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query(
            "delete from todos where id = $1 and user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
}
