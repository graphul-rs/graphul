pub type Response = axum::response::Response;
pub type Value = serde_json::Value;

pub use axum::response::{Form, Html, Redirect};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
