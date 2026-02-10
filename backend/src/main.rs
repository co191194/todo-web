use axum::{routing::get, Json, Router};
use jsonwebtoken::DecodingKey;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::models::auth::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use crate::repositories::token_repository::TokenRepository;
use crate::repositories::user_repository::UserRepository;
use crate::services::auth_service::AuthService;
use crate::error::ErrorResponse;

mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub decoding_key: DecodingKey,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::auth::refresh,
        handlers::auth::logout,
        handlers::auth::me,
    ),
    components(schemas(
        RegisterRequest,
        LoginRequest,
        AuthResponse,
        UserResponse,
        ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication API")
    )
)]
struct ApiDoc;

// Bearer認証スキームの定義
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth", 
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    )
                )
            );
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load config
    let config = Config::from_env().expect("Failed to load config");

    // Database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database connected and migrations applied");

    // Build AppState
    let user_repo = UserRepository::new(pool.clone());
    let token_repo = TokenRepository::new(pool.clone());
    let auth_service =
        AuthService::new(user_repo, token_repo, config.clone()).expect("Failed to initialize AuthService");

    // 公開鍵の読み込み（JWTの検証用）
    let public_key_data =
        std::fs::read(&config.jwt_public_key_path).expect("Failed to read public key");
    let decoding_key = DecodingKey::from_rsa_pem(&public_key_data).expect("Invalid public key");
    let state = AppState {
        auth_service,
        decoding_key,
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .nest("/api/auth", routes::auth_routes(state.clone()))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
