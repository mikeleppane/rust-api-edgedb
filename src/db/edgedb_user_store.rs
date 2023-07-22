use std::sync::Arc;

use color_eyre::Result;
use edgedb_protocol::model::Json;
use edgedb_tokio::{Builder, Client};
use tokio::sync::RwLock;

use crate::models::User;

type Db = Arc<RwLock<Client>>;

pub type UserResult<T> = Result<T, edgedb_tokio::Error>;

pub const DEFAULT_EDGEDB_DSN: &str = "edgedb://edgedb@db:5656/edgedb?tls_security=insecure";

pub async fn initialize_db() -> Result<Db> {
    let dsn = std::env::var("EDGEDB_DSN").unwrap_or(DEFAULT_EDGEDB_DSN.to_string());
    let pool = Client::new(&Builder::new().dsn(&dsn)?.build_env().await?);
    pool.ensure_connected().await?;
    Ok(Arc::new(RwLock::new(pool)))
}

#[derive(Debug, Clone)]
pub struct EdgeDBUserStore {
    pool: Db,
}

impl EdgeDBUserStore {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }

    pub async fn get_users(&self) -> UserResult<Json> {
        let query = r#"
        select User {
          name,
          email
        }
        "#;
        self.pool.read().await.query_json(query, &()).await
    }

    pub async fn create_user(&self, user: User) -> UserResult<Json> {
        let query = r#"
        select (insert User {
            name := <str>$0,
            email := <str>$1,
        }) {
            name,
            email
        } filter .email = <str>$1
        "#;
        self.pool
            .read()
            .await
            .query_required_single_json(query, &(user.name, user.email))
            .await
    }
    pub async fn delete_user(&self, user: User) -> UserResult<Json> {
        let query = r#"
        select (delete User filter .name = <str>$0 and .email = <str>$1) {
            name,
            email
        };
        "#;
        self.pool
            .read()
            .await
            .query_required_single_json(query, &(user.name, user.email))
            .await
    }
}
