use axum::response::IntoResponse;

pub const API_VERSION: &str = "v0.0.1";

pub async fn root_handler() -> impl IntoResponse {
    format!(
        "Welcome to try API dev with Rust + EdgeDB. You are using version: {}",
        API_VERSION
    )
}
