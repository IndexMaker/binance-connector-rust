use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::QueryIndexPriceConstituentsParams,
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
    let params =
        QueryIndexPriceConstituentsParams::builder("symbol_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .query_index_price_constituents(params)
        .await
        .context("query_index_price_constituents request failed")?;

    info!(?response.rate_limits, "query_index_price_constituents rate limits");
    let data = response.data().await?;
    info!(?data, "query_index_price_constituents data");

    Ok(())
}
