/*
 * Binance Dual Investment REST API
 *
 * OpenAPI Specification for the Binance Dual Investment REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::dual_investment::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAutoCompoundStatusResponse {
    #[serde(rename = "positionId", skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(rename = "autoCompoundPlan", skip_serializing_if = "Option::is_none")]
    pub auto_compound_plan: Option<String>,
}

impl ChangeAutoCompoundStatusResponse {
    #[must_use]
    pub fn new() -> ChangeAutoCompoundStatusResponse {
        ChangeAutoCompoundStatusResponse {
            position_id: None,
            auto_compound_plan: None,
        }
    }
}
