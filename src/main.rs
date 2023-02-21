use log::{info};
use log4rs;

mod advanced_trade_rest_client;
mod advanced_trade_websocket;
mod config_builder;
mod models;
mod rest_client;
mod sig_gen;
mod websocket;

#[tokio::main]
async fn main() {
    // establish logging
    log4rs::init_file("logconfig.yml", Default::default())
        .expect("Log config file not found.");
    info!("We now have nice logging!");

    // api testing
    let client = advanced_trade_rest_client::AdvancedTradeRESTClient::new("https://api.coinbase.com/api/v3");
    let result = client.get_available_symbols().await.unwrap();
    info!("Coinbase available symbols: {:?}", result);

    // let mut coinbase_advanced_trade = advanced_trade_websocket::AdvancedTradeWebSockets::new(
    //     vec!["".to_string()],
    //     advanced_trade_websocket::SubscribeProducts::All,
    // );
    // coinbase_advanced_trade.run().await.unwrap();
}
