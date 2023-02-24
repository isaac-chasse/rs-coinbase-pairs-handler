use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt;

/// Generic REST API Client
pub struct Client {
    host: String,
    inner_client: reqwest::Client,
}

/* 
TODO: Abstract out the key / secret parameters. this should sit at the AdvancedTrade level
    and does not make for an idiomatic Client implementation since we are doing AdvancedTr
    -ade specific tasks like:
        > build_headers -- specific header construction is relevant to AdvancedTrade scope
        > we can implement a generic header handler though -- have it sit in Client and is
            used to handle headers that are generated in the AdvancedTrade scope
*/
impl Client {
    /// Implements a new function for the REST API Client with a host and inner client
    /// `inner_client` is automatically constructed with the `reqwest` library using `reqwest::Client::builder()`
    pub fn new(host: String) -> Self {
        Client {
            host,
            inner_client: reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    /// Implemention of a response handler function.
    /// Matches to the following set of expected responses: `StatusCode::OK`, `StatusCode::INTERNAL_SERVER_ERROR`,
    /// `StatusCode::SERVICE_UNAVAILABLE`, `StatusCode::UNAUTHORIZED`, `StatusCode::BAD_REQUEST`
    /// Defaults to a `bail!` with relevant response message for uncovered StatusCodes.
    async fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?),
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }, 
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }, 
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }, 
            StatusCode::BAD_REQUEST => {
                let error: ContentError = response.json().await?;
                bail!(error)
            }, 
            s => {
                bail!(format!("Received response: {:?}", s))
            }
        }
    }

    /// Extracts the request path from the `host` parameter
    /// 
    /// Utilizes string search for `".com"` within the `host` parameter and returns the string to the 
    /// right of the `".com"` index
    /// 
    /// # Returns
    /// 
    /// `Option<&str>` - which is the optional reference to the string described after `".com"`
    /// 
    /// # Example
    /// 
    /// TODO: Insert an example here
    pub fn extract_request_path(&self) -> Option<&str> {
        let idx = self.host.find(".com").unwrap() + ".com".len();
        Some(&self.host[idx..])
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str, headers: HeaderMap, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if let Some(request) = request {  // handle request payload if necessary
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client
            .get(url.as_str())
            .headers(headers)
            .send()
            .await?;

        self.handler(response).await
    }

    #[allow(dead_code)]
    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str, headers: HeaderMap) -> Result<T> {
        let url: String = format!("{}{}", self.host, endpoint);
        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(headers)
            .send()
            .await?;

        self.handler(response).await
    }

}

#[derive(Debug, Deserialize)]
pub struct ContentError {
    pub code: i16,
    pub msg: String,
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {} \nmsg: {}", self.code, self.msg) // human readable error message
    }
}