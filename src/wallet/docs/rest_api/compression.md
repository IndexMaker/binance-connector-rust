# Compression Configuration

```rust
use binance_sdk::wallet;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = wallet::WalletRestApi::production(configuration);
let params = wallet::rest_api::AccountInfoParams::default();
let response = client.account_info(params).await?;
```
