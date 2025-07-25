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
pub struct DepositAssetsIntoTheManagedSubAccountResponse {
    #[serde(rename = "tranId", skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
}

impl DepositAssetsIntoTheManagedSubAccountResponse {
    #[must_use]
    pub fn new() -> DepositAssetsIntoTheManagedSubAccountResponse {
        DepositAssetsIntoTheManagedSubAccountResponse { tran_id: None }
    }
}
