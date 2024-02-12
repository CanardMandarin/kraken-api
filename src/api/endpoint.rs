use std::any;

use async_trait::async_trait;
use http::{header, Method, Request};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::api::{
    client::{AsyncClient, Client},
    error::{ApiError, ServerError},
    params::QueryParams,
    query::{url_to_http_uri, AsyncQuery, Query},
};

pub enum EndpointType {
    Spot,
    Futures,
}

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;

    /// The path to the endpoint.
    fn endpoint(&self) -> String;

    /// Whether this endpoint needs authorization.
    fn is_authenticated(&self) -> bool {
        false
    }

    /// Whether this endpoint needs authorization.
    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Spot
    }

    /// Query parameters for the endpoint.
    fn parameters(&self) -> Option<QueryParams> {
        None
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    fn body(&self) -> Option<(&'static str, Map<String, Value>)> {
        None
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let is_authenicated = self.is_authenticated();
        let endpoint = self.endpoint();
        let endpoint_type = self.endpoint_type();

        // Build the URL.
        let mut url = client.rest_endpoint(&endpoint, &endpoint_type)?;

        // Add query parameters to the URL.
        if let Some(parameters) = self.parameters() {
            parameters.add_to_url(&mut url);
        }

        let request_builder = Request::builder()
            .method(self.method())
            .uri(url_to_http_uri(url));

        // Add the body to the request if any.
        let (request_builder, body) = if let Some((mime, data)) = self.body() {
            (request_builder.header(header::CONTENT_TYPE, mime), data)
        } else {
            (request_builder, Map::new())
        };

        // Send off the request
        let rsp = client.rest(
            request_builder,
            body,
            is_authenicated.then_some(endpoint),
            &endpoint_type,
        )?;

        // Check the response status and extract errors if needed.
        let status = rsp.status();

        let v = serde_json::from_slice(rsp.body()).map_err(|_e| ApiError::ServerError {
            status,
            source: ServerError::InvalidJson {
                data: rsp.body().into_iter().copied().collect(),
            },
        })?;

        if !status.is_success() {
            return Err(ApiError::ServerError {
                status,
                source: ServerError::NotSuccess { obj: v },
            });
        }

        // Deserialize into whatever type the caller is asking.
        serde_json::from_value::<T>(v.clone())
            .map(|response| response)
            .map_err(|e| ApiError::DataType {
                typename: any::type_name::<T>(),
                obj: v,
                source: e,
            })
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let is_authenicated = self.is_authenticated();
        let endpoint = self.endpoint();
        let endpoint_type = self.endpoint_type();

        // Build the URL.
        let mut url = client.rest_endpoint(&endpoint, &endpoint_type)?;

        // Add query parameters to the URL.
        if let Some(parameters) = self.parameters() {
            parameters.add_to_url(&mut url);
        }

        let request_builder = Request::builder()
            .method(self.method())
            .uri(url_to_http_uri(url));

        // Add the body to the request if any.
        let (request_builder, body) = if let Some((mime, data)) = self.body() {
            (request_builder.header(header::CONTENT_TYPE, mime), data)
        } else {
            (request_builder, Map::new())
        };

        // Send off the request
        let rsp = client
            .rest_async(
                request_builder,
                body,
                is_authenicated.then_some(endpoint),
                &endpoint_type,
            )
            .await?;

        // Check the response status and extract errors if needed.
        let status = rsp.status();

        let v = serde_json::from_slice(rsp.body()).map_err(|_e| ApiError::ServerError {
            status,
            source: ServerError::InvalidJson {
                data: rsp.body().into_iter().copied().collect(),
            },
        })?;

        if !status.is_success() {
            return Err(ApiError::ServerError {
                status,
                source: ServerError::NotSuccess { obj: v },
            });
        }

        // Deserialize into whatever type the caller is asking.
        serde_json::from_value::<T>(v.clone())
            .map(|response| response)
            .map_err(|e| ApiError::DataType {
                typename: any::type_name::<T>(),
                obj: v,
                source: e,
            })
    }
}
