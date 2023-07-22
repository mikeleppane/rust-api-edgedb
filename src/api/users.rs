use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{db::EdgeDBUserStore, models::User};

pub async fn get_users(State(store): State<EdgeDBUserStore>) -> impl IntoResponse {
    match store.get_users().await {
        Ok(users) => (StatusCode::OK, users.into()),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()),
    }
}

pub async fn create_user(
    State(store): State<EdgeDBUserStore>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    match store.create_user(user).await {
        Ok(user) => (StatusCode::CREATED, user.into()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}

pub async fn delete_user(
    State(store): State<EdgeDBUserStore>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    match store.delete_user(user).await {
        Ok(user) => (StatusCode::OK, user.to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}
