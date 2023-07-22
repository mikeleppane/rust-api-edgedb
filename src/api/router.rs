use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use super::health::health_check;
use super::root::root_handler;
use super::users::create_user;
use super::users::delete_user;
use super::users::get_users;
use crate::db;

pub fn build_router(db: db::EdgeDBUserStore) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/health_check", get(health_check))
        .route("/users", get(get_users))
        .route("/user", post(create_user))
        .route("/user", delete(delete_user))
        .with_state(db)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
