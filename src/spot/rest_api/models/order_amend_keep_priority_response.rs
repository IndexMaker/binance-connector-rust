/*
 * Binance Spot REST API
 *
 * OpenAPI Specifications for the Binance Spot REST API
 *
 * API documents:
 * - [Github rest-api documentation file](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md)
 * - [General API information for rest-api on website](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-api-information)
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
use crate::spot::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderAmendKeepPriorityResponse {
    #[serde(rename = "transactTime", skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<i64>,
    #[serde(rename = "executionId", skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<i64>,
    #[serde(rename = "amendedOrder", skip_serializing_if = "Option::is_none")]
    pub amended_order: Option<Box<models::OrderAmendKeepPriorityResponseAmendedOrder>>,
    #[serde(rename = "listStatus", skip_serializing_if = "Option::is_none")]
    pub list_status: Option<Box<models::OrderAmendKeepPriorityResponseListStatus>>,
}

impl OrderAmendKeepPriorityResponse {
    #[must_use]
    pub fn new() -> OrderAmendKeepPriorityResponse {
        OrderAmendKeepPriorityResponse {
            transact_time: None,
            execution_id: None,
            amended_order: None,
            list_status: None,
        }
    }
}
