use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;
use todo_backend::{build_app_state, build_router};
use tower::ServiceExt;

mod helper;
use helper::{post_json, response_json, test_config};

// ////////////////////////////////////////////////////////////
// テストケース
// ////////////////////////////////////////////////////////////

const URI_AUTH_REGISTER: &str = "/api/auth/register";
const URI_AUTH_LOGIN: &str = "/api/auth/login";

const PROP_ACCESS_TOKEN: &str = "accessToken";
const PROP_TOKEN_TYPE: &str = "tokenType";
const PROP_EXPIRES_IN: &str = "expiresIn";

// 正常なメールアドレスとパスワードでユーザー登録が成功することを確認する
#[sqlx::test]
async fn test_register_success(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    let body = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = app
        .oneshot(post_json(URI_AUTH_REGISTER, &body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let json = response_json(response.into_body()).await;
    assert_eq!(json["email"], "test@example.com");
    assert!(json["id"].is_string());
}

// 同じメールアドレスで2回登録すると、2回目が409 Conflictになることを確認する
#[sqlx::test]
async fn test_register_duplicate_email(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    let body = json!({
        "email": "dup@example.com",
        "password": "password123"
    });

    // 1回目は成功
    let response = app
        .clone()
        .oneshot(post_json(URI_AUTH_REGISTER, &body))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // 2回目は失敗
    let response = app
        .clone()
        .oneshot(post_json(URI_AUTH_REGISTER, &body))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

// 不正なメールアドレス形式で登録すると400 Bad Requestになることを確認する
#[sqlx::test]
async fn test_register_invalid_email(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    let body = json!({
        "email": "not-an-email",
        "password": "password123"
    });

    let response = app
        .oneshot(post_json(URI_AUTH_REGISTER, &body))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// パスワードが短すぎる場合に400 Bad Requestになることを確認する
#[sqlx::test]
async fn test_register_short_password(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    let body = json!({
        "email": "test@example.com",
        "password": "short"
    });

    let response = app
        .oneshot(post_json(URI_AUTH_REGISTER, &body))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// 登録済みユーザーで正しい認証情報を使いログインが成功し、アクセストークンとリフレッシュトークン（Cookie）が返却されることを確認する
#[sqlx::test]
async fn test_login_success(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    // 先に登録
    let register_body = json!({
        "email": "login@example.com",
        "password": "password123"
    });
    let _ = app
        .clone()
        .oneshot(post_json(URI_AUTH_REGISTER, &register_body))
        .await
        .unwrap();

    // ログイン
    let login_body = json!({
        "email": "login@example.com",
        "password": "password123"
    });
    let response = app
        .oneshot(post_json(URI_AUTH_LOGIN, &login_body))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Set-Cookie ヘッダーにリフレッシュトークンが含まれることを確認
    let set_cookie = response.headers().get(header::SET_COOKIE);
    assert!(set_cookie.is_some());
    let cookie_str = set_cookie.unwrap().to_str().unwrap();
    assert!(cookie_str.contains("refresh_token="));

    let json = response_json(response.into_body()).await;
    assert!(json[PROP_ACCESS_TOKEN].is_string());
    assert_eq!(json[PROP_TOKEN_TYPE], "Bearer");
    assert_eq!(json[PROP_EXPIRES_IN], 900);
}

// 登録済みユーザーに対して誤ったパスワードでログインすると401 Unauthorizedになることを確認する
#[sqlx::test]
async fn test_login_wrong_password(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    // 登録
    let _ = app
        .clone()
        .oneshot(post_json(
            URI_AUTH_REGISTER,
            &json!({
                "email": "wrong@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    // 間違ったパスワードでログイン
    let response = app
        .oneshot(post_json(
            URI_AUTH_LOGIN,
            &json!({
                "email": "wrong@example.com",
                "password": "wrongpassword"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// 存在しないユーザーでログインすると401 Unauthorizedになることを確認する
#[sqlx::test]
async fn test_login_nonexistent_user(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    let response = app
        .oneshot(post_json(
            URI_AUTH_LOGIN,
            &json!({
                "email": "nonexistent@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// アクセストークンなしで /api/auth/me にアクセスすると401 Unauthorizedになることを確認する
#[sqlx::test]
async fn test_me_without_token(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/auth/me")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// 有効なアクセストークンを付与して /api/auth/me にアクセスすると、ログインユーザーの情報が取得できることを確認する
#[sqlx::test]
async fn test_me_with_valid_token(pool: PgPool) {
    let config = test_config();
    let state = build_app_state(pool, config);
    let app = build_router(state);

    // 登録 → ログインしてトークン取得
    let _ = app
        .clone()
        .oneshot(post_json(
            URI_AUTH_REGISTER,
            &json!({
                "email": "me@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    let login_resp = app
        .clone()
        .oneshot(post_json(
            URI_AUTH_LOGIN,
            &json!({
                "email": "me@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    let login_json = response_json(login_resp.into_body()).await;
    let access_token = login_json[PROP_ACCESS_TOKEN].as_str().unwrap();

    // /me にアクセス
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/auth/me")
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response.into_body()).await;
    assert_eq!(json["email"], "me@example.com");
}
