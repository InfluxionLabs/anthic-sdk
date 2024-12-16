# Anthic SDK

Anthic Rust SDK which thinly wraps an Anthic API Client.

```rust
#[tokio::main]
async fn main() {
    let network = NetworkDefinition::from_str("stokenet").unwrap();
    let trade_api_url = "https://trade-api.staging.anthic.io";
    let anthic_api_key = "<YOUR-ANTHIC-API-KEY>";

    let client = AnthicClient::new(
        network.clone(),
        trade_api_url.to_string(),
        anthic_api_key.to_string()
    );
    
    let tokens = client.tokens().unwrap().await;
}
```