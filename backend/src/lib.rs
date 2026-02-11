//! ToDo Backend Library
//!
//! This module exposes the application's components for testing.

pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

use jsonwebtoken::DecodingKey;
use services::auth_service::AuthService;
use services::todo_service::TodoService;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub todo_service: TodoService,
    pub decoding_key: DecodingKey,
}

/// テスト・統合テスト用：AppStateを構築する
pub fn build_app_state(pool: sqlx::PgPool, config: config::Config) -> AppState {
    let user_repo = repositories::user_repository::UserRepository::new(pool.clone());
    let token_repo = repositories::token_repository::TokenRepository::new(pool.clone());
    let todo_repo = repositories::todo_repository::TodoRepository::new(pool);

    let auth_service = AuthService::new(user_repo, token_repo, config.clone())
        .expect("Failed to init AuthService");
    let todo_service = TodoService::new(todo_repo);

    let public_key_data =
        std::fs::read(&config.jwt_public_key_path).expect("Failed to read public key");
    let decoding_key = DecodingKey::from_rsa_pem(&public_key_data).expect("Failed public key");

    AppState {
        auth_service,
        todo_service,
        decoding_key,
    }
}

/// テスト・統合テスト用：Routerを構築
pub fn build_router(state: AppState) -> axum::Router {
    use axum::routing::get;
    use axum::Router;
    use tower_http::cors::{Any, CorsLayer};

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route(
            "/health",
            get(|| async { axum::Json(serde_json::json!({"status": "ok"})) }),
        )
        .nest("/api/auth", routes::auth_routes(state.clone()))
        .nest("/api/todos", routes::todo_routes(state.clone()))
        .with_state(state)
        .layer(cors)
}
