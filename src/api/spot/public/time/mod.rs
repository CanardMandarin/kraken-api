use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Time {}

impl Time {
    pub fn builder() -> TimeBuilder {
        TimeBuilder::default()
    }
}

impl Endpoint for Time {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "0/public/Time".to_owned()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

pub type LastTimeResp = TimeResp;
pub type HistTimeResp = Vec<TimeResp>;

#[derive(Debug, Deserialize)]
pub struct TimeResp {
    pub unixtime: u64,
    pub rfc1123: String,
}
