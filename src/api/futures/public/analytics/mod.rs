use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    endpoint::{Endpoint, EndpointType},
    params::QueryParams,
};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Analytics {
    interval: u64,
    since: i64,
    #[builder(default)]
    to: Option<u64>,
}

impl Analytics {
    pub fn builder() -> AnalyticsBuilder {
        AnalyticsBuilder::default()
    }
}

impl Endpoint for Analytics {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "/api/charts/v1/analytics/liquidity-pool".to_owned()
    }

    fn parameters(&self) -> Option<QueryParams> {
        let mut params = QueryParams::default();
        params.push("interval", self.interval.to_string());
        params.push("since", self.since.to_string());

        if let Some(to) = self.to {
            params.push("to", to.to_string());
        }

        Some(params)
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsRespData {
    pub usd_value: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsRespWrapped {
    pub timestamp: Vec<i64>,
    pub data: AnalyticsRespData,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnalyticsResp {
    pub result: AnalyticsRespWrapped,
}
