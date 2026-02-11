use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use reqwest::Method;
use serde_json::json;
use sqlx::PgPool;

mod helper;
use helper::{
    authed_request, register_and_login, register_and_login_user, response_json, test_config,
};
use todo_backend::{build_app_state, build_router};
use tower::ServiceExt;

const BASE_URI: &str = "/api/todos";

const PROP_TITLE: &str = "title";
const PROP_DESCRIPTION: &str = "description";
const PROP_PRIORITY: &str = "priority";
const PROP_STATUS: &str = "status";
const PROP_ID: &str = "id";
const PROP_TOTAL: &str = "total";
const PROP_ITEMS: &str = "items";
const PROP_PAGE: &str = "page";
const PROP_PER_PAGE: &str = "per_page";

#[sqlx::test]
async fn test_create_todo(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    const TITLE: &str = "Buy groceries";
    const DESCRIPTION: &str = "Milk, eggs, bread";
    const PRIORITY: &str = "high";

    let body = json!({
        PROP_TITLE: TITLE,
        PROP_DESCRIPTION: DESCRIPTION,
        PROP_PRIORITY: PRIORITY
    });

    let resp = app
        .oneshot(authed_request(Method::POST, BASE_URI, &token, Some(&body)))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let json = response_json(resp.into_body()).await;
    assert_eq!(json[PROP_TITLE], TITLE);
    assert_eq!(json[PROP_DESCRIPTION], DESCRIPTION);
    assert_eq!(json[PROP_PRIORITY], PRIORITY);
    assert_eq!(json[PROP_STATUS], "pending"); // デフォルト値
    assert!(json[PROP_ID].is_string());
}

