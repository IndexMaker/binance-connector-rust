use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginRestApi, rest_api::UmNotionalAndLeverageBracketsParams,
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

    // Create the DerivativesTradingPortfolioMargin REST API client
    let rest_client = DerivativesTradingPortfolioMarginRestApi::production(rest_conf);

    // Setup the API parameters
    let params = UmNotionalAndLeverageBracketsParams::default();

    // Make the API call
    let response = rest_client
        .um_notional_and_leverage_brackets(params)
        .await
        .context("um_notional_and_leverage_brackets request failed")?;

    info!(?response.rate_limits, "um_notional_and_leverage_brackets rate limits");
    let data = response.data().await?;
    info!(?data, "um_notional_and_leverage_brackets data");

    Ok(())
}
