/*
 * Binance Derivatives Trading Portfolio Margin REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading Portfolio Margin REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_portfolio_margin::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryAllCmOrdersResponseInner {
    #[serde(rename = "avgPrice", skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<String>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "cumBase", skip_serializing_if = "Option::is_none")]
    pub cum_base: Option<String>,
    #[serde(rename = "executedQty", skip_serializing_if = "Option::is_none")]
    pub executed_qty: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(rename = "origQty", skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<String>,
    #[serde(rename = "origType", skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

impl QueryAllCmOrdersResponseInner {
    #[must_use]
    pub fn new() -> QueryAllCmOrdersResponseInner {
        QueryAllCmOrdersResponseInner {
            avg_price: None,
            client_order_id: None,
            cum_base: None,
            executed_qty: None,
            order_id: None,
            orig_qty: None,
            orig_type: None,
            price: None,
            reduce_only: None,
            side: None,
            position_side: None,
            status: None,
            symbol: None,
            pair: None,
            time: None,
            time_in_force: None,
            r#type: None,
            update_time: None,
        }
    }
}
