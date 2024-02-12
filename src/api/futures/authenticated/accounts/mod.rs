use std::collections::HashMap;

use derive_builder::Builder;
use http::Method;
use serde::{
    de::{self, IntoDeserializer},
    Deserialize,
};
use serde_json::Value;

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, Copy, Builder)]
pub struct Accounts {}

impl Accounts {
    pub fn builder() -> AccountsBuilder {
        AccountsBuilder::default()
    }
}

impl Endpoint for Accounts {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v3/accounts")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
    }
}

#[derive(Debug, Deserialize)]
pub struct CashAccount {
    pub balances: HashMap<String, f64>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct MarginAccountAuxiliary {
    pub af: f64,
    pub funding: f64,
    pub pnl: f64,
    pub pv: f64,
    pub usd: f64,
}

#[derive(Debug, Deserialize)]
pub struct MarginAccountRequirements {
    pub im: f64,
    pub lt: f64,
    pub mm: f64,
    pub tt: f64,
}

#[derive(Debug, Deserialize)]
pub struct MarginAccount {
    pub auxiliary: MarginAccountAuxiliary,
    pub balances: HashMap<String, f64>,
    pub currency: String,
    #[serde(rename = "marginRequirements")]
    pub margin_requirements: MarginAccountRequirements,
    #[serde(rename = "triggerEstimates")]
    pub trigger_estimates: MarginAccountRequirements,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct MultiCollateralMarginAccountCurrency {
    pub available: f64,
    pub collateral: f64,
    pub quantity: f64,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct MultiCollateralMarginAccount {
    #[serde(rename = "availableMargin")]
    pub available_margin: f64,
    #[serde(rename = "balanceValue")]
    pub balance_value: f64,
    #[serde(rename = "collateralValue")]
    pub collateral_value: f64,
    pub currencies: HashMap<String, MultiCollateralMarginAccountCurrency>,
    #[serde(rename = "initialMargin")]
    pub initial_margin: f64,
    #[serde(rename = "initialMarginWithOrders")]
    pub initial_margin_with_orders: f64,
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: f64,
    #[serde(rename = "marginEquity")]
    pub margin_equity: f64,
    pub pnl: f64,
    #[serde(rename = "portfolioValue")]
    pub portfolio_value: f64,
    #[serde(rename = "totalUnrealized")]
    pub total_unrealized: f64,
    #[serde(rename = "unrealizedFunding")]
    pub unrealized_funding: f64,
}

#[derive(Debug)]
pub enum FuturesAccount {
    CashAccount(CashAccount),
    MarginAccount(MarginAccount),
    MultiCollateralMarginAccount(MultiCollateralMarginAccount),
}

#[derive(Debug, Deserialize)]
pub struct AccountsResp {
    pub result: String,
    pub accounts: HashMap<String, FuturesAccount>,
    #[serde(rename = "serverTime")]
    pub server_time: String,
}

// Custom deserialization function for FuturesAccount
impl<'de> Deserialize<'de> for FuturesAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: HashMap<String, Value> = Deserialize::deserialize(deserializer)?;

        // Check if "type" field exists
        if let Some(account_type) = value.get("type") {
            match account_type.as_str() {
                Some("cashAccount") => {
                    return CashAccount::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(FuturesAccount::CashAccount);
                }
                Some("marginAccount") => {
                    return MarginAccount::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(FuturesAccount::MarginAccount);
                }
                Some("multiCollateralMarginAccount") => {
                    return MultiCollateralMarginAccount::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(FuturesAccount::MultiCollateralMarginAccount);
                }
                _ => {}
            }
        }

        Err(serde::de::Error::custom("Unknown account type"))
    }
}
