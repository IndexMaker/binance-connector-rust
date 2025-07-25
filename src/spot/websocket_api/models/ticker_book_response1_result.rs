/*
 * Binance Spot WebSocket API
 *
 * OpenAPI Specifications for the Binance Spot WebSocket API
 *
 * API documents:
 * - [Github web-socket-api documentation file](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-api.md)
 * - [General API information for web-socket-api on website](https://developers.binance.com/docs/binance-spot-api-docs/web-socket-api/general-api-information)
 *
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::spot::websocket_api::models;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickerBookResponse1Result {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "bidPrice", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    #[serde(rename = "bidQty", skip_serializing_if = "Option::is_none")]
    pub bid_qty: Option<String>,
    #[serde(rename = "askPrice", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<String>,
    #[serde(rename = "askQty", skip_serializing_if = "Option::is_none")]
    pub ask_qty: Option<String>,
}

impl TickerBookResponse1Result {
    #[must_use]
    pub fn new() -> TickerBookResponse1Result {
        TickerBookResponse1Result {
            symbol: None,
            bid_price: None,
            bid_qty: None,
            ask_price: None,
            ask_qty: None,
        }
    }
}
