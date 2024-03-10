use derive_builder::Builder;
use http::Method;
use serde::Deserialize;
use serde_json::{Map, Value};
use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct WalletTransfer {
    pub amount: f64,
    pub asset: String,
    #[builder(default = "String::from(\"Spot Wallet\")")]
    pub from: String,
    #[builder(default = "String::from(\"Futures Wallet\")")]
    pub to: String,
}

impl WalletTransfer {
    pub fn builder() -> WalletTransferBuilder {
        WalletTransferBuilder::default()
    }
}

impl Endpoint for WalletTransfer {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/0/private/WalletTransfer")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Map<String, Value>)> {
        let mut params = Map::new();

        params.insert("amount".to_string(), Value::String(self.amount.to_string()));
        params.insert("asset".to_string(), Value::String(self.asset.clone()));
        params.insert("from".to_string(), Value::String(self.from.clone()));
        params.insert("to".to_string(), Value::String(self.to.clone()));

        Some(("application/x-www-form-urlencoded", params))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct WalletTransferRef {
    pub refid: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct WalletTransferResp {
    pub result: WalletTransferRef,
}
