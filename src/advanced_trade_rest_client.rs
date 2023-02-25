use crate::{rest_client::Client, models::{RestEndpoint, Products}, config_builder::CoinbaseConfig, sig_gen::create_rest_signature};
use anyhow::{bail, Result};
use log::{debug}; // removed `error`
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::{SystemTime, UNIX_EPOCH};

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
            ),
            key: config.api_key,
            secret: config.api_secret,
        }
    }

    /// Builds the headers based on the Coinbase Advanced Trade API specification
    /// 
    /// Builds a `HeaderMap` of the following values:
    /// * `CB-ACCESS-KEY`: User Coinbase API Key as pulled from the `CoinbaseConfig`
    /// * `CB-ACCESS-SIGN`: User generated signature which is further described by `sig_gen.rs`
    /// * `CB-ACCESS-TIMESTAMP`: System timestamp pulled from `SystemTime`
    /// 
    /// # Arguments
    /// * `rmethod`: REST method e.g. `"GET"`, `"POST"`
    /// * `rpath`: The request path + the REST endpoints e.g `"/api/v3/brokerage/products"
    /// * `rbody`: Relevant request body - in our case, `None`
    /// 
    /// # Returns
    /// 
    /// `Result<HeaderMap>` which can simply be applied later in a `.build_headers()` context
    pub fn build_headers_with_signature(&self, rmethod: &str, rpath:&str, rbody: &str) -> Result<HeaderMap> {
        // signature creation
        let rts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
        let signature = create_rest_signature(
            rts.as_str(), 
            rmethod, 
            rpath, 
            rbody, 
            self.secret.as_bytes());
        let sign = signature.as_str();
        
        // build out request headers
        let mut custom_headers = HeaderMap::new(); 

        custom_headers.insert("CB-ACCESS-KEY", HeaderValue::from_str(self.key.as_str()).unwrap());
        custom_headers.insert("CB-ACCESS-SIGN", HeaderValue::from_str(sign).unwrap());
        custom_headers.insert("CB-ACCESS-TIMESTAMP", HeaderValue::from_str(rts.as_str()).unwrap());

        Ok(custom_headers)
    }

    // returns all product information
    pub async fn get_available_products(&self) -> Result<Products> {
        let api_endpoints: RestEndpoint = RestEndpoint{ 
            endpoint_url: String::from("/brokerage/products/"), 
            method: String::from("GET"), 
            resource: None,
        };
        let header_map = match self.build_headers_with_signature(
            &api_endpoints.method, 
            format!("{}{}", self.client.extract_request_path().unwrap(), &api_endpoints.endpoint_url).as_str(), 
            ""
        ) {
            Ok(header_map) => header_map,
            Err(_) => {
                bail!("Ill-defined headers");
            }
        };

        let result = self.client
            .get(
                api_endpoints.endpoint_url.as_str(), 
                header_map,
                api_endpoints.resource)
            .await;

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