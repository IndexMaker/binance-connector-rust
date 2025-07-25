/*
 * Binance Convert REST API
 *
 * OpenAPI Specification for the Binance Convert REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::convert::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryOrderQuantityPrecisionPerAssetResponseInner {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "fraction", skip_serializing_if = "Option::is_none")]
    pub fraction: Option<i64>,
}

impl QueryOrderQuantityPrecisionPerAssetResponseInner {
    #[must_use]
    pub fn new() -> QueryOrderQuantityPrecisionPerAssetResponseInner {
        QueryOrderQuantityPrecisionPerAssetResponseInner {
            asset: None,
            fraction: None,
        }
    }
}
