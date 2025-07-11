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
pub struct OrderCancelReplaceResponseData {
    #[serde(rename = "cancelResult", skip_serializing_if = "Option::is_none")]
    pub cancel_result: Option<String>,
    #[serde(rename = "newOrderResult", skip_serializing_if = "Option::is_none")]
    pub new_order_result: Option<String>,
    #[serde(rename = "cancelResponse", skip_serializing_if = "Option::is_none")]
    pub cancel_response: Option<Box<models::OrderCancelReplaceResponseDataCancelResponse>>,
    #[serde(rename = "newOrderResponse", skip_serializing_if = "Option::is_none")]
    pub new_order_response: Option<Box<models::OrderCancelReplaceResponseDataNewOrderResponse>>,
}

impl OrderCancelReplaceResponseData {
    #[must_use]
    pub fn new() -> OrderCancelReplaceResponseData {
        OrderCancelReplaceResponseData {
            cancel_result: None,
            new_order_result: None,
            cancel_response: None,
            new_order_response: None,
        }
    }
}
