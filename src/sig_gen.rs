use hmac::{Hmac, Mac};
use sha2::Sha256;

// sign your requests per coinbase 
// https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-auth
// maybe this shoudld be an async method
pub fn create_rest_signature(
    ts: &str, 
    method: &str, 
    request_path: &str, 
    body: &str, 
    secret_key: &[u8]
) -> String {
    type HmacSha256 = Hmac<Sha256>;

    let signature_string = format!("{}{}{}{}", ts, method, request_path, body);
    let mut mac = HmacSha256::new_from_slice(secret_key).unwrap();
    mac.update(signature_string.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

// websocket sig gen is a little different
// https://docs.cloud.coinbase.com/advanced-trade-api/docs/ws-overview#subscribe
pub fn create_ws_signature(ts: String, channel: String, products: Vec<String>, secret_key: &[u8]) -> String {
    type HmacSha256 = Hmac<Sha256>;

    let products_string = products.join(",");
    let signature_string: String = format!("{}{}{}", ts, channel, products_string);
    let mut mac = HmacSha256::new_from_slice(secret_key).unwrap();
    mac.update(signature_string.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}