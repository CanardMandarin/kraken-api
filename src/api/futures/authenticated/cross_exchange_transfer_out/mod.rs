use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    endpoint::{Endpoint, EndpointType},
    params::QueryParams,
};

#[derive(Debug, Clone, Builder)]
pub struct CrossExchangeTransferOut {
    pub amount: f64,
    pub currency: String,
    pub source_wallet: String,
    
}

impl CrossExchangeTransferOut {
    pub fn builder() -> CrossExchangeTransferOutBuilder {
        CrossExchangeTransferOutBuilder::default()
    }
}

impl Endpoint for CrossExchangeTransferOut {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v4/crossexchangetransfer/out")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }

    fn parameters(&self) -> Option<QueryParams> {
        let mut params = QueryParams::default();
        params.push("amount", self.amount.to_string());
        params.push("currency", self.currency.to_string());
        params.push("sourceWallet", self.source_wallet.to_string());

        Some(params)
    }
}

#[derive(Debug, Deserialize)]
pub struct CrossExchangeTransferOutResp {
    pub result: String,
    pub server_time: String,
}
