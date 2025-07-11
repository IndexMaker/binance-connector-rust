/*
 * Binance Derivatives Trading Portfolio Margin WebSocket Market Streams
 *
 * OpenAPI Specification for the Binance Derivatives Trading Portfolio Margin WebSocket Market Streams
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_portfolio_margin::websocket_streams::models;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConditionalOrderTradeUpdate {
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub t_uppercase: Option<i64>,
    #[serde(rename = "E", skip_serializing_if = "Option::is_none")]
    pub e_uppercase: Option<i64>,
    #[serde(rename = "fs", skip_serializing_if = "Option::is_none")]
    pub fs: Option<String>,
    #[serde(rename = "so", skip_serializing_if = "Option::is_none")]
    pub so: Option<Box<models::ConditionalOrderTradeUpdateSo>>,
}

impl ConditionalOrderTradeUpdate {
    #[must_use]
    pub fn new() -> ConditionalOrderTradeUpdate {
        ConditionalOrderTradeUpdate {
            t_uppercase: None,
            e_uppercase: None,
            fs: None,
            so: None,
        }
    }
}
