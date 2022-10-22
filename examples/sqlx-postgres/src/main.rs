mod db;

use graphul::{
    http::{Methods, StatusCode},
    Context, Graphul,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let pool = db::db_con().await.expect("can connect to database");

    // build our application
    let mut app = Graphul::share_state(pool);

    app.get("/", using_connection_pool);

    app.run("127.0.0.1:3000").await;
}

// we can extract the connection pool with `State` or `Context`
async fn using_connection_pool(c: Context<PgPool>) -> Result<String, (StatusCode, String)> {
    let pool = c.state();
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(pool)
        .await
        .map_err(db::internal_error)
}
