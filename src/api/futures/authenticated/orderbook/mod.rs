use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    endpoint::{Endpoint, EndpointType},
    params::QueryParams,
};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct OrderBook {
    pub symbol: String,
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
        String::from("/derivatives/api/v3/orderbook")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }

    fn parameters(&self) -> Option<QueryParams> {
        let mut params = QueryParams::default();
        params.push("symbol", self.symbol.to_string());

        Some(params)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderBookEntry {
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderBookWrapper {
    pub asks: Vec<OrderBookEntry>,
    pub bids: Vec<OrderBookEntry>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResp {
    pub order_book: OrderBookWrapper,
}
