use async_trait::async_trait;
use bytes::Bytes;
use http::{header, request::Builder as RequestBuilder, Response};
use reqwest::blocking::Client as ReqClient;
use reqwest::Client as ReqAsyncClient;
use serde_json::{Map, Value};
use thiserror::Error;
use url::Url;

use crate::{
    api::{
        client::{AsyncClient, Client, RestClient},
        endpoint::EndpointType,
        error::ApiError,
    },
    auth::Auth,
};

#[derive(Debug, Error)]
pub enum RestError {
    #[error("`HTTP error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },

    #[error("Communication with Kraken: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
}

const SPOT_API_URL: &str = "https://api.kraken.com";
const FUTURES_API_URL: &str = "https://futures.kraken.com";
const TEST_FUTURES_API_URL: &str = "https://demo-futures.kraken.com/";

#[derive(Debug)]
pub struct Kraken {
    /// The client to use for API calls.
    client: ReqClient,

    /// The base URL to use for spot API calls.
    spot_api_url: Url,

    /// The base URL to use for futures API calls.
    futures_api_url: Url,

    /// The authentication information to use.
    auth: Option<Auth>,
}

#[derive(Debug)]
pub struct AsyncKraken {
    /// The client to use for API calls.
    client: ReqAsyncClient,

    /// The base URL to use for spot API calls.
    spot_api_url: Url,

    /// The base URL to use for futures API calls.
    futures_api_url: Url,

    /// The authentication information to use.
    auth: Option<Auth>,
}

impl Kraken {
    pub fn new() -> Self {
        Self {
            client: ReqClient::new(),
            spot_api_url: Url::parse(SPOT_API_URL).unwrap(),
            futures_api_url: Url::parse(FUTURES_API_URL).unwrap(),
            auth: None,
        }
    }

    pub fn new_auth(api_key: &str, secret_key: &str) -> Self {
        Self {
            client: ReqClient::new(),
            spot_api_url: Url::parse(SPOT_API_URL).unwrap(),
            futures_api_url: Url::parse(FUTURES_API_URL).unwrap(),
            auth: Some(Auth::new(api_key.to_string(), secret_key.to_string())),
        }
    }

    pub fn new_auth_with_test(api_key: &str, secret_key: &str) -> Self {
        Self {
            client: ReqClient::new(),
            spot_api_url: Url::parse(SPOT_API_URL).unwrap(),
            futures_api_url: Url::parse(TEST_FUTURES_API_URL).unwrap(),
            auth: Some(Auth::new(api_key.to_string(), secret_key.to_string())),
        }
    }
}

impl Default for Kraken {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncKraken {
    pub fn new() -> Self {
        Self {
            client: ReqAsyncClient::new(),
            spot_api_url: Url::parse(SPOT_API_URL).unwrap(),
            futures_api_url: Url::parse(FUTURES_API_URL).unwrap(),
            auth: None,
        }
    }

    pub fn new_auth(api_key: &str, secret_key: &str) -> Self {
        Self {
            client: ReqAsyncClient::new(),
            spot_api_url: Url::parse(SPOT_API_URL).unwrap(),
            futures_api_url: Url::parse(FUTURES_API_URL).unwrap(),
            auth: Some(Auth::new(api_key.to_string(), secret_key.to_string())),
        }
    }

    pub fn new_auth_with_test(api_key: &str, secret_key: &str) -> Self {
        Self {
            client: ReqAsyncClient::new(),
            spot_api_url: Url::parse(SPOT_API_URL).unwrap(),
            futures_api_url: Url::parse(TEST_FUTURES_API_URL).unwrap(),
            auth: Some(Auth::new(api_key.to_string(), secret_key.to_string())),
        }
    }
}

impl Default for AsyncKraken {
    fn default() -> Self {
        Self::new()
    }
}

impl RestClient for Kraken {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
        endpoint_type: &EndpointType,
    ) -> Result<Url, ApiError<Self::Error>> {
        match endpoint_type {
            EndpointType::Spot => Ok(self.spot_api_url.join(endpoint)?),
            EndpointType::Futures => Ok(self.futures_api_url.join(endpoint)?),
        }
    }
}

