//! Example of application using <https://github.com/launchbadge/sqlx>
//!
//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-sqlx-postgres
//! ```
//!
//! Test with curl:
//!
//! ```not_rust
//! curl 127.0.0.1:3000
//! curl -X POST 127.0.0.1:3000
//! ```

use graphul::{
    Context,
    http::{Methods, StatusCode},
    Graphul,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_uri = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect_timeout(Duration::from_secs(3))
    .connect(&db_uri)
    .await
    .expect("can connect to database");

    // build our application
    let mut app = Graphul::share_state(pool);

    app.get("/", using_connection_pool_extractor);

    app.run("127.0.0.1:3000").await;
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(c: Context<PgPool>,) -> Result<String, (StatusCode, String)> {
    let pool = c.state();
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&*pool)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
