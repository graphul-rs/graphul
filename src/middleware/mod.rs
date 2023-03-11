mod rate_limit_layer;

use crate::Body;


pub use axum::middleware::{self, from_fn, from_fn_with_state};

pub type Next = middleware::Next<Body>;
pub use tower_http as tower;

pub mod limit {
    use std::time::Duration;

    pub use ::tower::limit::{ConcurrencyLimit, RateLimit};
    use ::tower::limit::{
        ConcurrencyLimitLayer as OriginalConcurrencyLimitLayer,
        GlobalConcurrencyLimitLayer as OriginalGlobalConcurrencyLimitLayer,
    };
    use tower_http::add_extension::{AddExtension, AddExtensionLayer};

    use super::rate_limit_layer::RateLimitLayer as InHouseRateLimitLayer;

    /// Wrapper of RateLimitLayer of tower
    /// Enforces a rate limit on the number of requests the underlying
    /// service can handle over a period of time.
    #[derive(Debug, Clone)]
    pub struct RateLimitLayer(AddExtensionLayer<InHouseRateLimitLayer>);
    /// Wrapper of ConcurrencyLimitLayer of tower
    /// Enforces a limit on the concurrent number of requests the underlying
    /// service can handle.
    #[derive(Debug, Clone)]
    pub struct ConcurrencyLimitLayer(AddExtensionLayer<OriginalConcurrencyLimitLayer>);
    /// Wrapper of GlobalConcurrencyLimitLayer of tower
    /// Enforces a limit on the concurrent number of requests the underlying
    /// service can handle.
    ///
    /// Unlike [`ConcurrencyLimitLayer`], which enforces a per-service concurrency
    /// limit, this layer accepts a owned semaphore (`Arc<Semaphore>`) which can be
    /// shared across multiple services.
    ///
    /// Cloning this layer will not create a new semaphore.
    #[derive(Debug, Clone)]
    pub struct GlobalConcurrencyLimitLayer(AddExtensionLayer<OriginalGlobalConcurrencyLimitLayer>);

    impl RateLimitLayer {
        /// Create new rate limit layer.
        pub fn new(limit: u64, per: Duration) -> Self {
            Self(AddExtensionLayer::new(InHouseRateLimitLayer::new()))
        }
    }

    impl<S> tower_layer::Layer<S> for RateLimitLayer {
        type Service = AddExtension<S, InHouseRateLimitLayer>;
        fn layer(&self, service: S) -> Self::Service {
            self.0.layer(service)
        }
    }

    impl ConcurrencyLimitLayer {
        /// Create a new concurrency limit layer.
        pub fn new(max: usize) -> Self {
            Self(AddExtensionLayer::new(OriginalConcurrencyLimitLayer::new(
                max,
            )))
        }
    }

    impl<S> tower_layer::Layer<S> for ConcurrencyLimitLayer {
        type Service = AddExtension<S, OriginalConcurrencyLimitLayer>;
        fn layer(&self, service: S) -> Self::Service {
            self.0.layer(service)
        }
    }

    impl GlobalConcurrencyLimitLayer {
        /// Create a new `GlobalConcurrencyLimitLayer`.
        pub fn new(max: usize) -> Self {
            Self(AddExtensionLayer::new(
                OriginalGlobalConcurrencyLimitLayer::new(max),
            ))
        }
    }

    impl<S> tower_layer::Layer<S> for GlobalConcurrencyLimitLayer {
        type Service = AddExtension<S, OriginalGlobalConcurrencyLimitLayer>;
        fn layer(&self, service: S) -> Self::Service {
            self.0.layer(service)
        }
    }
}
