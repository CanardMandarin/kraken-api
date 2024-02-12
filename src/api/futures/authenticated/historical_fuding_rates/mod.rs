use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    endpoint::{Endpoint, EndpointType},
    params::QueryParams,
};

#[derive(Debug, Clone, Builder)]
pub struct HistoricalFundingRates {
    pub symbol: String,
}

impl HistoricalFundingRates {
    pub fn builder() -> HistoricalFundingRatesBuilder {
        HistoricalFundingRatesBuilder::default()
    }
}

impl Endpoint for HistoricalFundingRates {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v4/historicalfundingrates")
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

#[derive(Debug, Deserialize)]
pub enum Side {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
}

#[derive(Debug, Deserialize)]
pub struct FundingRate {
    #[serde(rename = "fundingRate")]
    pub funding_rate: f64,
    #[serde(rename = "relativeFundingRate")]
    pub relative_funding_rate: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalFundingRatesResp {
    pub result: String,
    pub rates: Vec<FundingRate>,
    #[serde(rename = "serverTime")]
    pub server_time: String,
}
