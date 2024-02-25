use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Copy, Builder)]
pub struct OpenPositions {}

impl OpenPositions {
    pub fn builder() -> OpenPositionsBuilder {
        OpenPositionsBuilder::default()
    }
}

impl Endpoint for OpenPositions {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v3/openpositions")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum Side {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]

pub struct OpenPosition {
    pub fill_time: String,
    pub price: f64,
    pub side: Side,
    pub size: f64,
    pub symbol: String,
    pub unrealized_funding: Option<f64>,
    pub max_fixed_leverage: Option<f64>,
    pub pnl_currency: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]

pub struct OpenPositionsResp {
    pub result: String,
    pub open_positions: Vec<OpenPosition>,
    pub server_time: String,
}
