use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::{Endpoint, EndpointType};

use super::ticker::TickerJson;

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(strip_option, into), default)]
pub struct Tickers {}

impl Tickers {
    pub fn builder() -> TickersBuilder {
        TickersBuilder::default()
    }
}

impl Endpoint for Tickers {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        let mut endpoint = String::from("derivatives/api/v3/tickers/");

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
pub struct TickerResp {
    pub result: String,
    pub tickers: Vec<TickerJson>,
    pub server_time: String,
}
