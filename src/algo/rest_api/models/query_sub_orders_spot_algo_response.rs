/*
 * Binance Algo REST API
 *
 * OpenAPI Specification for the Binance Algo REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::algo::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuerySubOrdersSpotAlgoResponse {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "executedQty", skip_serializing_if = "Option::is_none")]
    pub executed_qty: Option<String>,
    #[serde(rename = "executedAmt", skip_serializing_if = "Option::is_none")]
    pub executed_amt: Option<String>,
    #[serde(rename = "subOrders", skip_serializing_if = "Option::is_none")]
    pub sub_orders: Option<Vec<models::QuerySubOrdersFutureAlgoResponseSubOrdersInner>>,
}

impl QuerySubOrdersSpotAlgoResponse {
    #[must_use]
    pub fn new() -> QuerySubOrdersSpotAlgoResponse {
        QuerySubOrdersSpotAlgoResponse {
            total: None,
            executed_qty: None,
            executed_amt: None,
            sub_orders: None,
        }
    }
}
