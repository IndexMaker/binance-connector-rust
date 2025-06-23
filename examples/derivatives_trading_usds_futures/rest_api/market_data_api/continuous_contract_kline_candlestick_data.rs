use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi,
    rest_api::{
        ContinuousContractKlineCandlestickDataContractTypeEnum,
        ContinuousContractKlineCandlestickDataIntervalEnum,
        ContinuousContractKlineCandlestickDataParams,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = ContinuousContractKlineCandlestickDataParams::builder(
        "pair_example".to_string(),
        ContinuousContractKlineCandlestickDataContractTypeEnum::Perpetual,
        ContinuousContractKlineCandlestickDataIntervalEnum::Interval1m,
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .continuous_contract_kline_candlestick_data(params)
        .await
        .context("continuous_contract_kline_candlestick_data request failed")?;

    info!(?response.rate_limits, "continuous_contract_kline_candlestick_data rate limits");
    let data = response.data().await?;
    info!(?data, "continuous_contract_kline_candlestick_data data");

    Ok(())
}
