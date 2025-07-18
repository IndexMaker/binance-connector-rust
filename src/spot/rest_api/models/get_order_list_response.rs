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
pub struct GetOrderListResponse {
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<i64>,
    #[serde(rename = "contingencyType", skip_serializing_if = "Option::is_none")]
    pub contingency_type: Option<String>,
    #[serde(rename = "listStatusType", skip_serializing_if = "Option::is_none")]
    pub list_status_type: Option<String>,
    #[serde(rename = "listOrderStatus", skip_serializing_if = "Option::is_none")]
    pub list_order_status: Option<String>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    #[serde(rename = "transactionTime", skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<i64>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<models::GetOrderListResponseOrdersInner>>,
}

impl GetOrderListResponse {
    #[must_use]
    pub fn new() -> GetOrderListResponse {
        GetOrderListResponse {
            order_list_id: None,
            contingency_type: None,
            list_status_type: None,
            list_order_status: None,
            list_client_order_id: None,
            transaction_time: None,
            symbol: None,
            orders: None,
        }
    }
}
