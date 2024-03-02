use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Builder)]
pub struct GetPnlPreferences {}

impl GetPnlPreferences {
    pub fn builder() -> GetPnlPreferencesBuilder {
        GetPnlPreferencesBuilder::default()
    }
}

impl Endpoint for GetPnlPreferences {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v3/pnlpreferences")
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
pub struct PnlPreferences {
    pub pnl_currency: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetPnlPreferencesResp {
    #[serde(default)]
    pub preferences: Vec<PnlPreferences>,
    pub result: String,
    pub server_time: String,
}