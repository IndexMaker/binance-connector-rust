/*
 * Binance VIP Loan REST API
 *
 * OpenAPI Specification for the Binance VIP Loan REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use async_trait::async_trait;
use derive_builder::Builder;
use reqwest;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

use crate::common::{
    config::ConfigurationRestApi,
    models::{ParamBuildError, RestApiResponse},
    utils::send_request,
};
use crate::vip_loan::rest_api::models;

const HAS_TIME_UNIT: bool = false;

#[async_trait]
pub trait UserInformationApi: Send + Sync {
    async fn check_vip_loan_collateral_account(
        &self,
        params: CheckVipLoanCollateralAccountParams,
    ) -> anyhow::Result<RestApiResponse<models::CheckVipLoanCollateralAccountResponse>>;
    async fn get_vip_loan_ongoing_orders(
        &self,
        params: GetVipLoanOngoingOrdersParams,
    ) -> anyhow::Result<RestApiResponse<models::GetVipLoanOngoingOrdersResponse>>;
    async fn query_application_status(
        &self,
        params: QueryApplicationStatusParams,
    ) -> anyhow::Result<RestApiResponse<models::QueryApplicationStatusResponse>>;
}

#[derive(Debug, Clone)]
pub struct UserInformationApiClient {
    configuration: ConfigurationRestApi,
}

impl UserInformationApiClient {
    pub fn new(configuration: ConfigurationRestApi) -> Self {
        Self { configuration }
    }
}

/// Request parameters for the [`check_vip_loan_collateral_account`] operation.
///
/// This struct holds all of the inputs you can pass when calling
/// [`check_vip_loan_collateral_account`](#method.check_vip_loan_collateral_account).
#[derive(Clone, Debug, Builder, Default)]
#[builder(pattern = "owned", build_fn(error = "ParamBuildError"))]
pub struct CheckVipLoanCollateralAccountParams {
    ///
    /// The `order_id` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub order_id: Option<i64>,
    ///
    /// The `collateral_account_id` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub collateral_account_id: Option<i64>,
    ///
    /// The `recv_window` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub recv_window: Option<i64>,
}

impl CheckVipLoanCollateralAccountParams {
    /// Create a builder for [`check_vip_loan_collateral_account`].
    ///
    #[must_use]
    pub fn builder() -> CheckVipLoanCollateralAccountParamsBuilder {
        CheckVipLoanCollateralAccountParamsBuilder::default()
    }
}
/// Request parameters for the [`get_vip_loan_ongoing_orders`] operation.
///
/// This struct holds all of the inputs you can pass when calling
/// [`get_vip_loan_ongoing_orders`](#method.get_vip_loan_ongoing_orders).
#[derive(Clone, Debug, Builder, Default)]
#[builder(pattern = "owned", build_fn(error = "ParamBuildError"))]
pub struct GetVipLoanOngoingOrdersParams {
    ///
    /// The `order_id` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub order_id: Option<i64>,
    ///
    /// The `collateral_account_id` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub collateral_account_id: Option<i64>,
    ///
    /// The `loan_coin` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub loan_coin: Option<String>,
    ///
    /// The `collateral_coin` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub collateral_coin: Option<String>,
    /// Currently querying page. Start from 1, Default:1, Max: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub current: Option<i64>,
    /// Default: 10, Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub limit: Option<i64>,
    ///
    /// The `recv_window` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub recv_window: Option<i64>,
}

impl GetVipLoanOngoingOrdersParams {
    /// Create a builder for [`get_vip_loan_ongoing_orders`].
    ///
    #[must_use]
    pub fn builder() -> GetVipLoanOngoingOrdersParamsBuilder {
        GetVipLoanOngoingOrdersParamsBuilder::default()
    }
}
/// Request parameters for the [`query_application_status`] operation.
///
/// This struct holds all of the inputs you can pass when calling
/// [`query_application_status`](#method.query_application_status).
#[derive(Clone, Debug, Builder, Default)]
#[builder(pattern = "owned", build_fn(error = "ParamBuildError"))]
pub struct QueryApplicationStatusParams {
    /// Currently querying page. Start from 1, Default:1, Max: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub current: Option<i64>,
    /// Default: 10, Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub limit: Option<i64>,
    ///
    /// The `recv_window` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub recv_window: Option<i64>,
}

impl QueryApplicationStatusParams {
    /// Create a builder for [`query_application_status`].
    ///
    #[must_use]
    pub fn builder() -> QueryApplicationStatusParamsBuilder {
        QueryApplicationStatusParamsBuilder::default()
    }
}

#[async_trait]
impl UserInformationApi for UserInformationApiClient {
    async fn check_vip_loan_collateral_account(
        &self,
        params: CheckVipLoanCollateralAccountParams,
    ) -> anyhow::Result<RestApiResponse<models::CheckVipLoanCollateralAccountResponse>> {
        let CheckVipLoanCollateralAccountParams {
            order_id,
            collateral_account_id,
            recv_window,
        } = params;

        let mut query_params = BTreeMap::new();

        if let Some(rw) = order_id {
            query_params.insert("orderId".to_string(), json!(rw));
        }

        if let Some(rw) = collateral_account_id {
            query_params.insert("collateralAccountId".to_string(), json!(rw));
        }

        if let Some(rw) = recv_window {
            query_params.insert("recvWindow".to_string(), json!(rw));
        }

        send_request::<models::CheckVipLoanCollateralAccountResponse>(
            &self.configuration,
            "/sapi/v1/loan/vip/collateral/account",
            reqwest::Method::GET,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            true,
        )
        .await
    }

    async fn get_vip_loan_ongoing_orders(
        &self,
        params: GetVipLoanOngoingOrdersParams,
    ) -> anyhow::Result<RestApiResponse<models::GetVipLoanOngoingOrdersResponse>> {
        let GetVipLoanOngoingOrdersParams {
            order_id,
            collateral_account_id,
            loan_coin,
            collateral_coin,
            current,
            limit,
            recv_window,
        } = params;

        let mut query_params = BTreeMap::new();

        if let Some(rw) = order_id {
            query_params.insert("orderId".to_string(), json!(rw));
        }

        if let Some(rw) = collateral_account_id {
            query_params.insert("collateralAccountId".to_string(), json!(rw));
        }

        if let Some(rw) = loan_coin {
            query_params.insert("loanCoin".to_string(), json!(rw));
        }

        if let Some(rw) = collateral_coin {
            query_params.insert("collateralCoin".to_string(), json!(rw));
        }

        if let Some(rw) = current {
            query_params.insert("current".to_string(), json!(rw));
        }

        if let Some(rw) = limit {
            query_params.insert("limit".to_string(), json!(rw));
        }

        if let Some(rw) = recv_window {
            query_params.insert("recvWindow".to_string(), json!(rw));
        }

        send_request::<models::GetVipLoanOngoingOrdersResponse>(
            &self.configuration,
            "/sapi/v1/loan/vip/ongoing/orders",
            reqwest::Method::GET,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            true,
        )
        .await
    }

    async fn query_application_status(
        &self,
        params: QueryApplicationStatusParams,
    ) -> anyhow::Result<RestApiResponse<models::QueryApplicationStatusResponse>> {
        let QueryApplicationStatusParams {
            current,
            limit,
            recv_window,
        } = params;

        let mut query_params = BTreeMap::new();

        if let Some(rw) = current {
            query_params.insert("current".to_string(), json!(rw));
        }

        if let Some(rw) = limit {
            query_params.insert("limit".to_string(), json!(rw));
        }

        if let Some(rw) = recv_window {
            query_params.insert("recvWindow".to_string(), json!(rw));
        }

        send_request::<models::QueryApplicationStatusResponse>(
            &self.configuration,
            "/sapi/v1/loan/vip/request/data",
            reqwest::Method::GET,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            true,
        )
        .await
    }
}

#[cfg(all(test, feature = "vip_loan"))]
mod tests {
    use super::*;
    use crate::TOKIO_SHARED_RT;
    use crate::{errors::ConnectorError, models::DataFuture, models::RestApiRateLimit};
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct DummyRestApiResponse<T> {
        inner: Box<dyn FnOnce() -> DataFuture<Result<T, ConnectorError>> + Send + Sync>,
        status: u16,
        headers: HashMap<String, String>,
        rate_limits: Option<Vec<RestApiRateLimit>>,
    }

    impl<T> From<DummyRestApiResponse<T>> for RestApiResponse<T> {
        fn from(dummy: DummyRestApiResponse<T>) -> Self {
            Self {
                data_fn: dummy.inner,
                status: dummy.status,
                headers: dummy.headers,
                rate_limits: dummy.rate_limits,
            }
        }
    }

    struct MockUserInformationApiClient {
        force_error: bool,
    }

    #[async_trait]
    impl UserInformationApi for MockUserInformationApiClient {
        async fn check_vip_loan_collateral_account(
            &self,
            _params: CheckVipLoanCollateralAccountParams,
        ) -> anyhow::Result<RestApiResponse<models::CheckVipLoanCollateralAccountResponse>>
        {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"collateralAccountId":"12345678","collateralCoin":"BNB,BTC,ETH"},{"collateralAccountId":"23456789","collateralCoin":"BNB,BTC,ETH"}],"total":2}"#).unwrap();
            let dummy_response: models::CheckVipLoanCollateralAccountResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::CheckVipLoanCollateralAccountResponse");

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn get_vip_loan_ongoing_orders(
            &self,
            _params: GetVipLoanOngoingOrdersParams,
        ) -> anyhow::Result<RestApiResponse<models::GetVipLoanOngoingOrdersResponse>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"orderId":100000001,"loanCoin":"BUSD","totalDebt":"10000","residualInterest":"10.27687923","collateralAccountId":"12345678,23456789","collateralCoin":"BNB,BTC,ETH","totalCollateralValueAfterHaircut":"25000.27565492","lockedCollateralValue":"25000.27565492","currentLTV":"0.57","expirationTime":1575018510000,"loanDate":"1676851200000","loanTerm":"30days"}],"total":1}"#).unwrap();
            let dummy_response: models::GetVipLoanOngoingOrdersResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::GetVipLoanOngoingOrdersResponse");

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn query_application_status(
            &self,
            _params: QueryApplicationStatusParams,
        ) -> anyhow::Result<RestApiResponse<models::QueryApplicationStatusResponse>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"loanAccountId":"12345678","orderId":"12345678","requestId":"12345678","loanCoin":"BTC","loanAmount":"100.55","collateralAccountId":"12345678,12345678,12345678","collateralCoin":"BUSD,USDT,ETH","loanTerm":"30","status":"Repaid","loanDate":"1676851200000"}],"total":1}"#).unwrap();
            let dummy_response: models::QueryApplicationStatusResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::QueryApplicationStatusResponse");

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }
    }

    #[test]
    fn check_vip_loan_collateral_account_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: false };

            let params = CheckVipLoanCollateralAccountParams::builder().build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"collateralAccountId":"12345678","collateralCoin":"BNB,BTC,ETH"},{"collateralAccountId":"23456789","collateralCoin":"BNB,BTC,ETH"}],"total":2}"#).unwrap();
            let expected_response : models::CheckVipLoanCollateralAccountResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::CheckVipLoanCollateralAccountResponse");

            let resp = client.check_vip_loan_collateral_account(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn check_vip_loan_collateral_account_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: false };

            let params = CheckVipLoanCollateralAccountParams::builder().order_id(1).collateral_account_id(1).recv_window(5000).build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"collateralAccountId":"12345678","collateralCoin":"BNB,BTC,ETH"},{"collateralAccountId":"23456789","collateralCoin":"BNB,BTC,ETH"}],"total":2}"#).unwrap();
            let expected_response : models::CheckVipLoanCollateralAccountResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::CheckVipLoanCollateralAccountResponse");

            let resp = client.check_vip_loan_collateral_account(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn check_vip_loan_collateral_account_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: true };

            let params = CheckVipLoanCollateralAccountParams::builder()
                .build()
                .unwrap();

            match client.check_vip_loan_collateral_account(params).await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn get_vip_loan_ongoing_orders_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: false };

            let params = GetVipLoanOngoingOrdersParams::builder().build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"orderId":100000001,"loanCoin":"BUSD","totalDebt":"10000","residualInterest":"10.27687923","collateralAccountId":"12345678,23456789","collateralCoin":"BNB,BTC,ETH","totalCollateralValueAfterHaircut":"25000.27565492","lockedCollateralValue":"25000.27565492","currentLTV":"0.57","expirationTime":1575018510000,"loanDate":"1676851200000","loanTerm":"30days"}],"total":1}"#).unwrap();
            let expected_response : models::GetVipLoanOngoingOrdersResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::GetVipLoanOngoingOrdersResponse");

            let resp = client.get_vip_loan_ongoing_orders(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn get_vip_loan_ongoing_orders_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: false };

            let params = GetVipLoanOngoingOrdersParams::builder().order_id(1).collateral_account_id(1).loan_coin("loan_coin_example".to_string()).collateral_coin("collateral_coin_example".to_string()).current(1).limit(10).recv_window(5000).build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"orderId":100000001,"loanCoin":"BUSD","totalDebt":"10000","residualInterest":"10.27687923","collateralAccountId":"12345678,23456789","collateralCoin":"BNB,BTC,ETH","totalCollateralValueAfterHaircut":"25000.27565492","lockedCollateralValue":"25000.27565492","currentLTV":"0.57","expirationTime":1575018510000,"loanDate":"1676851200000","loanTerm":"30days"}],"total":1}"#).unwrap();
            let expected_response : models::GetVipLoanOngoingOrdersResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::GetVipLoanOngoingOrdersResponse");

            let resp = client.get_vip_loan_ongoing_orders(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn get_vip_loan_ongoing_orders_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: true };

            let params = GetVipLoanOngoingOrdersParams::builder().build().unwrap();

            match client.get_vip_loan_ongoing_orders(params).await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn query_application_status_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: false };

            let params = QueryApplicationStatusParams::builder().build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"loanAccountId":"12345678","orderId":"12345678","requestId":"12345678","loanCoin":"BTC","loanAmount":"100.55","collateralAccountId":"12345678,12345678,12345678","collateralCoin":"BUSD,USDT,ETH","loanTerm":"30","status":"Repaid","loanDate":"1676851200000"}],"total":1}"#).unwrap();
            let expected_response : models::QueryApplicationStatusResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::QueryApplicationStatusResponse");

            let resp = client.query_application_status(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn query_application_status_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: false };

            let params = QueryApplicationStatusParams::builder().current(1).limit(10).recv_window(5000).build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"rows":[{"loanAccountId":"12345678","orderId":"12345678","requestId":"12345678","loanCoin":"BTC","loanAmount":"100.55","collateralAccountId":"12345678,12345678,12345678","collateralCoin":"BUSD,USDT,ETH","loanTerm":"30","status":"Repaid","loanDate":"1676851200000"}],"total":1}"#).unwrap();
            let expected_response : models::QueryApplicationStatusResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::QueryApplicationStatusResponse");

            let resp = client.query_application_status(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn query_application_status_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserInformationApiClient { force_error: true };

            let params = QueryApplicationStatusParams::builder().build().unwrap();

            match client.query_application_status(params).await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }
}
