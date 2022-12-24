use std::time::Instant;

use graphul::{http::response::Response, middleware::Next, Req};
use tracing::{Instrument, Level};

pub async fn tracing_middleware(request: Req, next: Next) -> Response {
    let span = tracing::span!(
        Level::INFO,
        "Request",
        path = request.uri().to_string(),
        version = ?request.version(),
        method = request.method().to_string()
    );

    async move {
        let now = Instant::now();
        let response = next.run(request).await;
        let duration = now.elapsed();

        tracing::info!(
            status = response.status().to_string(),
            duration = ?duration,
            "Response"
        );
        response
    }
    .instrument(span)
    .await
}
