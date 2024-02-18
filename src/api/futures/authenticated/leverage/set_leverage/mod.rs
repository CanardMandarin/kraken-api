use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct SetLeveragePreferences {
    pub max_leverage: f64,
    pub symbol: String,
}
impl SetLeveragePreferences {
    pub fn builder() -> SetLeveragePreferencesBuilder {
        SetLeveragePreferencesBuilder::default()
    }
}

impl Endpoint for SetLeveragePreferences {
    fn method(&self) -> Method {
        Method::PUT
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

    fn body(&self) -> Option<(&'static str, Map<String, Value>)> {
        let serialized_params: serde_json::Value =
            serde_json::to_value(&self).expect("Serialization failed");

        match serialized_params {
            serde_json::Value::Object(params) => {
                Some(("application/x-www-form-urlencoded", params))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeveragePreferencesResp {
    pub result: String,
    pub server_time: String,
}
