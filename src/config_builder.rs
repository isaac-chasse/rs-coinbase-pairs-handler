use std::env;

pub struct CoinbaseConfig {
    pub api_key: String,
    pub api_secret: String,
}

impl CoinbaseConfig {
    pub fn new() -> Self {
        CoinbaseConfig {
            api_key: env::var("COINBASE_API_KEY").expect("Failed to set config"),
            api_secret: env::var("COINBASE_API_SECRET").expect("Failed to set config"),
        }
    }
}