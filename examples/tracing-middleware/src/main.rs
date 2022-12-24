use graphul::{http::Methods, middleware, Graphul};

pub mod middlewares;
use middlewares::tracing::tracing_middleware;

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();
    tracing_subscriber::fmt().init();

    // router
    app.get("/", || async { "hello world!" });
    app.middleware(middleware::from_fn(tracing_middleware));
    app.run("127.0.0.1:8000").await;
}
