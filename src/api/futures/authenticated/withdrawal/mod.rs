use derive_builder::Builder;
use http::Method;
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Withdrawal {
    pub amount: f64,
    pub currency: String,
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
        let mut params = Map::new();

        params.insert("amount".to_string(), Value::String(self.amount.to_string()));
        params.insert("currency".to_string(), Value::String(self.currency.clone()));

        if let Some(source_wallet) = &self.source_wallet {
            params.insert(
                "sourceWallet".to_string(),
                Value::String(source_wallet.clone()),
            );
        }

        Some(("application/x-www-form-urlencoded", params))
    }
}

#[derive(Debug, Deserialize)]
pub struct WithdrawalResp {
    pub result: String,
    #[serde(rename = "serverTime")]
    pub server_time: String,
}
