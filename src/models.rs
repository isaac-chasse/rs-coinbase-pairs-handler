use serde::{Deserialize, Serialize};

/*
REST - Models that store REST requests
*/
#[derive(Debug, Deserialize, Clone)]
pub struct RestEndpoint {
    pub endpoint_url: String,
    pub method: String,
    pub resource: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Products {
    pub num_products: i64,
    pub products: Vec<ProductData>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ProductData {
    pub product_id: String,
    pub product_type: String,
    pub base_currency_id: String,
    pub base_increment: String,
    pub base_max_size: String,
    pub base_min_size: String,
    pub quote_currency_id: String,
    pub quote_increment: String,
    pub quote_max_size: String,
    pub quote_min_size: String,
    pub status: String,
    pub trading_disabled: bool,
}

/*
WEBSOCKETS - Models for handling websocket messages

TODO: For some reason all the UpdateTickers are being classified as SnapshotTickers during run time
    I should look into fixing this to ensure type safety and proper API usage
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum WebsocketEvent {
    SnapshotEvent(SnapshotMessage),
    UpdateEvent(UpdateMessage),
    SubscriptionEvent(SubscriptionMessage),
    Unkown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotTicker {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub product_id: String,
    pub price: String,
    pub volume_24_h: String,
    pub low_24_h: String,
    pub high_24_h: String,
    pub low_52_w: String,
    pub high_52_w: String,
    pub price_percent_chg_24_h: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub tickers: Vec<SnapshotTicker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTicker {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub product_id: String,
    pub price: String,
    pub volume_24_h: String,
    pub low_24_h: String,
    pub high_24_h: String,
    pub low_52_w: String,
    pub high_52_w: String,
    pub price_percent_chg_24_h: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub tickers: Vec<SnapshotTicker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionMessage {
    pub subscriptions: Subscriptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscriptions {
    pub ticker: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<WebsocketEvent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorMesage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub message: String,
}

// actually not sure if this is correct impl
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelSubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub product_ids: Vec<String>,
    pub channel: String, 
    pub api_key: String,
    pub timestamp: String,
    pub signature: String,
}