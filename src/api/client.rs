use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use serde_json::Value;
use url::Url;

use crate::api::error::ApiError;

use super::endpoint::EndpointType;

/// A trait representing a client which can communicate with the Kraken REST API.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(
        &self,
        endpoint: &str,
        endpoint_type: EndpointType,
    ) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with the Kraken REST API.
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request_builder: RequestBuilder,
        body: serde_json::Map<String, Value>,
        path_to_sign: Option<String>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with the Kraken REST API.
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        mut request_builder: RequestBuilder,
        mut body: serde_json::Map<String, Value>,
        path_to_sign: Option<String>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
