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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickerTradingDayResponse {
    TickerTradingDayResponse1(Box<models::TickerTradingDayResponse1>),
    TickerTradingDayResponse2(Vec<models::TickerTradingDayResponse2Inner>),
    Other(serde_json::Value),
}

impl Default for TickerTradingDayResponse {
    fn default() -> Self {
        Self::TickerTradingDayResponse1(Default::default())
    }
}
