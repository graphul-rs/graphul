use std::collections::HashMap;

use http::response::Response as HttpResponse;
use http::StatusCode;
use hyper::Body;
pub use serde_json::json;

pub type Response = HttpResponse<Body>;
pub type Value = serde_json::Value;

pub struct ResponseBody {
    status: StatusCode,
    headers: HashMap<String, String>,
}

impl ResponseBody {
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
        }
    }
    pub fn status(&mut self, status: StatusCode) -> &Self {
        self.status = status;
        self
    }
    pub fn header(&mut self, name: &str, value: &str) -> &Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }
    pub fn send(&self, body: &'static str) -> Response {
        let mut builder = HttpResponse::builder().status(self.status);
        for (name, value) in self.headers.iter() {
            builder = builder.header(name, value);
        }
        builder.body(Body::from(body)).unwrap()
    }
    pub fn json(&self, body: serde_json::Value) -> Response {
        let mut builder = HttpResponse::builder().status(self.status);
        for (name, value) in self.headers.iter() {
            builder = builder.header(name, value);
        }
        builder = builder.header("Content-Type", "application/json");
        builder.body(Body::from(body.to_string())).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
