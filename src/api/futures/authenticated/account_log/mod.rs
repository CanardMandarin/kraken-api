use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

use crate::api::{
    endpoint::{Endpoint, EndpointType},
    params::QueryParams,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AccountLogInfo {
    #[default]
    #[serde(rename = "futures trade")]
    FuturesTrade,
    #[serde(rename = "futures liquidation")]
    FuturesLiquidation,
    #[serde(rename = "assignor")]
    Assignor,
    #[serde(rename = "assignee")]
    Assignee,
    #[serde(rename = "unwind counterparty")]
    UnwindCounterparty,
    #[serde(rename = "unwind bankrupt")]
    UnwindBankrupt,
    #[serde(rename = "covered liquidation")]
    CoveredLiquidation,
    #[serde(rename = "funding rate change")]
    FundingRateChange,
    #[serde(rename = "conversion")]
    Conversion,
    #[serde(rename = "interest payment")]
    InterestPayment,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "cross-exchange transfer")]
    CrossExchangeTransfer,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AccountLogSort {
    #[default]
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Debug, Clone, Builder, Serialize, Default)]
#[builder(setter(into), default)]
pub struct AccountLog {
    pub before: Option<u64>,
    pub count: Option<u64>,
    pub from: Option<String>,
    pub info: Option<AccountLogInfo>,
    pub since: Option<u64>,
    pub sort: Option<AccountLogSort>,
    pub to: Option<u64>,
}

impl AccountLog {
    pub fn builder() -> AccountLogBuilder {
        AccountLogBuilder::default()
    }
}

impl Endpoint for AccountLog {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("/api/history/v3/account-log")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }

    fn parameters(&self) -> Option<QueryParams> {
        let mut params = QueryParams::default();

        if let Some(before) = &self.before {
            params.push("before", before.to_string());
        }

        if let Some(count) = &self.count {
            params.push("count", count.to_string());
        }

        if let Some(from) = &self.from {
            params.push("from", from.to_string());
        }

        if let Some(info) = &self.info {
            params.push("info", serde_json::to_string(info).unwrap());
        }

        if let Some(since) = &self.since {
            params.push("since", since.to_string());
        }

        if let Some(sort) = &self.sort {
            params.push("sort", serde_json::to_string(sort).unwrap());
        }

        if let Some(to) = &self.to {
            params.push("to", to.to_string());
        }

        Some(params)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub asset: Option<String>,
    pub booking_uid: Option<String>,
    pub collateral: Option<String>,
    pub contract: Option<String>,
    pub conversion_spread_percentage: Option<f64>,
    pub date: String,
    pub execution: Option<String>,
    pub fee: Option<f64>,
    pub funding_rate: Option<f64>,
    pub id: i32,
    pub info: AccoumtLogInfo,
    pub liquidation_fee: Option<f64>,
    pub margin_account: Option<String>,
    pub mark_price: Option<f64>,
    pub new_average_entry_price: Option<f64>,
    pub new_balance: Option<f64>,
    pub old_average_entry_price: Option<f64>,
    pub old_balance: Option<f64>,
    pub realized_funding: Option<f64>,
    pub realized_pnl: Option<f64>,
    pub trade_price: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountLogResp {
    pub account_uid: String,
    pub logs: Vec<Log>,
}

