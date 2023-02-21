use crate::{advanced_trade_rest_client::AdvancedTradeRESTClient, config_builder::CoinbaseConfig, models, websocket, sig_gen};
use anyhow::{bail, Result};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::{stream::MaybeTlsStream, Message};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AdvancedTradeEvents {
    GenericEvent(models::GenericMessage),
    ErrorEvent(models::ErrorMesage),
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SubscribeProducts {
    All, 
    Custom(Vec<String>),
}


pub struct AdvancedTradeWebSockets {
    exchange: String,
    channels: Vec<String>,
    product_ids: SubscribeProducts,
    client: AdvancedTradeRESTClient,
    key: String,
    secret: String,
}

impl AdvancedTradeWebSockets {
    pub fn new(
        channels: Vec<String>, 
        product_ids: SubscribeProducts, 
    ) -> AdvancedTradeWebSockets {
        let config = CoinbaseConfig::new();

        AdvancedTradeWebSockets {
            exchange: "coinbase-advanced-trade".to_string(),
            channels,
            product_ids,
            client: AdvancedTradeRESTClient::new("https://api.coinbase.com/api/v3"),
            key: config.api_key,
            secret: config.api_secret,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let keep_running = AtomicBool::new(true);

        if let Err(e) = self.event_loop(&keep_running).await {
            error!("Error: {}", e);
        }
        info!("[{}] Loop stopped running", &self.exchange);

        Ok(())
    }

    async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        // get connected 
        info!("Establishing connection...");
        let mut socket = match self.connect().await {
            Ok(socket_ok) => {
                socket_ok
            },
            Err(e) => {
                bail!("Error: {}", e)
            }
        };

        // engage event loop
        info!("Starting event loop...");
        while running.load(Ordering::Relaxed) {
            // get messages
            let message = match socket.0.read_message() {
                Ok(msg) => {
                    debug!("{:?}", msg);
                    msg
                },
                Err(e) => {
                    error!("Error: {}", e);
                    info!("[{}] Reconnecting WebSocket due to error.", &self.exchange);
                    socket = match self.connect().await {
                        Ok(socket) => socket,
                        Err(e) => {
                            bail!("Error: {}", e)
                        }
                    };
                    continue;
                }
            };

            // handle messages
            match message {
                Message::Text(msg) => {
                    if let Err(e) = self.handle_msg(&msg, &mut socket.0).await {
                        error!("Error on handling stream message: {}", e);
                        continue;
                    }
                },
                // We can ignore these message because tungstenite takes care of them for us.
                Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => (),
                Message::Close(e) => {
                    error!("Disconnected {:?}", e);
                    continue;
                },
                // throwing a catch just in case
                m => {
                    bail!(format!("Received unhandled message of type: {:?}", m))
                }
            }
        }
        socket.0.close(None)?;
        Ok(())
    }

    async fn handle_msg(
        &self, 
        msg: &str, 
        socket: &mut WebSocket<MaybeTlsStream<TcpStream>>
    ) -> Result<()> {
        let mut advanced_trade_event: AdvancedTradeEvents = match serde_json::from_str(msg) {
            Ok(deserialized_event) => deserialized_event,
            Err(e) => {
                error!("Error unpacking advanced trade websocket event: {:?}", e);
                AdvancedTradeEvents::Unknown
            },
        };

        match advanced_trade_event {
            AdvancedTradeEvents::GenericEvent(event) => {
                info!("{:?}", event);
            },
            AdvancedTradeEvents::ErrorEvent(event) => {
                error!("Error message encountered: {:?}", event);
            },
            AdvancedTradeEvents::Unknown => {
                debug!("Unknown event encounteres")
            }

        }
        Ok(())
    }

    async fn subscribe_to_channel(&self, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
        let products: Vec<String> = match &self.product_ids {
            SubscribeProducts::All => {
                self.client.get_available_symbols().await.unwrap()
            },
            SubscribeProducts::Custom(products) => {
                products.clone()
            }
        };

        let mut channels_clone = self.channels.clone();
        let mut current_channel = channels_clone.pop();
        
        while let Some(channel) = current_channel.clone() {
            for product in &products {
                if socket.can_write() {
                    let subscribe_channel = format!("{}{}", channel, product);
                    info!(
                        "[{}] Subscribing to [{}] for product: {}", 
                        &self.exchange, 
                        channel,
                        product
                    );
                    // generate signature
                    let current_ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
                    let msg = models::ChannelSubscriptionMessage {
                        msg_type: "subscribe".to_string(),
                        product_ids: vec![product.clone()],
                        channel: channel.clone(),
                        api_key: self.key.clone(),
                        timestamp: current_ts.clone(),
                        signature: sig_gen::create_ws_signature(
                            current_ts, 
                            channel.clone(), 
                            vec![product.clone()], 
                            self.secret.clone().as_bytes()
                        )
                    };
                    let json = serde_json::to_string(&msg).unwrap();
                    let message = Message::Text(json);
                    match socket.write_message(message) {
                        Ok(_) => {
                            std::thread::sleep(std::time::Duration::from_millis(100));
                            continue;
                        },
                        Err(e) => {
                            error!("Error occurred for symbol: {}", product);
                            error!("Error: {}", e);
                            continue;
                        }
                    }
                } else {
                    error!("Cannot write to socket.")
                }
            }
            current_channel = channels_clone.pop();
        }
    }

    // just "finished" the above code block

    async fn connect(&mut self) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, Response)> {
        let mut websocket_urls = Vec::new();
        websocket_urls.push("wss://advanced-trade-ws.coinbase.com".to_string());

        if let Ok(con) = websocket::connect_wss(&self.exchange, websocket_urls) {
            return Ok(con);
        }

        bail!("Unable to connect.");
    }

}