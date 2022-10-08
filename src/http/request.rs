use std::collections::HashMap;


pub struct RequestBody {
    pub method: String,
    pub path: String,
    pub args: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
}

impl RequestBody {
    pub fn new() -> Self {
        todo!()
    }
    pub fn args(&self, _name: &str) -> String {
        todo!()
    }
    pub fn params(&self, _name: &str) -> String {
        todo!()
    }
}
