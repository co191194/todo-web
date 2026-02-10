use axum::{Json, Extension, extract::State, response::IntoResponse, http::StatusCode};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::auth::{AuthResponse, Claims, LoginRequest, RegisterRequest, UserResponse};
use crate::AppState;

const REFRESH_TOKEN_KEY: &str = "refresh_token";

/// ユーザー登録
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists"),
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    // バリデーション
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let user = state.auth_service.register(req).await?;
    
    // レスポンス用の構造体
    let body = serde_json::json!({
        "id": user.id,
        "email": user.email,
        "created_at": user.created_at,
    });
    
    Ok((StatusCode::CREATED, Json(body)))
}

/// ログイン
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(req): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    
    let (auth_response, refresh_token) = state.auth_service.login(req).await?;
    
    // リフレッシュトークンをHttpOnly Cookieにセット
    let cookie = Cookie::build((REFRESH_TOKEN_KEY, refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();
    
    Ok((jar.add(cookie), Json(auth_response)))
}

/// トークンリフレッシュ
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    responses(
        (status = 200, description = "Token refreshed", body = AuthResponse),
        (status = 401, description = "Invalid or expired refresh token"),
    ),
    tag = "auth"
)]
pub async fn refresh(
    State(state): State<AppState>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    let refresh_token = jar
        .get(REFRESH_TOKEN_KEY)
        .map(|c| c.value().to_string())
        .ok_or_else(|| AppError::Auth("Missing refresh token".into()))?;
    
    let (auth_response, new_refresh_token) = state.auth_service.refresh(&refresh_token).await?;
    
    // 新しいリフレッシュトークンでCookieを更新（ローテーション）
    let cookie = Cookie::build((REFRESH_TOKEN_KEY, new_refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();
    
    Ok((jar.add(cookie), Json(auth_response)))
}

/// ログアウト
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 204, description = "Logout successful"),
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async  fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    // Cookieからリフレッシュトークンを取得してDBから削除
    if let Some(cookie) = jar.get(REFRESH_TOKEN_KEY) {
        let _ = state.auth_service.logout(cookie.value()).await;
    }
    
    // Cookieを削除
    let cookie = Cookie::build((REFRESH_TOKEN_KEY, ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();
    
    Ok((jar.remove(cookie), StatusCode::NO_CONTENT))
}


/// 現在のユーザー情報取得
#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "Current user info"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn me(Extension(claims): Extension<Claims>) -> AppResult<impl IntoResponse> {
    let body = serde_json::json!({
        "id": claims.sub,
        "email": claims.email,
    });
    
    Ok(Json(body))
}