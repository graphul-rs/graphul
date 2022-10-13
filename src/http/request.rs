use std::collections::HashMap;

use async_trait::async_trait;
use axum::extract::{
    rejection::QueryRejection,
    FromRef, FromRequest,
};
pub use axum::http::Request;

use crate::Body;

type HashMapRequest = HashMap<String, String>;

#[derive(Debug)]
pub struct Context<InnerState = (), B = Body> {
    params_map: HashMapRequest,
    query_map: HashMapRequest,
    req: Request<B>,
    state: InnerState,
}

// update context to get params and query

impl<InnerState, B> Context<InnerState, B> {
    pub fn params(&self, key: &'static str) -> String {
        match self.params_map.get(key) {
            Some(value) => value.clone(),
            None => String::new(),
        }
    }
    pub fn request(&self) -> &Request<B> {
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
impl<OuterState, InnerState, B> FromRequest<OuterState, B> for Context<InnerState, B>
where
    OuterState: Send + Sync + 'static,
    InnerState: FromRef<OuterState>,
    B: Send + 'static
{
    type Rejection = QueryRejection;

    async fn from_request(
        req: axum::http::Request<B>,
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
