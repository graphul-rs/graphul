pub mod context;
pub mod methods;
pub mod request;
pub mod resource;
pub mod response;

pub use methods::Methods as Methods;

use crate::Body;

pub type Request = axum::http::Request<Body>;

pub type StatusCode = hyper::StatusCode;
