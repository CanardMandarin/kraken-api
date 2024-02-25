use std::collections::HashMap;

use derive_builder::Builder;
use http::Method;
use serde::{
    de::{self, IntoDeserializer},
    Deserialize, Serialize,
};
use serde_json::{Map, Value};

use crate::api::endpoint::{Endpoint, EndpointType};

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub enum OrderType {
    #[serde(rename = "lmt")]
    Limit,
    #[serde(rename = "post")]
    Post,
    #[default]
    #[serde(rename = "mkt")]
    Market,
    #[serde(rename = "stp")]
    Stop,
    #[serde(rename = "take_profit")]
    TakeProfit,
    #[serde(rename = "ioc")]
    Ioc,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum OrderSide {
    #[default]
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum LimitPriceOffsetUnit {
    #[serde(rename = "QUOTE_CURRENCY")]
    QuoteCurrency,
    #[serde(rename = "PERCENT")]
    Percent,
}

#[derive(Debug, Clone, Serialize)]
pub enum TriggerSignal {
    #[serde(rename = "mark")]
    Mark,
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "last")]
    Last,
}

#[derive(Debug, Clone, Builder, Serialize, Default)]
#[builder(build_fn(validate = "Self::validate"), setter(strip_option, into), default)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SendOrder {
    pub order_type: OrderType,
    pub side: OrderSide,
    pub size: f64,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price_offset_unit: Option<LimitPriceOffsetUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price_offset_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stop_deviation_unit: Option<LimitPriceOffsetUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stop_max_deviation: Option<LimitPriceOffsetUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_signal: Option<TriggerSignal>,
}

impl SendOrderBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(Some(ref arg)) = self.cli_ord_id {
            if arg.len() > 100 {
                return Err("Max length for cli_ord_id is 100".to_string());
            }
        }

        if matches!(
            (self.limit_price_offset_unit, self.limit_price_offset_value),
            (Some(None), Some(_)) | (Some(_), Some(None))
        ) {
            return Err("Either both 'limit_price_offset_unit' and 'limit_price_offset_value' must be set or none of them should be set.".to_string());
        }

        if let Some(ref arg) = self.order_type {
            if (arg == &OrderType::Stop || arg == &OrderType::TakeProfit)
                && self.stop_price.unwrap_or(None).is_none()
            {
                return Err("stop_price is required with Stop or TakeProfit order type".to_string());
            }

            if arg == &OrderType::TrailingStop
                && (self.trailing_stop_deviation_unit.unwrap_or(None).is_none()
                    || self.trailing_stop_max_deviation.unwrap_or(None).is_none())
            {
                return Err("trailing_stop_deviation_unit and trailing_stop_max_deviation are required with TrailingStop order type".to_string());
            }
        }

        Ok(())
    }
}

impl SendOrder {
    pub fn builder() -> SendOrderBuilder {
        SendOrderBuilder::default()
    }
}

impl Endpoint for SendOrder {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("/derivatives/api/v3/sendorder")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Futures
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
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Place,
    Cancel,
    Edit,
    Reject,
    Execution,
}

