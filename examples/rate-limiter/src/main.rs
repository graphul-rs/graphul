use std::time::Duration;

use graphul::{http::Methods, Graphul, middleware::limit::RateLimitLayer};

#[tokio::main]
async fn main() {
  let mut app = Graphul::new();

  app.get("/", || async {
    "hello world!"
  });
  // 1000 requests per 10 seconds max
  app.middleware(RateLimitLayer::new(1000, Duration::from_secs(100)));

  app.run("127.0.0.1:8000").await;
}