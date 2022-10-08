use super::request::RequestBody;
use super::response::{Response, ResponseBody};

pub struct Context {
    pub response: ResponseBody,
    pub request: RequestBody,
}

// get context from request, in progressss......
impl Context {
    pub fn new(req: RequestBody, resp: ResponseBody) -> Self {
        Self {
            response: resp,
            request: req,
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
    pub fn send(&self, body: &'static str) -> Response {
        self.response.send(body)
    }
    pub fn json(&self, body: serde_json::Value) -> Response {
        self.response.json(body)
    }
}
