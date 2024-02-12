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
pub struct MarginLevel {
    pub contracts: Option<f64>,
    #[serde(rename = "initialMargin")]
    pub initial_margin: f64,
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: f64,
    #[serde(default, rename = "numNonContractUnits")]
    pub num_non_contract_units: f64,
}

#[derive(Debug, Deserialize)]
pub struct Instrument {
    pub symbol: String,
    pub category: Option<String>,

    #[serde(rename = "contractSize")]
    pub contract_size: Option<f64>,
    #[serde(rename = "contractValueTradePrecision")]
    pub contract_value_trade_precision: Option<f64>,
    #[serde(rename = "feeScheduleUid")]
    pub fee_schedule_uid: Option<String>,

    #[serde(rename = "fundingRateCoefficient")]
    pub funding_rate_coefficient: Option<f64>,

    #[serde(rename = "impactMidSize")]
    pub impact_mid_size: Option<f64>,

    pub isin: Option<String>,

    #[serde(rename = "lastTradingTime")]
    pub last_trading_time: Option<String>,

    #[serde(default, rename = "marginLevels")]
    pub margin_levels: Vec<MarginLevel>,

    #[serde(rename = "maxPositionSize")]
    pub max_position_size: Option<f64>,

    #[serde(rename = "maxRelativeFundingRate")]
    pub max_relative_funding_rate: Option<f64>,

    #[serde(rename = "openingDate")]
    pub opening_date: Option<String>,

    #[serde(rename = "postOnly")]
    pub post_only: Option<bool>,

    #[serde(default, rename = "retailMarginLevels")]
    pub retail_margin_levels: Vec<MarginLevel>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "tickSize")]
    pub tick_size: Option<f64>,
    pub tradeable: bool,
    #[serde(rename = "type")]
    pub instrument_type: String,
    pub underlying: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentsResp {
    pub result: String,
    pub instruments: Vec<Instrument>,
    #[serde(rename = "serverTime")]
    pub server_time: String,
}
