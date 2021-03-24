#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        //"https://binance.us/api/v3/exchangeInfo"
        //"https://binance.us/api/v3/depth?symbol=BTCUSDT&limit=5"
        "https://binance.us/api/v3/avgPrice?symbol=BTCUSDT"
    ;
    let resp_json: serde_json::Value = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", resp_json);

    Ok(())
}
