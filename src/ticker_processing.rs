use serde::Serialize;

#[derive(Serialize)]
struct TickData {
    channel: String,
    client_id: String, 
    timestamp: String,
    sequence_num: String,
    msg_type: String,
    product_id: String,
    price: String,
    volume_24_h: String,
    low_24_h: String,
    high_24_h: String,
    low_52_w: String,
    high_52_w: String,
    price_percent_chg_24_h: String,
}

async fn process_data() {
    // process the data
    // write to file
}

async fn write_batch_to_file() {
    // writes the batches to a file
}