#![feature(test)]

#[macro_use]
extern crate hex_literal;

mod de_string_or_number;
use de_string_or_number::{de_string_or_number_to_f64, de_string_or_number_to_u64};

mod signature_binance;

use signature_binance::{binance_signature, query_vec_u8};

use hex;
use reqwest;

use serde::{Deserialize, Serialize};

const TEST_BY_WINK20210309: &[u8] =
    b"sx6bFVKosMPoNsX3OHadmYfXGxkU9s7XRKIw8FwvNIGLFBJcciUaOxxrzoBkwPKq";

fn test() {
    // query_params, sig_key and expected from:
    //   https://github.com/binance-us/binance-official-api-docs/blob/5a1dd14437bd3be4631778e78d3203014cf30b63/rest-api.md#example-3-mixed-query-string-and-request-body
    let query_params = vec![
        ("symbol", "LTCBTC"),
        ("side", "BUY"),
        ("type", "LIMIT"),
        ("timeInForce", "GTC"),
    ];

    let body_params = vec![
        ("quantity", "1"),
        ("price", "0.1"),
        ("recvWindow", "5000"),
        ("timestamp", "1499827319559"),
    ];
    let sig_key = b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
    let expected = hex!("0fd168b8ddb4876a0358a8d14d0c9f3da0e9b20c5d52b2a00fcf7d1c602f9a77");

    let qs = query_vec_u8(&query_params);
    let body = query_vec_u8(&body_params);

    // Calculate the signature from the data and key
    let signature = binance_signature(sig_key, &qs, &body);
    println!("signature= {}", hex::encode(signature));

    let public_key = TEST_BY_WINK20210309.to_vec();
    let secret_key = match std::env::var("SECRET_KEY") {
        Ok(val) => val.into_bytes(),
        Err(e) => {
            println!("Secret key Err: {}", e);
            vec![]
        }
    };
    println!(
        "secret_key={}",
        String::from_utf8(secret_key.clone()).unwrap()
    );
    println!(
        "public_key={}",
        String::from_utf8(public_key.clone()).unwrap()
    );

    // Validate
    assert_eq!(signature.len(), 32);
    assert_eq!(signature, expected);
}

#[derive(Debug, Deserialize, Serialize)]
struct AvgPrice {
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    mins: u64,
    #[serde(deserialize_with = "de_string_or_number_to_f64")]
    price: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path =
        "/api/v3/avgPrice?symbol=BTCUSDT"
        //"/api/v3/depth?symbol=BTCUSDT&limit=5"
        //"/api/v3/exchangeInfo"
    ;

    test();

    // Some variant implementations
    match 2u8 {
        0 => {
            // For POST's use binance.us
            let url = "https://binance.us".to_string() + path;

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
            // For GET's use api.binance.us so as to eliminate the 301 redirect error
            let url = "https://api.binance.us".to_string() + path;

            let resp_json = reqwest::Client::new().get(url).send().await?.text().await?;
            println!("resp_json={:#?}", resp_json);

            let avg_price: AvgPrice = serde_json::from_str(&resp_json).unwrap();
            println!("avg_price={:#?}", avg_price);
        }
        2 => {
            // Separate the getting the response and converting to json
            // For GET's use api.binance.us so as to eliminate the 301 redirect error
            let url = "https://api.binance.us".to_string() + path;

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
            // For POST's use binance.us
            let url = "https://binance.us".to_string() + path;

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
