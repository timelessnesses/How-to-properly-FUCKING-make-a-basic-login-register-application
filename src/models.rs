use tokio_pg_mapper_derive;
use serde;

/// User row
#[derive(tokio_pg_mapper_derive::PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub username: String,
    pub hashed_password: String
}

/// User form for both logins and registers
#[derive(serde::Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String
}