/*
 * Binance Staking REST API
 *
 * OpenAPI Specification for the Binance Staking REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::staking::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBnsolRateHistoryResponseRowsInner {
    #[serde(
        rename = "annualPercentageRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub annual_percentage_rate: Option<String>,
    #[serde(rename = "exchangeRate", skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    #[serde(rename = "boostRewards", skip_serializing_if = "Option::is_none")]
    pub boost_rewards: Option<Vec<models::GetBnsolRateHistoryResponseRowsInnerBoostRewardsInner>>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

impl GetBnsolRateHistoryResponseRowsInner {
    #[must_use]
    pub fn new() -> GetBnsolRateHistoryResponseRowsInner {
        GetBnsolRateHistoryResponseRowsInner {
            annual_percentage_rate: None,
            exchange_rate: None,
            boost_rewards: None,
            time: None,
        }
    }
}
