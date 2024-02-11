use derive_builder::Builder;
use http::Method;
use serde::{de::Error, Deserialize};
use serde_json::Value;

use crate::api::{endpoint::{Endpoint, Response}, params::QueryParams, spot::ApiResponse};

#[derive(Debug, Clone, Copy, Builder)]
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
        "/0/public/Time".to_owned()
    }

    fn parameters(&self) -> Option<QueryParams> {
        None
    }
}

pub type LastTimeResp = TimeResp;
pub type HistTimeResp = Vec<TimeResp>;

#[derive(Debug, Deserialize)]
pub struct TimeResp {
    pub unixtime: u64,
    pub rfc1123: String,
}

impl Response for TimeResp {
    fn unwrap(v: Value) -> Self {
        serde_json::from_value::<ApiResponse<Self>>(v.clone()).map(|res| res.result).unwrap()
    }
}