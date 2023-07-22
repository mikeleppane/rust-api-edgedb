mod models;

use std::sync::Arc;

use edgedb_protocol::model::Json;
use models::User;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json as AxumJson, Router,
};
use color_eyre::Result;
use edgedb_tokio::{Builder, Client};
use tokio::sync::RwLock;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

type Db = Arc<RwLock<Client>>;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt().with_target(false).pretty().init();

    let Ok(db) = initialize_db_state().await else {
        panic!("Could not create EdgeDB client!");
    };

    let port = std::env::var("PORT")
        .expect("PORT must be set.")
        .to_string();
    let addr: std::net::SocketAddr = std::net::SocketAddr::from((
        [0, 0, 0, 0],
        port.parse::<u16>()
            .unwrap_or_else(|_| panic!("could not parse the given port {}", port)),
    ));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app(db).into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn initialize_db_state() -> Result<Db> {
    let pool = Client::new(
        &Builder::new()
            .dsn("edgedb://edgedb@db:5656/edgedb?tls_security=insecure")?
            .build_env()
            .await?,
    );
    pool.ensure_connected().await?;
    Ok(Arc::new(RwLock::new(pool)))
}

fn app(db: Db) -> Router {
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

async fn root_handler() -> &'static str {
    "Welcome to try API dev with Rust + EdgeDB"
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[tracing::instrument]
async fn get_users(State(db): State<Db>) -> impl IntoResponse {
    let query = r#"
    select User {
      name,
      email
    }
    "#;
    let res: Result<Json, edgedb_tokio::Error> = db.read().await.query_json(query, &()).await;
    match res {
        Ok(users) => (StatusCode::OK, users.into()),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()),
    }
}

async fn create_user(State(db): State<Db>, AxumJson(user): AxumJson<User>) -> impl IntoResponse {
    tracing::debug!("Creating user: {:?}", &user);
    let query = r#"
    select (insert User {
        name := <str>$0,
        email := <str>$1,
      }) {
        name,
        email
      } filter .email = <str>$1
    "#;
    tracing::debug!("Creating user: {:?}", &user);
    let res: Result<Json, edgedb_tokio::Error> = db
        .read()
        .await
        .query_required_single_json(query, &(user.name, user.email))
        .await;

    match res {
        Ok(user) => (StatusCode::CREATED, user.into()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}

async fn delete_user(State(db): State<Db>, AxumJson(user): AxumJson<User>) -> impl IntoResponse {
    let query = r#"
    select (delete User filter .name = <str>$0 and .email = <str>$1) {
        name,
        email
      };
    "#;
    let res: Result<Json, edgedb_tokio::Error> = db
        .read()
        .await
        .query_required_single_json(query, &(user.name, user.email))
        .await;
    match res {
        Ok(user) => (StatusCode::OK, user.to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}
