use std::time::Duration;

use graphul::http::StatusCode;
use sqlx::{postgres::{PgPoolOptions}, Error, Pool, Postgres};

pub async fn db_con() -> Result<Pool<Postgres>, Error> {
    let db_uri = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    PgPoolOptions::new()
    .max_connections(5)
    .connect_timeout(Duration::from_secs(3))
    .connect(&db_uri)
    .await
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
