use std::collections::HashMap;

use async_trait::async_trait;
pub use axum::http::Request;
use axum::{
    body::{Bytes, HttpBody},
    extract::{rejection::JsonRejection, FromRef, FromRequest, Query, Path},
    BoxError, Json, http::Extensions,
};
use hyper::{HeaderMap, Method, Uri, Version};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::Body;

type HashMapRequest = HashMap<String, String>;

#[derive(Debug)]
pub struct Context<InnerState = ()> {
    params_map: Option<HashMapRequest>,
    query_map: Option<HashMapRequest>,
    bytes: Bytes,
    inner_state: InnerState,
    headers: HeaderMap,
    method: Method,
    uri: Uri,
    version: Version,
}

// update context to get params and query implementar params y query genericos que no solo soporte maps si no tambien otros structs
// Json

impl<InnerState> Context<InnerState> {
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn body(&self) -> String {
        String::from_utf8(self.bytes.to_vec()).expect("")
    }
    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }
    pub fn state(&self) -> &InnerState {
        &self.inner_state
    }
    async fn parse_params(&mut self) {
        todo!()
    }
    pub fn all_params(&self) -> &Option<HashMapRequest> {
        &self.params_map
    }
    pub fn params(&self, _key: &'static str) -> String {
        todo!()
    }
    async fn parse_query(&mut self) {
        // get query
        let query = self.uri.query().unwrap_or_default();
        match serde_urlencoded::from_str(query) {
            Ok(value) => self.query_map = Some(value),
            Err(_) => (),
        };
    }
    pub fn query(&self, key: &'static str) -> String {
        todo!()
    }
    pub fn all_query(&self) -> &Option<HashMapRequest> {
        &self.query_map
    }

    pub async fn payload<T: DeserializeOwned + Default>(&self) -> Result<Json<T>, JsonRejection> {
        // forse parsing
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(self.bytes.clone()));

        let request = match request {
            Ok(value) => value,
            Err(_) => Request::default(),
        };

        Json::from_request(request, &()).await
    }

    pub fn json(&self, payload: Value) -> Json<Value> {
        Json(payload)
    }

    pub fn send(value: &str) -> &str {
        value
    }
}

#[async_trait]
impl<OuterState, InnerState, B> FromRequest<OuterState, B> for Context<InnerState>
where
    OuterState: Send + Sync + 'static,
    InnerState: FromRef<OuterState> + Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = JsonRejection;

    async fn from_request(
        req: axum::http::Request<B>,
        state: &OuterState,
    ) -> Result<Self, Self::Rejection> {
        let inner_state = InnerState::from_ref(state);
        let headers = req.headers().clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let version = req.version();
        let bytes = Bytes::from_request(req, state).await?;
        Ok(Context {
            version,
            headers,
            method,
            uri,
            bytes,
            inner_state,
            params_map: None,
            query_map: None,
        })
    }
}
