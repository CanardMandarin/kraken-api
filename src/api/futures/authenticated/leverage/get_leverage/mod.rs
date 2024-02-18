use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Builder)]
pub struct GetLeveragePreferences {}

impl GetLeveragePreferences {
    pub fn builder() -> GetLeveragePreferencesBuilder {
        GetLeveragePreferencesBuilder::default()
    }
}

impl Endpoint for GetLeveragePreferences {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v3/leveragepreferences")
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

pub struct LeveragePreferences {
    pub max_leverage: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeveragePreferencesResp {
    pub result: String,
    #[serde(default)]
    pub leverage_preferences: Vec<LeveragePreferences>,
    pub server_time: String,
}
