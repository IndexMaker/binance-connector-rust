use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::margin_trading::{
    MarginTradingRestApi, rest_api::KeepaliveIsolatedMarginUserDataStreamParams,
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

    // Create the MarginTrading REST API client
    let rest_client = MarginTradingRestApi::production(rest_conf);

    // Setup the API parameters
    let params = KeepaliveIsolatedMarginUserDataStreamParams::builder(
        "symbol_example".to_string(),
        "listen_key_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .keepalive_isolated_margin_user_data_stream(params)
        .await
        .context("keepalive_isolated_margin_user_data_stream request failed")?;

    info!(?response.rate_limits, "keepalive_isolated_margin_user_data_stream rate limits");
    let data = response.data().await?;
    info!(?data, "keepalive_isolated_margin_user_data_stream data");

    Ok(())
}
