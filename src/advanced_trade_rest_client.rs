use crate::{rest_client::Client, models::{RestEndpoint, Products}, config_builder::CoinbaseConfig};
use anyhow::{bail, Result};
use log::{debug}; // removed `error`

pub struct AdvancedTradeRESTClient {
    client: Client,
    key: String,
    secret: String,
}

impl AdvancedTradeRESTClient {
    pub fn new(host: &str) -> AdvancedTradeRESTClient {
        let config = CoinbaseConfig::new();

        AdvancedTradeRESTClient {
            client: Client::new(
                host.to_string(), 
                config.api_key, 
                config.api_secret,
            ),
            key,
            secret
        }
    }

    // returns all product information
    pub async fn get_available_products(&self) -> Result<Products> {
        let api_endpoints: RestEndpoint = RestEndpoint{ 
            endpoint_url: String::from("/brokerage/products/"), 
            method: String::from("GET"), 
            resource: None,
        };
        let result = self.client.get(api_endpoints.endpoint_url.as_str(), api_endpoints.resource).await;

        match result {
            Ok(symbols) => Ok(symbols),
            Err(e) => bail!(format!("Error retrieving products: {:?}", e)),
        }
    }

    // returns a list of available symbols
    pub async fn get_available_symbols(&self) -> Result<Vec<String>> {
        let symbols = match self.get_available_products().await {
            Ok(symbols) => symbols,
            Err(e) => {
                bail!(e);
            }
        };

        let symbols_list: Vec<String> = symbols.products.iter().map(|f| f.product_id.clone()).collect();
        debug!("Found {} available symbols.", symbols_list.len());

        Ok(symbols_list)
    }
}