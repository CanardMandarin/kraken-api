use std::collections::HashMap;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::Endpoint;

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
}

#[derive(Debug, Deserialize)]
pub struct BalanceResp {
    pub result: HashMap<String, String>,
}
