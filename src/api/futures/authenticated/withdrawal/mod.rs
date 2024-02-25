use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Builder, Default, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Withdrawal {
    pub amount: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_wallet: Option<String>,
}

impl Withdrawal {
    pub fn builder() -> WithdrawalBuilder {
        WithdrawalBuilder::default()
    }
}

impl Endpoint for Withdrawal {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v3/withdrawal")
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalResp {
    pub result: String,
    pub server_time: String,
}
