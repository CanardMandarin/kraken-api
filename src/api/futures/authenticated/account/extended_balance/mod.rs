use std::collections::HashMap;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, Builder)]
pub struct ExtendedBalance {}

impl ExtendedBalance {
    pub fn builder() -> ExtendedBalanceBuilder {
        ExtendedBalanceBuilder::default()
    }
}

impl Endpoint for ExtendedBalance {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/0/private/BalanceEx")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Map<String, Value>)> {
        Some(("application/x-www-form-urlencoded", Map::new()))
    }
}

#[derive(Debug, Deserialize)]
pub struct ExtendedBalanceAsset {
    pub balance: String,
    pub credit: Option<String>,
    pub credit_used: Option<String>,
    pub hold_trade: Option<String>,

}

#[derive(Debug, Deserialize)]
pub struct ExtendedBalanceResp(HashMap<String, ExtendedBalanceAsset>);