#[sqlx::test]
async fn test_create_todo_validation_error(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    // タイトルが空
    let body = json!({PROP_TITLE: ""});
    let resp = app
        .oneshot(authed_request(Method::POST, BASE_URI, &token, Some(&body)))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn test_create_todo_unauthorized(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);

    let body = json!({PROP_TITLE: "TEST"});
    let req = Request::builder()
        .method(Method::POST)
        .uri(BASE_URI)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_list_todos(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    // 3件作成
    for i in 1..=3 {
        let body = json!({PROP_TITLE: format!("Todo {}", i)});
        let _ = app
            .clone()
            .oneshot(authed_request(Method::POST, BASE_URI, &token, Some(&body)))
            .await
            .unwrap();
    }

    // 一覧取得
    let resp = app
        .oneshot(authed_request(Method::GET, BASE_URI, &token, None))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let json = response_json(resp.into_body()).await;
    assert_eq!(json[PROP_TOTAL], 3);
    assert_eq!(json[PROP_ITEMS].as_array().unwrap().len(), 3);
    assert_eq!(json[PROP_PAGE], 1);
    assert_eq!(json[PROP_PER_PAGE], 20);
}

#[sqlx::test]
async fn test_list_todos_with_filter(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    // 異なるステータスで作成
    let _ = app
        .clone()
        .oneshot(authed_request(
            Method::POST,
            BASE_URI,
            &token,
            Some(&json!({PROP_TITLE: "Pending todo", PROP_STATUS: "pending"})),
        ))
        .await
        .unwrap();
    let _ = app
        .clone()
        .oneshot(authed_request(
            Method::POST,
            BASE_URI,
            &token,
            Some(&json!({PROP_TITLE: "Completed todo", PROP_STATUS: "completed"})),
        ))
        .await
        .unwrap();

    // statusのフィルタ
    let resp = app
        .oneshot(authed_request(
            Method::GET,
            &format!("{}?status=pending", BASE_URI),
            &token,
            None,
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let json = response_json(resp.into_body()).await;
    assert_eq!(json[PROP_TOTAL], 1);
    assert_eq!(json[PROP_ITEMS][0][PROP_TITLE], "Pending todo");
}

#[sqlx::test]
async fn test_get_todo_by_id(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    const TITLE: &str = "Detail test";
    // 作成
    let resp = app
        .clone()
        .oneshot(authed_request(
            Method::POST,
            BASE_URI,
            &token,
            Some(&json!({PROP_TITLE: TITLE})),
        ))
        .await
        .unwrap();
    let created = response_json(resp.into_body()).await;
    let id = created[PROP_ID].as_str().unwrap();

    // 詳細を取得
    let resp = app
        .oneshot(authed_request(
            Method::GET,
            &format!("{}/{}", BASE_URI, id),
            &token,
            None,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = response_json(resp.into_body()).await;
    assert_eq!(json[PROP_TITLE], TITLE);
}

#[sqlx::test]
async fn test_get_todo_not_found(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    let fake_id = uuid::Uuid::new_v4();
    let resp = app
        .oneshot(authed_request(
            Method::GET,
            &format!("{}/{}", BASE_URI, fake_id),
            &token,
            None,
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn test_update_todo(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    // 作成
    let resp = app
        .clone()
        .oneshot(authed_request(
            Method::POST,
            BASE_URI,
            &token,
            Some(&json!({PROP_TITLE: "Original"})),
        ))
        .await
        .unwrap();
    let created = response_json(resp.into_body()).await;
    let id = created[PROP_ID].as_str().unwrap();

    // 更新
    const EXPECTED_TITLE: &str = "Updated";
    const EXPECTED_PRIORITY: &str = "high";
    let update_body = json!({
        PROP_TITLE: EXPECTED_TITLE,
        PROP_PRIORITY: EXPECTED_PRIORITY
    });
    let resp = app
        .oneshot(authed_request(
            Method::PUT,
            &format!("{}/{}", BASE_URI, id),
            &token,
            Some(&update_body),
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let json = response_json(resp.into_body()).await;
    assert_eq!(json[PROP_TITLE], EXPECTED_TITLE);
    assert_eq!(json[PROP_PRIORITY], EXPECTED_PRIORITY);
}

#[sqlx::test]
async fn test_update_todo_status(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    // 作成
    let resp = app
        .clone()
        .oneshot(authed_request(
            Method::POST,
            BASE_URI,
            &token,
            Some(&json!({PROP_TITLE: "Status test"})),
        ))
        .await
        .unwrap();
    let created = response_json(resp.into_body()).await;
    let id = created[PROP_ID].as_str().unwrap();

    // ステータスを変更
    const EXPECTED_STATUS: &str = "completed";
    let resp = app
        .oneshot(authed_request(
            Method::PATCH,
            &format!("{}/{}/status", BASE_URI, id),
            &token,
            Some(&json!({PROP_STATUS: EXPECTED_STATUS})),
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let json = response_json(resp.into_body()).await;
    assert_eq!(json[PROP_STATUS], EXPECTED_STATUS);
}

#[sqlx::test]
async fn test_delete_todo(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);
    let (app, token) = register_and_login(app).await;

    // 作成
    let resp = app
        .clone()
        .oneshot(authed_request(
            Method::POST,
            BASE_URI,
            &token,
            Some(&json!({PROP_TITLE: "Delete me"})),
        ))
        .await
        .unwrap();
    let created = response_json(resp.into_body()).await;
    let id = created[PROP_ID].as_str().unwrap();

    // 削除
    let resp = app
        .clone()
        .oneshot(authed_request(
            Method::DELETE,
            &format!("{}/{}", BASE_URI, id),
            &token,
            None,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // 削除後に取得 -> 404
    let resp = app
        .oneshot(authed_request(
            Method::GET,
            &format!("{}/{}", BASE_URI, id),
            &token,
            None,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn test_cannot_access_other_user_todo(pool: PgPool) {
    let state = build_app_state(pool, test_config());
    let app = build_router(state);

    // ユーザーA
    let (app, token_a) = register_and_login_user(app, "userA@example.com").await;

    let resp = app
        .clone()
        .oneshot(authed_request(
            Method::POST,
            BASE_URI,
            &token_a,
            Some(&json!({PROP_TITLE: "User A's todo"})),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let todo_a = response_json(resp.into_body()).await;
    let todo_a_id = todo_a[PROP_ID].as_str().unwrap();

    // ユーザーB
    let (app, token_b) = register_and_login_user(app, "userB@example.com").await;

    // ユーザーBがユーザーAのToDoを取得 -> 404
    let resp = app
        .clone()
        .oneshot(authed_request(
            Method::GET,
            &format!("{}/{}", BASE_URI, todo_a_id),
            &token_b,
            None,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // ユーザーBがユーザーAのToDoを削除 -> 404
    let resp = app
        .oneshot(authed_request(
            Method::DELETE,
            &format!("{}/{}", BASE_URI, todo_a_id),
            &token_b,
            None,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
