use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(strip_option, into), default)]
pub struct Ticker {
    pub symbol: Option<String>,
}

impl Ticker {
    pub fn builder() -> TickerBuilder {
        TickerBuilder::default()
    }
}

impl Endpoint for Ticker {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        let mut endpoint = String::from("derivatives/api/v3/tickers/");

        if let Some(symbol) = &self.symbol {
            endpoint.push_str(&symbol);
        }

        endpoint
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerJson {
    pub ask: f64,
    pub ask_size: f64,
    pub bid: f64,
    pub bid_size: f64,
    pub change24h: f64,
    pub funding_rate: f64,
    pub funding_rate_prediction: f64,
    pub index_price: f64,
    pub last: f64,
    pub last_size: f64,
    pub last_time: String,
    pub mark_price: f64,
    pub open24h: f64,
    pub open_interest: f64,
    pub pair: String,
    pub post_only: bool,
    pub suspended: bool,
    pub symbol: String,
    pub tag: String,
    pub vol24h: f64,
    pub volume_quote: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerResp {
    pub result: String,
    pub ticker: TickerJson,
    pub server_time: String,
}
