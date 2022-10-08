use super::response::{Response};

pub struct Context {
}

// get context from request, in progressss......
impl Context {
    pub fn new() -> Self {
        Self {
        }
    }
    // part of request
    pub fn args(&self, _name: &str) -> Option<String> {
        None
    }
    pub fn params(&self, _name: &str) -> Option<&str> {
        None
    }
    // part of response
    pub fn send(&self, _body: &'static str) -> Response {
        todo!()
    }
    pub fn json(&self, _body: serde_json::Value) -> Response {
        todo!()
    }
}
