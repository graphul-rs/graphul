use std::time::Duration;

use graphul::{http::Methods, Graphul, middleware::limit::RateLimitLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

  let mut app = Graphul::new();

  app.get("/noop", || async {
    "hello world!2"
  });
  // 1000 requests per 10 seconds max
  app.middleware(RateLimitLayer::new(2, Duration::from_secs(100)));

  app.get("/", || async {
    "hello world!"
  });

  app.run("127.0.0.1:8000").await;
}