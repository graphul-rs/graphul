use std::collections::HashMap;

use async_trait::async_trait;
pub use axum::http::request::Parts;
pub use axum::http::Request;
use axum::{
    body::Bytes,
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        FromRef, FromRequest, FromRequestParts, Path, Query,
    },
    Json,
};
use futures::StreamExt;
use hyper::{HeaderMap, Method, Uri, Version};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::Body;

type HashMapRequest = HashMap<String, String>;

#[derive(Debug)]
pub struct Context<InnerState = ()> {
    params_map: HashMapRequest,
    query_map: HashMapRequest,
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

    pub async fn parse_params<T: DeserializeOwned>(&self) -> Result<Json<T>, JsonRejection> {
        let value = match serde_json::to_string(&self.params_map) {
            Ok(data) => data,
            Err(_) => String::new(),
        };
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(value));

        let request = match request {
            Ok(value) => value,
            Err(_) => Request::default(),
        };

        Json::from_request(request, &()).await
    }
    pub fn all_params(&self) -> &HashMapRequest {
        &self.params_map
    }
    pub fn params(&self, key: &'static str) -> String {
        match self.params_map.get(key) {
            Some(value) => value.clone(),
            None => String::new(),
        }
    }
    pub async fn parse_query<T: DeserializeOwned>(&self) -> Result<Json<T>, JsonRejection> {
        let value = match serde_json::to_string(&self.query_map) {
            Ok(data) => data,
            Err(_) => String::new(),
        };
        let request = Request::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(value));

        let request = match request {
            Ok(value) => value,
            Err(_) => Request::default(),
        };

        Json::from_request(request, &()).await
    }
    pub fn query(&self, key: &'static str) -> String {
        match self.query_map.get(key) {
            Some(value) => value.clone(),
            None => String::new(),
        }
    }
    pub fn all_query(&self) -> &HashMapRequest {
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
impl<OuterState, InnerState> FromRequest<OuterState, Body> for Context<InnerState>
where
    OuterState: Send + Sync + 'static,
    InnerState: FromRef<OuterState> + Send + Sync,
{
    type Rejection = JsonRejection;

    async fn from_request(
        req: axum::http::Request<Body>,
        state: &OuterState,
    ) -> Result<Self, Self::Rejection> {
        let inner_state = InnerState::from_ref(state);
        let headers = req.headers().clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let version = req.version();
        let (parts, body) = &mut req.into_parts();
        let mut params_map = HashMap::new();
        let mut query_map = HashMap::new();
        let result_params: Result<Path<HashMapRequest>, PathRejection> =
            Path::from_request_parts(parts, &()).await;

        if let Ok(params) = result_params {
            match params {
                Path(parse_params) => {
                    params_map = parse_params;
                }
            }
        }

        let result_query: Result<Query<HashMapRequest>, QueryRejection> =
            Query::from_request_parts(parts, &()).await;
        if let Ok(params) = result_query {
            match params {
                Query(parse_params) => {
                    query_map = parse_params;
                }
            }
        }

        let mut bytes = Bytes::new();
        let n = body.map(|x| {
            if let Ok(value) = x {
                bytes = value
            }
        });
        // get value from iter map
        n.collect::<Vec<_>>().await;
        Ok(Context {
            version,
            headers,
            method,
            uri,
            bytes,
            inner_state,
            params_map,
            query_map,
        })
    }
}
