use std::{collections::HashMap};

use async_trait::async_trait;
use axum::extract::{FromRequest, rejection::{QueryRejection, PathRejection}, RequestParts, Query, Path};

use crate::Body;



type HashMapRequest = HashMap<String, String>;

#[derive(Debug)]
pub struct Context {
    params_map: HashMapRequest,
    query_map: HashMapRequest,
}

impl Context {
    pub fn params(&self, key: &'static str) -> String {
        match self.params_map.get(key) {
            Some(value) => value.clone(),
            None => String::new()
        }
    }
    pub fn all_params(&self) -> HashMapRequest {
        self.params_map.clone()
    }
    pub fn query(&self, key: &'static str) -> String {
        match self.query_map.get(key) {
            Some(value) => value.clone(),
            None => String::new()
        }
    }
    pub fn all_query(&self) -> HashMapRequest {
        self.query_map.clone()
    }
}

#[async_trait]
impl FromRequest<Body> for Context {
    type Rejection = QueryRejection;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let mut query_map: HashMapRequest = HashMap::new();
        let mut params_map: HashMapRequest = HashMap::new();
        // get query
        let result_query : Result<Query<HashMapRequest>, QueryRejection> = Query::from_request(req).await;
        match result_query {
            Ok(query) => {
                match query {
                     Query(parse_query) => {
                        query_map = parse_query;
                     }
                }
            },
            Err(_) => {}
        }
        // get params
        let result_params : Result<Path<HashMapRequest>, PathRejection> = Path::from_request(req).await;
        match result_params {
            Ok(params) => {
                match params {
                     Path(parse_params) => {
                        params_map = parse_params;
                     }
                }
            },
            Err(_) => {}
        }

        Ok(Context { params_map: params_map, query_map: query_map })
    }
}
