use axum::{
    body::Body,
    http::{header, Method, Request},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use todo_backend::config::Config;
use tower::ServiceExt;

/// テスト用のConfigを作成
pub fn test_config() -> Config {
    dotenvy::dotenv().ok();
    Config::from_env().expect("Failed to load config")
}

/// レスポンスボディをJSONに変換するヘルパー
pub async fn response_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

/// POST リクエストを作成するヘルパー
pub fn post_json(uri: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

pub fn authed_request(
    method: Method,
    uri: &str,
    token: &str,
    body: Option<&Value>,
) -> Request<Body> {
    let builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::CONTENT_TYPE, "application/json");

    if let Some(b) = body {
        builder
            .body(Body::from(serde_json::to_string(b).unwrap()))
            .unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    }
}

/// ユーザー登録 -> ログイン -> アクセストークンを返すヘルパー
pub async fn register_and_login(app: axum::Router) -> (axum::Router, String) {
    const TEST_EMAIL: &str = "todo@example.com";

    register_and_login_user(app, TEST_EMAIL).await
}

pub async fn register_and_login_user(app: axum::Router, email: &str) -> (axum::Router, String) {
    const TEST_PASSWORD: &str = "password123";
    let _ = app
        .clone()
        .oneshot(post_json(
            "/api/auth/register",
            &json!({"email": email, "password": TEST_PASSWORD}),
        ))
        .await
        .unwrap();

    let resp = app
        .clone()
        .oneshot(post_json(
            "/api/auth/login",
            &json!({"email": email, "password": TEST_PASSWORD}),
        ))
        .await
        .unwrap();

    let body = response_json(resp.into_body()).await;
    let token = body["access_token"].as_str().unwrap().to_string();
    (app, token)
}
