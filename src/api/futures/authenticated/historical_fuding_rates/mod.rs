use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    endpoint::{Endpoint, EndpointType},
    params::QueryParams,
};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
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

#[derive(Debug, Deserialize, Clone)]
pub enum Side {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]

pub struct FundingRate {
    pub funding_rate: f64,
    pub relative_funding_rate: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFundingRatesResp {
    pub result: String,
    pub rates: Vec<FundingRate>,
    pub server_time: String,
}
