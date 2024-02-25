use std::collections::HashMap;

use crate::api::{endpoint::Endpoint, params::QueryParams};
use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Ticker {
    pub pair: Option<String>,
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
        "/0/public/Ticker".to_owned()
    }

    fn parameters(&self) -> Option<QueryParams> {
        let mut params = QueryParams::default();

        if let Some(pair) = &self.pair {
            params.push("pair", pair.to_string());
        }

        Some(params)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetTicker {
    #[serde(rename = "a")]
    pub ask: Vec<String>, // Ask [<price>, <whole lot volume>, <lot volume>]
    #[serde(rename = "b")]
    pub bid: Vec<String>, // Bid [<price>, <whole lot volume>, <lot volume>]
    #[serde(rename = "c")]
    pub last_trade_closed: Vec<String>, // Last trade closed [<price>, <lot volume>]
    #[serde(rename = "v")]
    pub volume: Vec<String>, // Volume [<today>, <last 24 hours>]
    #[serde(rename = "p")]
    pub volume_weighted_average_price: Vec<String>, // Volume weighted average price [<today>, <last 24 hours>]
    #[serde(rename = "t")]
    pub num_trades: Vec<u64>, // Number of trades [<today>, <last 24 hours>]
    #[serde(rename = "l")]
    pub low: Vec<String>, // Low [<today>, <last 24 hours>]
    #[serde(rename = "h")]
    pub high: Vec<String>, // High [<today>, <last 24 hours>]
    #[serde(rename = "o")]
    pub opening_price: String, // Today's opening price
}

#[derive(Debug, Deserialize, Clone)]
pub struct TickerResp {
    pub result: HashMap<String, AssetTicker>,
}
