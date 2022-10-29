use crate::Body;

pub use axum::middleware::{self, from_fn};

pub type Next = middleware::Next<Body>;
pub use tower_http as tower;
