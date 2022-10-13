use std::collections::HashMap;

use async_trait::async_trait;
use axum::extract::{
    rejection::QueryRejection,
    FromRef, FromRequest,
};
use axum::http::Request;

use crate::Body;

type HashMapRequest = HashMap<String, String>;

#[derive(Debug)]
pub struct Context<InnerState = ()> {
    params_map: HashMapRequest,
    query_map: HashMapRequest,
    req: Request<Body>,
    state: InnerState,
}

impl<InnerState> Context<InnerState> {
    pub fn params(&self, key: &'static str) -> String {
        match self.params_map.get(key) {
            Some(value) => value.clone(),
            None => String::new(),
        }
    }
    pub fn request(&self) -> &Request<Body> {
        &self.req
    }
    pub fn state(&self) -> &InnerState {
        &self.state
    }
    pub fn all_params(&self) -> &HashMapRequest {
        &self.params_map
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
}

#[async_trait]
impl<OuterState, InnerState> FromRequest<OuterState, Body> for Context<InnerState>
where
    OuterState: Send + Sync + 'static,
    InnerState: FromRef<OuterState>,
{
    type Rejection = QueryRejection;

    async fn from_request(
        req: axum::http::Request<Body>,
        state: &OuterState,
    ) -> Result<Self, Self::Rejection> {
        let inner_state = InnerState::from_ref(state);
        Ok(Context {
            req: req,
            state: inner_state,
            params_map: HashMap::new(),
            query_map: HashMap::new(),
        })
    }
}
