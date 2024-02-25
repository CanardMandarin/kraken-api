use std::collections::HashMap;

use crate::api::{endpoint::Endpoint, params::QueryParams};
use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct OrderBook {
    pub pair: String,
    #[builder(default = "100")]
    pub count: u32
}

impl OrderBook {
    pub fn builder() -> OrderBookBuilder {
        OrderBookBuilder::default()
    }
}

impl Endpoint for OrderBook {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "/0/public/Depth".to_owned()
    }

    fn parameters(&self) -> Option<QueryParams> {
        let mut params = QueryParams::default();
        params.push("pair", self.pair.to_string());
        params.push("count", self.count.to_string());

        Some(params)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    pub price: String,
    pub volume: String,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderBookW {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderBookResp {
    pub result: HashMap<String, OrderBookW>,
}
