//! Route definitions

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handlers::auth, middleware::auth::require_auth, AppState};

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
