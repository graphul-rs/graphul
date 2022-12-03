use crate::Body;

pub use axum::middleware::{self, from_fn, from_fn_with_state};

pub type Next = middleware::Next<Body>;
pub use tower_http as tower;