impl RestClient for AsyncKraken {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
        endpoint_type: &EndpointType,
    ) -> Result<Url, ApiError<Self::Error>> {
        match endpoint_type {
            EndpointType::Spot => Ok(self.spot_api_url.join(endpoint)?),
            EndpointType::Futures => Ok(self.futures_api_url.join(endpoint)?),
        }
    }
}

impl Client for Kraken {
    fn rest(
        &self,
        mut request_builder: RequestBuilder,
        mut body: Map<String, Value>,
        path_to_sign: Option<String>,
        endpoint_type: &EndpointType,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || {
            // If a path to sign has been provided, compute and adds the necessary authorization headers to the request.
            if let (Some(path_to_sign), Some(auth)) = (path_to_sign, &self.auth) {
                auth.set_headers(
                    request_builder.headers_mut().unwrap(),
                    &path_to_sign,
                    &mut body,
                    endpoint_type,
                );
            }

            // Build the request.
            let encoded_body = if let Some(Some(content_type)) = request_builder
                .headers_ref()
                .map(|h| h.get(header::CONTENT_TYPE))
            {
                match content_type.to_str().unwrap() {
                    "application/x-www-form-urlencoded" => {
                        serde_urlencoded::to_string(&body).unwrap()
                    }
                    "application/json" => serde_json::to_string(&body).unwrap(),
                    _ => "".to_owned(),
                }
            } else {
                "".to_owned()
            };

            let http_request = request_builder.body(encoded_body)?;

            // Convert it to a reqwest::Request type and send it.
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            // Build the HTTP response.
            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());

            // Insert any headers in the reponses.
            if let Some(headers) = http_rsp.headers_mut() {
                for (key, value) in rsp.headers() {
                    headers.insert(key, value.clone());
                }
            }

            // Return the reponse as raw bytes.
            Ok(http_rsp.body(rsp.bytes()?)?)
        };

        call().map_err(ApiError::client)
    }
}

#[async_trait]
impl AsyncClient for AsyncKraken {
    async fn rest_async(
        &self,
        mut request_builder: RequestBuilder,
        mut body: Map<String, Value>,
        path_to_sign: Option<String>,
        endpoint_type: &EndpointType,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        let call = || async {
            // If a path to sign has been provided, compute and adds the necessary authorization headers to the request.
            if let (Some(path_to_sign), Some(auth)) = (path_to_sign, &self.auth) {
                auth.set_headers(
                    request_builder.headers_mut().unwrap(),
                    &path_to_sign,
                    &mut body,
                    endpoint_type,
                );
            }

            // // Build the request.
            // let encoded_body = if let Some(Some(content_type)) = request_builder
            //     .headers_ref()
            //     .map(|h| h.get(header::CONTENT_TYPE))
            // {
            //     match content_type.to_str().unwrap() {
            //         "application/x-www-form-urlencoded" => {
            //             serde_urlencoded::to_string(&body).unwrap()
            //         }
            //         "application/json" => serde_json::to_string(&body).unwrap(),
            //         _ => "".to_owned(),
            //     }
            // } else {
            //     "".to_owned()
            // }.into_bytes();

            let http_request = request_builder.body(vec![])?;


            // Convert it to a reqwest::Request type and send it.
            let request = http_request.try_into()?;
            println!("{:#?}", request);

            let rsp = self.client.execute(request).await?;

            // Build the HTTP response.
            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());

            // Insert any headers in the reponses.
            if let Some(headers) = http_rsp.headers_mut() {
                for (key, value) in rsp.headers() {
                    headers.insert(key, value.clone());
                }
            }

            // Return the reponse as raw bytes.
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };

        call().await.map_err(ApiError::client)
    }
}
