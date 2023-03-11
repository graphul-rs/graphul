use std::task::{Context, Poll};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use tower::Service;

#[derive(Debug, Clone)]
pub struct RateLimitLayer {
  counter: Arc<AtomicU64>
}

impl RateLimitLayer {
    pub fn new() -> Self {
        println!("Created new rate limit layer");
        Self {
            counter: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl<Request> Service<Request> for RateLimitLayer {
    type Response = ();
    type Error = ();
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("Hello World");
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request) -> Self::Future {
        println!("Received Request");
        Box::pin(futures::future::ready(Ok(())))
    }
}
