#![feature(test)]

use chrono::prelude::*;
use hex;
use reqwest;
use serde::{Deserialize, Serialize};

mod de_string_or_number;
use de_string_or_number::{de_string_or_number_to_f64, de_string_or_number_to_u64};

mod signature_binance;
use signature_binance::{binance_signature, query_vec_u8};

#[derive(Debug, Deserialize, Serialize)]
struct AvgPrice {
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    mins: u64,
    #[serde(deserialize_with = "de_string_or_number_to_f64")]
    price: f64,
}

fn time_ms_utc_now() -> i64 {
    let now = Utc::now();
    now.timestamp_millis()
}

fn append_signature(query: &mut Vec<u8>, signature: [u8; 32]) {
    let signature_string = hex::encode(&signature);
    println!("signature_string={}", signature_string);

    let signature_params = vec![("signature", signature_string.as_str())];
    println!("signature_params={:?}", signature_params);
    query.append(&mut vec![b'&']);
    query.append(&mut query_vec_u8(&signature_params));
}

fn get_env_var(key: &str) -> String {
    let value = match std::env::var(key) {
        Ok(val) => {
            if val.len() > 0 {
                val
            } else {
                panic!("env var \"{}\" is empty", key);
            }
        }
        Err(e) => {
            panic!("env var \"{}\" Err: {}", key, e);
        }
    };

    value
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path =
        "/api/v3/avgPrice?symbol=BTCUSDT"
        //"/api/v3/depth?symbol=BTCUSDT&limit=5"
        //"/api/v3/exchangeInfo"
    ;

    // Some variant implementations
    match 5u8 {
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
        4 => {
            println!("Order Test");

            let sig_key = get_env_var("SECRET_KEY").into_bytes();
            let api_key = get_env_var("API_KEY");

            let mut params = vec![
                ("symbol", "BTCUSD"),
                ("side", "BUY"),
                ("type", "MARKET"),
                ("quantity", "0.0002"),
                ("recvWindow", "5000"),
            ];
            let ts_string: String = format!("{}", time_ms_utc_now());
            params.append(&mut vec![("timestamp", ts_string.as_str())]);

            let mut query = query_vec_u8(&params);

            // Calculate the signature using sig_key and the data is qs and query as body
            let signature = binance_signature(&sig_key, &vec![], &query);

            // Append the signature to query
            append_signature(&mut query, signature);

            // Convert to a string
            let query_string = String::from_utf8(query).unwrap();
            println!("query_string={}", &query_string);

            let path = "/api/v3/order/test";
            let url = "https://api.binance.us".to_string() + path;

            // Build request
            let client = reqwest::Client::builder();
            let req_builder = client
                .proxy(reqwest::Proxy::https("http://localhost:8080")?)
                .build()?
                .post(url)
                .header("X-MBX-APIKEY", api_key)
                .body(query_string);
            println!("req_builder={:#?}", req_builder);

            // Send and get response
            let resp = req_builder.send().await;
            println!("resp={:#?}", &resp);
            match resp {
                Ok(response) => {
                    if response.status() == 200 {
                        let resp_json = response.text().await?;
                        println!("resp_json={}", resp_json);
                    } else {
                        println!("response status={}", response.status());
                    }
                }
                Err(err) => println!("err: {}", err),
            }
        }
        5 => {
            println!("Get Account Information");

            let sig_key = get_env_var("SECRET_KEY").into_bytes();
            let api_key = get_env_var("API_KEY");

            let mut params = vec![];
            let ts_string: String = format!("{}", time_ms_utc_now());
            params.append(&mut vec![("timestamp", ts_string.as_str())]);

            let mut query = query_vec_u8(&params);

            // Calculate the signature using sig_key and the data is qs and query as body
            let signature = binance_signature(&sig_key, &vec![], &query);

            // Append the signature to query
            append_signature(&mut query, signature);

            // Convert to a string
            let query_string = String::from_utf8(query).unwrap();
            println!("query_string={}", &query_string);

            let path = "/api/v3/account";
            let url = "https://api.binance.us".to_string() + path + "?" + &query_string;

            // Build request
            let client = reqwest::Client::builder();
            let req_builder = client
                .proxy(reqwest::Proxy::https("http://localhost:8080")?)
                .build()?
                .get(url)
                .header("X-MBX-APIKEY", api_key);
            println!("req_builder={:#?}", req_builder);

            // Send and get response
            let resp = req_builder.send().await;
            println!("resp={:#?}", &resp);
            match resp {
                Ok(response) => {
                    if response.status() == 200 {
                        let resp_json = response.text().await;
                        match resp_json {
                            Ok(resp) => {
                                let val: Result<serde_json::Value, _> = serde_json::from_str(&resp);
                                match val {
                                    Ok(json_val) => println!(
                                        "json_val={}",
                                        serde_json::to_string_pretty(&json_val).unwrap()
                                    ),
                                    Err(e) => println!("json_val Err e={}", e),
                                }
                            }
                            Err(e) => println!("Error processing response: e={}", e),
                        }
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
