use crate::{sig_gen::create_rest_signature};

use anyhow::{bail, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Client {
    host: String,
    inner_client: reqwest::Client,
    key: String,
    secret: String,
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
    pub fn new(host: String, key: String, secret: String) -> Self {
        Client {
            host,
            inner_client: reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
            key,
            secret,
        }
    }

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

    fn extract_request_path(&self) -> Option<&str> {
        let idx = self.host.find(".com").unwrap() + ".com".len();
        Some(&self.host[idx..])
    }

    fn build_headers(&self, rmethod: &str, rpath: &str, rbody: &str) -> Result<HeaderMap> {
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

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let request_path = format!("{}{}", self.extract_request_path().unwrap(), endpoint); 

        /*
        Possible implementation of boolean header flag is by using an...
        if let Some(boolean_flag: bool) = boolean_flag {
            if boolean_flag {
                // generate get with headers
            } else {
                // generate get without headers
            }
        } 
        */
        
        let client = &self.inner_client;
        let response = client
            .get(url.as_str())
            .headers(self.build_headers("GET", request_path.as_str(), "")?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url: String = format!("{}{}", self.host, endpoint);

        let request_path = format!("{}{}", self.extract_request_path().unwrap(), endpoint); 

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers("POST", request_path.as_str(), "")?)
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