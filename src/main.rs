#![feature(test)]

#[macro_use]
extern crate hex_literal;

mod de_string_or_number;
use de_string_or_number::{de_string_or_number_to_f64, de_string_or_number_to_u64};

mod signature_binance;

use reqwest;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AvgPrice {
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    mins: u64,
    #[serde(deserialize_with = "de_string_or_number_to_f64")]
    price: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        //"https://binance.us/api/v3/exchangeInfo"
        //"https://binance.us/api/v3/depth?symbol=BTCUSDT&limit=5"
        //"https://binance.us/api/v3/avgPrice?symbol=BTCUSDT"
        //"https://api.binance.us/api/v3/avgPrice?symbol=BTCUSDT"
        "https://binance.us/api/v3/avgPrice?symbol=BTCUSDT"
    ;

    // Some variant implementations
    match 3u8 {
        0 => {
            // Using value
            let resp_json = reqwest::Client::new()
                .post(url)
                .send()
                .await?
                .json()
                .await?;
            println!("resp_json={:#?}", resp_json);

            let avg_price: AvgPrice = serde_json::from_value(resp_json).unwrap();
            println!("avg_price={:#?}", avg_price);
        }
        1 => {
            // Using text, this would seem to require less processing?
            let resp_json = reqwest::Client::new().get(url).send().await?.text().await?;
            println!("resp_json={:#?}", resp_json);

            let avg_price: AvgPrice = serde_json::from_str(&resp_json).unwrap();
            println!("avg_price={:#?}", avg_price);
        }
        2 => {
            // Separate the getting the response and converting to json

            let client = reqwest::Client::new();
            let req_builder = client.get(url);
            println!("req_builder={:#?}", req_builder);

            let resp = req_builder.send().await?;
            println!("resp={:#?}", resp);

            let resp_json = resp.text().await?;
            println!("resp_json={:#?}", resp_json);

            let avg_price: AvgPrice = serde_json::from_str(&resp_json).unwrap();
            println!("avg_price={:#?}", avg_price);
        }
        3 => {
            // Use a proxy so we use wireshark to see the traffic
            let resp = reqwest::Client::builder()
                .proxy(reqwest::Proxy::https("http://localhost:8080")?)
                .build()?
                .post(url)
                .send()
                .await;
            println!("resp={:#?}", resp);

            match resp {
                Ok(response) => {
                    if response.status() == 200 {
                        let resp_json = response.text().await?;
                        println!("resp_json={:#?}", resp_json);

                        let avg_price: AvgPrice = serde_json::from_str(&resp_json).unwrap();
                        println!("avg_price={:#?}", avg_price);
                    } else {
                        println!("response status={}", response.status());
                    }
                }
                Err(err) => println!("err: {}", err),
            }
        }
        _ => {
            Err("Bad variant")? // From: https://stackoverflow.com/a/55125216/4812090
        }
    }

    Ok(())
}
