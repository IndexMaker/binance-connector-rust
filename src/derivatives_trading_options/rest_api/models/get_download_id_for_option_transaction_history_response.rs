/*
 * Binance Derivatives Trading Options REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading Options REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_options::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDownloadIdForOptionTransactionHistoryResponse {
    #[serde(
        rename = "avgCostTimestampOfLast30d",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    pub download_id: Option<String>,
}

impl GetDownloadIdForOptionTransactionHistoryResponse {
    #[must_use]
    pub fn new() -> GetDownloadIdForOptionTransactionHistoryResponse {
        GetDownloadIdForOptionTransactionHistoryResponse {
            avg_cost_timestamp_of_last30d: None,
            download_id: None,
        }
    }
}
