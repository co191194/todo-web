use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppResult, models::token::RefreshToken};

#[derive(Clone)]
pub struct TokenRepository {
    pool: PgPool,
}

impl TokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            insert into refresh_tokens (user_id, token_hash, expires_at)
            values ($1, $2, $3)
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// トークンハッシュでリフレッシュトークンを検索
    /// -> refresh エンドポイントで使用
    pub async fn find_by_token_hash(&self, token_hash: &str) -> AppResult<Option<RefreshToken>> {
        let token = sqlx::query_as::<_, RefreshToken>(
            r#"
            select 
                * 
            from 
                refresh_tokens 
            where 
                token_hash = $1
                and expires_at > now()
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    /// 特定のリフレッシュトークンを削除（ローテーション時に旧トークンを無効化）
    /// -> refresh / logout エンドポイントで使用
    pub async fn delete_by_token_hash(&self, token_hash: &str) -> AppResult<()> {
        sqlx::query("delete from refresh_tokens where token_hash = $1")
            .bind(token_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// ユーザーの全リフレッシュトークンを削除（全端末ログアウト用、将来の利用を想定）
    pub async fn delete_all_by_user_id(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query("delete from refresh_tokens where user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
