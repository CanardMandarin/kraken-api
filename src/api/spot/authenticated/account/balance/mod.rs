use std::collections::HashMap;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::api::{endpoint::{Endpoint, Response}, spot::ApiResponse};

#[derive(Debug, Clone, Copy, Builder)]
pub struct Balance {}

impl Balance {
    pub fn builder() -> BalanceBuilder {
        BalanceBuilder::default()
    }
}

impl Endpoint for Balance {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/0/private/Balance")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Map<String, Value>)> {
        Some(("application/x-www-form-urlencoded", Map::new()))
    }
}

#[derive(Debug, Deserialize)]
pub struct BalanceResp(HashMap<String, String>);

impl Response for BalanceResp {
    fn unwrap(v: Value) -> Self {
        serde_json::from_value::<ApiResponse<Self>>(v.clone()).map(|res| res.result).unwrap()
    }
}