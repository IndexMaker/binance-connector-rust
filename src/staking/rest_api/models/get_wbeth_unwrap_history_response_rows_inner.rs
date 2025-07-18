/*
 * Binance Staking REST API
 *
 * OpenAPI Specification for the Binance Staking REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::staking::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetWbethUnwrapHistoryResponseRowsInner {
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "fromAsset", skip_serializing_if = "Option::is_none")]
    pub from_asset: Option<String>,
    #[serde(rename = "fromAmount", skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,
    #[serde(rename = "toAsset", skip_serializing_if = "Option::is_none")]
    pub to_asset: Option<String>,
    #[serde(rename = "toAmount", skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
    #[serde(rename = "exchangeRate", skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl GetWbethUnwrapHistoryResponseRowsInner {
    #[must_use]
    pub fn new() -> GetWbethUnwrapHistoryResponseRowsInner {
        GetWbethUnwrapHistoryResponseRowsInner {
            time: None,
            from_asset: None,
            from_amount: None,
            to_asset: None,
            to_amount: None,
            exchange_rate: None,
            status: None,
        }
    }
}
