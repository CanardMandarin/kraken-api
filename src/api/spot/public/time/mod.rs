use crate::api::endpoint::Endpoint;
use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

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
}

pub type LastTimeResp = TimeResp;
pub type HistTimeResp = Vec<TimeResp>;

#[derive(Debug, Deserialize, Clone)]
pub struct TimeRespWrapped {
    pub unixtime: u64,
    pub rfc1123: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TimeResp {
    pub result: TimeRespWrapped,
}
