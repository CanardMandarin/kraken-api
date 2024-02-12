use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    endpoint::{Endpoint, EndpointType},
    params::QueryParams,
};

#[derive(Debug, Clone, Builder)]
pub struct CrossExchangeTransferIn {
    pub amount: f64,
    pub currency: String,
    pub destination_wallet: String,
    
}

impl CrossExchangeTransferIn {
    pub fn builder() -> CrossExchangeTransferInBuilder {
        CrossExchangeTransferInBuilder::default()
    }
}

impl Endpoint for CrossExchangeTransferIn {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v4/crossexchangetransfer/in/destination-wallet")
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
        params.push("destinationWallet", self.destination_wallet.to_string());

        Some(params)
    }
}

#[derive(Debug, Deserialize)]
pub struct CrossExchangeTransferInResp {
    pub result: String,
    pub server_time: String,
}
