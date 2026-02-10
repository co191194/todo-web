use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, Validation, decode};

use crate::{AppState, error::AppError, models::auth::Claims};

pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Authorization ヘッダーを取得
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Auth("Missing authorization header".into()))?;

    // "Bearer " プレフィックスの除去
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Auth("Invalid authorization header format".into()))?;

    // RS256公開鍵でJWTを検証
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true; // 有効期限をチェック
    
    let token_date = decode::<Claims>(token, &state.decoding_key, &validation)
        .map_err(|e| AppError::Auth(format!("Invalid Error: {}", e)))?;
    
    // Claims を Extension に注入
    req.extensions_mut().insert(token_date.claims);
    
    Ok(next.run(req).await)
}
