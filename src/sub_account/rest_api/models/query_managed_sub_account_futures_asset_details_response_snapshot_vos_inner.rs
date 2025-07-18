/*
 * Binance Sub Account REST API
 *
 * OpenAPI Specification for the Binance Sub Account REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::sub_account::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInner {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data:
        Option<Box<models::QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerData>>,
}

impl QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInner {
    #[must_use]
    pub fn new() -> QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInner {
        QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInner {
            r#type: None,
            update_time: None,
            data: None,
        }
    }
}
