//! Route definitions

use axum::{
    middleware,
    routing::{get, patch, post},
    Router,
};

use crate::{
    handlers::{auth, todo},
    middleware::auth::require_auth,
    AppState,
};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout));

    let protected = Router::new()
        .route("/me", get(auth::me))
        .layer(middleware::from_fn_with_state(state, require_auth));

    public.merge(protected)
}

pub fn todo_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(todo::list).post(todo::create))
        .route(
            "/{id}",
            get(todo::get_by_id).put(todo::update).delete(todo::delete),
        )
        .route("/{id}/status", patch(todo::update_status))
        .layer(middleware::from_fn_with_state(state, require_auth))
}
