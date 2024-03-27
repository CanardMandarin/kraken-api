use async_trait::async_trait;
use http::Uri;
use url::Url;

use crate::api::{
    client::{AsyncClient, Client},
    error::ApiError,
};

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str().parse::<Uri>().unwrap()
}

/// A trait which represents a query which may be made to the Kraken REST API.
pub trait Query<T, C>
where
    C: for<'a> Client<'a>,
{
    /// Perform the query against the client.
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to the Kraken REST API.
#[async_trait]
pub trait AsyncQuery<T, C>
where
    C: for<'a> AsyncClient<'a>,
{
    /// Perform the query asynchronously against the client.
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
