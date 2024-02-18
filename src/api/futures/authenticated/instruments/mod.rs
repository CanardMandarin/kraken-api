use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Copy, Builder)]
pub struct Instruments {}

impl Instruments {
    pub fn builder() -> InstrumentsBuilder {
        InstrumentsBuilder::default()
    }
}

impl Endpoint for Instruments {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v3/instruments")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginLevel {
    pub contracts: Option<f64>,
    pub initial_margin: f64,
    pub maintenance_margin: f64,
    #[serde(default)]
    pub num_non_contract_units: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub symbol: String,
    pub category: Option<String>,
    pub contract_size: Option<f64>,
    pub contract_value_trade_precision: Option<f64>,
    pub fee_schedule_uid: Option<String>,
    pub funding_rate_coefficient: Option<f64>,
    pub impact_mid_size: Option<f64>,
    pub isin: Option<String>,
    pub last_trading_time: Option<String>,
    #[serde(default)]
    pub margin_levels: Vec<MarginLevel>,
    pub max_position_size: Option<f64>,
    pub max_relative_funding_rate: Option<f64>,
    pub opening_date: Option<String>,
    pub post_only: Option<bool>,
    #[serde(default)]
    pub retail_margin_levels: Vec<MarginLevel>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub tick_size: Option<f64>,
    pub tradeable: bool,
    pub r#type: String,
    pub underlying: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsResp {
    pub result: String,
    pub instruments: Vec<Instrument>,
    pub server_time: String,
}
