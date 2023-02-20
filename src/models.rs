use serde::{Deserialize, Serialize};

/*
REST - Models that store REST requests
TODO: Implement a boolean header flag in a channel struct
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
    msg_type: String,
    tickers: Vec<SnapshotTicker>,
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

/*
JSON request example for "ticker" channel
{
    "type": "subscribe", 
    "product_ids": [
        "ETH-USD"
        ], 
    "channel": "ticker", 
    "api_key": "KaHjgMpnvTV8rl1r", 
    "timestamp": "1675836190", 
    "signature": "3a576f746dfd8d4c52c890095ae2ec7880b807bc8062ae87c020e88ddbbfad7e"
}
JSON response example for "ticker" channel
// ERROR
{
	"type": "error",
	"message": "authentication failure"
}
// SUBSCRIPTIONS
{
	"channel": "subscriptions",
	"client_id": "",
	"timestamp": "2023-02-08T06:12:20.838432067Z",
	"sequence_num": 1,
	"events": [
		{
			"subscriptions": {
				"ticker": [
					"ETH-USD"
				]
			}
		}
	]
}
// SNAPSHOT
{
	"channel": "ticker",
	"client_id": "",
	"timestamp": "2023-02-08T06:12:20.838410617Z",
	"sequence_num": 0,
	"events": [
		{
			"type": "snapshot",
			"tickers": [
				{
					"type": "ticker",
					"product_id": "ETH-USD",
					"price": "1675.14",
					"volume_24_h": "185976.72526638",
					"low_24_h": "1624.53",
					"high_24_h": "1699.66",
					"low_52_w": "879.8",
					"high_52_w": "3581.6",
					"price_percent_chg_24_h": "2.48889541499945"
				}
			]
		}
	]
}
// UPDATE
{
	"channel": "ticker",
	"client_id": "",
	"timestamp": "2023-02-08T06:12:21.708730964Z",
	"sequence_num": 2,
	"events": [
		{
			"type": "update",
			"tickers": [
				{
					"type": "ticker",
					"product_id": "ETH-USD",
					"price": "1675.01",
					"volume_24_h": "185976.72526638",
					"low_24_h": "1624.53",
					"high_24_h": "1699.66",
					"low_52_w": "879.8",
					"high_52_w": "3581.6",
					"price_percent_chg_24_h": "2.48094171775388"
				}
			]
		}
	]
}
*/