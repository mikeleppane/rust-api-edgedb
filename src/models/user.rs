use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};
#[derive(Debug, Queryable, Deserialize, Serialize, Default)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Queryable, Deserialize, Serialize, Default)]
pub struct Email {
    pub email: String,
}
