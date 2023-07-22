use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use super::health::health_check;
use super::root::root_handler;
use super::users::create_user;
use super::users::delete_user;
use super::users::get_users;
use super::users::update_user;
use crate::db;

pub fn build_router(db: db::EdgeDBUserStore) -> Router {
    Router::new()
        .route("/v1", get(root_handler))
        .route("/v1/health_check", get(health_check))
        .route("/v1/users", get(get_users))
        .route("/v1/user", post(create_user))
        .route("/v1/user", put(update_user))
        .route("/v1/user", delete(delete_user))
        .with_state(db)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
