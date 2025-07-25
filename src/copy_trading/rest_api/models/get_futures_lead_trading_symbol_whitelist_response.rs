/*
 * Binance Copy Trading REST API
 *
 * OpenAPI Specification for the Binance Copy Trading REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::copy_trading::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFuturesLeadTradingSymbolWhitelistResponse {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::GetFuturesLeadTradingSymbolWhitelistResponseDataInner>>,
}

impl GetFuturesLeadTradingSymbolWhitelistResponse {
    #[must_use]
    pub fn new() -> GetFuturesLeadTradingSymbolWhitelistResponse {
        GetFuturesLeadTradingSymbolWhitelistResponse {
            code: None,
            message: None,
            data: None,
        }
    }
}
