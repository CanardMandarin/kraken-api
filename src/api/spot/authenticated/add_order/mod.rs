use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub enum OrderType {
    #[default]
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop-loss")]
    StopLoss,
    #[serde(rename = "take-profit")]
    TakeProfit,
    #[serde(rename = "stop-loss-limit")]
    StopLossLimit,
    #[serde(rename = "take-profit-limit")]
    TakeProfitLimit,
    #[serde(rename = "trailing-stop")]
    TrailingStop,
    #[serde(rename = "trailing-stop-limit")]
    TrailingStopLimit,
    #[serde(rename = "settle-position")]
    SettlePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum OrderSide {
    #[default]
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SelfTradePrevention {
    #[serde(rename = "cancel-newest")]
    CancelNewest,
    #[serde(rename = "cancel-oldest")]
    CancelOldest,
    #[serde(rename = "cancel-both")]
    CancelBoth,
}

#[derive(Debug, Clone, Serialize)]
pub enum Trigger {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "last")]
    Last,
}

#[derive(Debug, Clone, Serialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "GTD")]
    Gtd,
}

#[derive(Debug, Clone, Builder, Serialize, Default)]
#[builder(
    build_fn(validate = "Self::validate"),
    setter(strip_option, into),
    default
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AddOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userref: Option<i32>,
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    #[serde(rename = "type")]
    pub side: OrderSide,
    pub volume: f64,
    #[serde(rename = "displayvol", skip_serializing_if = "Option::is_none")]
    pub display_vol: Option<f64>,
    pub pair: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stptype: Option<SelfTradePrevention>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oflags: Option<String>,
    #[serde(rename = "timeinforce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starttm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiretm: Option<String>,

    #[serde(rename = "close[ordertype]", skip_serializing_if = "Option::is_none")]
    pub close_order_type: Option<OrderType>,
    #[serde(rename = "close[price]", skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(rename = "close[price2]", skip_serializing_if = "Option::is_none")]
    pub close_price2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(rename = "validate", skip_serializing_if = "Option::is_none")]
    pub validate_args: Option<bool>,
}

impl AddOrderBuilder {
    fn validate(&self) -> Result<(), String> {
        let order_type = self.order_type.as_ref().unwrap();

        if let Some(Some(arg)) = self.display_vol {
            if arg == 0.0 {
                return Err("Field display_vol cannot be 0".to_string());
            }

            if order_type == &OrderType::Limit
                || order_type == &OrderType::StopLossLimit
                || order_type == &OrderType::TakeProfitLimit
                || order_type == &OrderType::TrailingStopLimit
            {
                return Err("Order type has to be a limit when using displayvol".to_string());
            }
        }

        if (order_type == &OrderType::Limit
            || order_type == &OrderType::StopLossLimit
            || order_type == &OrderType::TakeProfitLimit
            || order_type == &OrderType::TrailingStopLimit)
            && self.price.is_none()
        {
            return Err("Cannot create limit order without a price".to_string());
        }
        Ok(())
    }
}

impl AddOrder {
    pub fn builder() -> AddOrderBuilder {
        AddOrderBuilder::default()
    }
}

impl Endpoint for AddOrder {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/0/private/AddOrder")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Map<String, Value>)> {
        let serialized_params: serde_json::Value =
            serde_json::to_value(&self).expect("Serialization failed");

        match serialized_params {
            serde_json::Value::Object(params) => {
                Some(("application/x-www-form-urlencoded", params))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Descr {
    pub order: String,
    pub close: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddOrderResult {
    pub descr: Descr,
    #[serde(default)]
    pub txid: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddOrderResp {
    pub error: Vec<String>,
    pub result: AddOrderResult,
}
