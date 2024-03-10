use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Ticker {
    pub symbol: String,
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
        endpoint.push_str(&self.symbol);
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
    pub ask: Option<f64>,
    pub ask_size: Option<f64>,
    pub bid: Option<f64>,
    pub bid_size: Option<f64>,
    pub change24h: Option<f64>,
    pub funding_rate: Option<f64>,
    pub funding_rate_prediction: Option<f64>,
    pub index_price: Option<f64>,
    pub last: Option<f64>,
    pub last_size: Option<f64>,
    pub last_time: Option<String>,
    pub mark_price: Option<f64>,
    pub open24h: Option<f64>,
    pub open_interest: Option<f64>,
    pub pair: Option<String>,
    pub post_only: Option<bool>,
    pub suspended: Option<bool>,
    pub symbol: String,
    pub tag: Option<String>,
    pub vol24h: Option<f64>,
    pub volume_quote: Option<f64>,
}


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerResp {
    pub result: String,
    pub ticker: TickerJson,
    pub server_time: String,
}
