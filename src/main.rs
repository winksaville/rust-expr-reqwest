use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://binance.us/api/v3/time";
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, u64>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
