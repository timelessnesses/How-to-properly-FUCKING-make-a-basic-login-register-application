use actix_web;
use bcrypt;
use deadpool_postgres;
use derive_more;
use json;
use tokio_pg_mapper;
use tokio_postgres;

/// Errors enum
#[derive(derive_more::Display, derive_more::From, Debug)]
pub enum Errors {
    /// Pool errors
    PoolError(deadpool_postgres::PoolError),
    /// Row mapping errors
    PostgresMapperError(tokio_pg_mapper::Error),
    /// PostgreSQL errors
    PostgresError(tokio_postgres::Error),
    /// Bcrypt Errors
    BCryptError(bcrypt::BcryptError),
    /// User not found in database
    UserNotFound,
    /// Invalid password
    InvalidPassword,
}

impl std::error::Error for Errors {}

impl actix_web::ResponseError for Errors {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            Errors::UserNotFound | Errors::InvalidPassword => actix_web::HttpResponse::NotFound()
                .body(json::stringify_pretty(
                    json::object! {
                        error: "user not found or invalid password"
                    },
                    4,
                )),
            _ => actix_web::HttpResponse::InternalServerError().finish(),
        }
    }
}
