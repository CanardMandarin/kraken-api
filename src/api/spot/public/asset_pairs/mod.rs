use std::collections::HashMap;

use crate::api::{endpoint::Endpoint, params::QueryParams};
use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub enum AssetPairsInfo {
    #[default]
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "leverage")]
    Leverage,
    #[serde(rename = "fees")]
    Fees,
    #[serde(rename = "margin")]
    Margin,
}

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(strip_option, into), default)]
pub struct AssetPairs {
    pub pair: Option<String>,
    pub info: Option<AssetPairsInfo>,
}

impl AssetPairs {
    pub fn builder() -> AssetPairsBuilder {
        AssetPairsBuilder::default()
    }
}

impl Endpoint for AssetPairs {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "/0/public/AssetPairs".to_owned()
    }

    fn parameters(&self) -> Option<QueryParams> {
        let mut params = QueryParams::default();

        if let Some(pair) = &self.pair {
            params.push("pair", pair.to_string());
        }

        if let Some(info) = &self.info {
            params.push("info", serde_json::to_string(info).unwrap());
        }

        Some(params)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeeRange {
    #[serde(rename = "0")]
    pub volume: u64,
    #[serde(rename = "1")]
    pub fee: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TradingPairStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "cancel_only")]
    CancelOnly,
    #[serde(rename = "post_only")]
    PostOnly,
    #[serde(rename = "limit_only")]
    LimitOnly,
    #[serde(rename = "reduce_only")]
    ReduceOnly,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradingPair {
    pub altname: String,
    pub wsname: Option<String>,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub lot: Option<String>,
    pub cost_decimals: u8,
    pub pair_decimals: u8,
    pub lot_decimals: u8,
    pub lot_multiplier: u8,
    pub leverage_buy: Vec<u32>,
    pub leverage_sell: Vec<u32>,
    pub fees: Vec<FeeRange>,
    pub fees_maker: Vec<FeeRange>,
    pub fee_volume_currency: String,
    pub margin_call: u32,
    pub margin_stop: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub ordermin: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub costmin: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub tick_size: f64,
    pub status: TradingPairStatus,
    pub long_position_limit: Option<u32>,
    pub short_position_limit: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetPairsResp {
    pub result: HashMap<String, TradingPair>,
}
