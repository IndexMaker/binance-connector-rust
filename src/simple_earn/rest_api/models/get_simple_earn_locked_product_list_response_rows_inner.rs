/*
 * Binance Simple Earn REST API
 *
 * OpenAPI Specification for the Binance Simple Earn REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::simple_earn::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSimpleEarnLockedProductListResponseRowsInner {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<Box<models::GetSimpleEarnLockedProductListResponseRowsInnerDetail>>,
    #[serde(rename = "quota", skip_serializing_if = "Option::is_none")]
    pub quota: Option<Box<models::GetSimpleEarnLockedProductListResponseRowsInnerQuota>>,
}

impl GetSimpleEarnLockedProductListResponseRowsInner {
    #[must_use]
    pub fn new() -> GetSimpleEarnLockedProductListResponseRowsInner {
        GetSimpleEarnLockedProductListResponseRowsInner {
            project_id: None,
            detail: None,
            quota: None,
        }
    }
}