#[derive(Debug, Deserialize, Clone)]
pub enum OrderTypeResp {
    #[serde(rename = "lmt")]
    Limit,
    #[serde(rename = "ioc")]
    Ioc,
    #[serde(rename = "post")]
    Post,
    #[serde(rename = "liquidation")]
    Liquidation,
    #[serde(rename = "assignment")]
    Assignment,
    #[serde(rename = "stp")]
    Stop,
    #[serde(rename = "unwind")]
    Unwind,
    #[serde(rename = "block")]
    Block,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub cli_ord_id: Option<String>,
    pub filled: f64,
    pub limit_price: Option<f64>,
    // Kraken made a typo apparently
    #[serde(rename = "order_id")]
    pub order_id:  Option<String>,
    pub quantity: f64,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub symbol: String,
    pub timestamp: String,
    pub r#type: OrderTypeResp,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TriggerSide {
    TriggerAbove,
    TriggerBelow,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TriggerSignalResp {
    MarkPrice,
    LastPrice,
    SpotPrice,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTrigger {
    pub client_id: Option<String>,
    pub timlast_update_timestampestamp: String,
    pub limit_price: Option<f64>,
    pub quantity: Option<f64>,

    pub reduce_only: bool,
    pub side: OrderSide,
    pub start_time: Option<String>,
    pub symbol: String,
    pub timestamp: String,
    pub trigger_price: Option<f64>,
    pub trigger_side: Option<TriggerSide>,
    pub trigger_signal: Option<TriggerSignalResp>,

    pub uid: String,
    pub r#type: OrderTypeResp,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTriggerEvent {
    pub order_trigger: OrderTrigger,
    pub r#type: EventType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelTriggerEvent {
    pub order_trigger: OrderTrigger,
    pub r#type: EventType,
    pub uid: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RejectTriggerEvent {
    pub order_trigger: OrderTrigger,
    pub r#type: EventType,
    pub reason: String,
    pub uid: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditEvent {
    pub new: Order,
    pub old: Order,
    pub reduced_quantity: Option<f64>,
    pub r#type: EventType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceEvent {
    pub order: Order,
    pub reduced_quantity: Option<f64>,
    pub r#type: EventType,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CancelEvent {
    pub order: Order,
    pub uid: String,
    pub r#type: EventType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RejectEvent {
    pub order: Order,
    pub uid: String,
    pub reason: RejectReason,
    pub r#type: EventType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteEvent {
    pub amount: f64,
    pub execution_id: String,
    pub order_prior_edit: Option<Order>,
    pub order_prior_execution: Order,
    pub price: f64,
    pub taker_reduced_quantity: Option<f64>,
    pub r#type: EventType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RejectReason {
    PostWouldExecute,
    IocWouldNotExecute,
    WouldNotReducePosition,
    OrderForEditNotFound,
}

#[derive(Debug, Clone)]
pub enum OrderEvent {
    PlaceEvent(PlaceEvent),
    CancelEvent(CancelEvent),
    EditEvent(EditEvent),
    RejectEvent(RejectEvent),
    ExecuteEvent(ExecuteEvent),
    PlaceTriggerEvent(PlaceTriggerEvent),
    CancelTriggerEvent(CancelTriggerEvent),
    RejectTriggerEvent(RejectTriggerEvent),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SendOrderStatus {
    Placed,
    PartiallyFilled,
    Filled,
    Cancelled,
    Edited,
    MarketSuspended,
    MarketInactive,
    InvalidPrice,
    InvalidSize,
    TooManySmallOrders,
    InsufficientAvailableFunds,
    WouldCauseLiquidation,
    ClientOrderIdAlreadyExist,
    ClientOrderIdTooBig,
    MaxPositionViolation,
    OutsidePriceCollar,
    WouldIncreasePriceDislocation,
    NotFound,
    OrderForEditNotAStop,
    OrderForEditNotFound,
    PostWouldExecute,
    IocWouldNotExecute,
    SelfFill,
    WouldNotReducePosition,
    MarketIsPostOnly,
    TooManyOrders,
    FixedLeverageTooHigh,
    ClientOrderIdInvalid,
    CannotEditTriggerPriceOfTrailingStop,
    CannotEditLimitPriceOfTrailingStop,
    WouldProcessAfterSpecifiedTime,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendStatus {
    #[serde(rename = "order_id")]
    pub order_id: String,
    pub received_time: String,
    pub status: SendOrderStatus,
    #[serde(default)]
    pub order_events: Vec<OrderEvent>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendOrderResp {
    pub result: String,
    pub send_status: SendStatus,
    pub server_time: String,
}

// Custom deserialization function for OrderEvent
impl<'de> Deserialize<'de> for OrderEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: HashMap<String, Value> = Deserialize::deserialize(deserializer)?;

        let is_trigger = value.contains_key("orderTrigger");

        // Check if "type" field exists
        if let Some(account_type) = value.get("type") {
            println!("{:?}", account_type);
            match account_type.as_str() {
                Some("PLACE") => {
                    if is_trigger {
                        return PlaceTriggerEvent::deserialize(value.into_deserializer())
                            .map_err(de::Error::custom)
                            .map(OrderEvent::PlaceTriggerEvent);
                    }
                    return PlaceEvent::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(OrderEvent::PlaceEvent);
                }
                Some("CANCEL") => {
                    if is_trigger {
                        return CancelTriggerEvent::deserialize(value.into_deserializer())
                            .map_err(de::Error::custom)
                            .map(OrderEvent::CancelTriggerEvent);
                    }
                    return CancelEvent::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(OrderEvent::CancelEvent);
                }
                Some("EDIT") => {
                    return EditEvent::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(OrderEvent::EditEvent);
                }
                Some("REJECT") => {
                    if is_trigger {
                        return RejectTriggerEvent::deserialize(value.into_deserializer())
                            .map_err(de::Error::custom)
                            .map(OrderEvent::RejectTriggerEvent);
                    }
                    return RejectEvent::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(OrderEvent::RejectEvent);
                }
                Some("EXECUTION") => {
                    return ExecuteEvent::deserialize(value.into_deserializer())
                        .map_err(de::Error::custom)
                        .map(OrderEvent::ExecuteEvent);
                }
                _ => {}
            }
        }

        Err(serde::de::Error::custom("Unknown order event type"))
    }
}
