#![feature(test)]

use chrono::prelude::*;
use hex;
use reqwest;
use serde::{Deserialize, Serialize};

use strum_macros::IntoStaticStr;

mod de_string_or_number;
use de_string_or_number::{de_string_or_number_to_f64, de_string_or_number_to_u64};

mod signature_binance;
use signature_binance::{binance_signature, query_vec_u8};

mod account_info;
use account_info::AccountInfo;

mod order_response;
use order_response::OrderResponse;

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
    const DEBUG: bool = false;
    let signature_string = hex::encode(&signature);
    if DEBUG {
        println!("signature_string={}", signature_string);
    }

    let signature_params = vec![("signature", signature_string.as_str())];
    if DEBUG {
        println!("signature_params={:?}", signature_params);
    }
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

#[derive(IntoStaticStr)]
pub enum Side {
    BUY,
    SELL,
}

async fn binance_market_order_or_test(
    symbol: &str,
    side: Side,
    quantity: f64,
    test: bool,
) -> Result<OrderResponse, Box<dyn std::error::Error>> {
    const DEBUG: bool = false;

    let sig_key = get_env_var("SECRET_KEY").into_bytes();
    let api_key = get_env_var("API_KEY");

    let side_str: &str = side.into();
    let quantity_str: &str = &format!("{}", quantity);

    println!(
        "binance_market_order_or_test: symbol={} side={} quantity={} test={}",
        symbol, side_str, quantity_str, test
    );

    let mut params = vec![
        ("symbol", symbol),
        ("side", side_str),
        ("type", "MARKET"),
        ("quantity", quantity_str),
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
    if DEBUG {
        println!("query_string={}", &query_string);
    }

    let path = if test {
        "/api/v3/order/test"
    } else {
        "/api/v3/order"
    };
    let url = "https://api.binance.us".to_string() + path;

    // Build request
    let client = reqwest::Client::builder();
    let req_builder = client
        .proxy(reqwest::Proxy::https("http://localhost:8080")?)
        .build()?
        .post(url)
        .header("X-MBX-APIKEY", api_key)
        .body(query_string);
    if DEBUG {
        println!("req_builder={:#?}", req_builder);
    }

    // Send and get response
    let response = req_builder.send().await?;
    if DEBUG {
        println!("response={:#?}", &response);
    }
    let response_status = response.status();
    let response_body = response.text().await?;
    if response_status == 200 {
        if DEBUG {
            println!("response_body={}", response_body);
        }
        let mut order_resp = OrderResponse::default();
        if !test {
            order_resp = serde_json::from_str(&&response_body)?;
        } else {
            order_resp.test = true;
        }
        if DEBUG {
            println!(
                "binance_market_order_or_test: symbol={} side={} quantity={} test={} order_response={:#?}",
                symbol, side_str, quantity_str, test, order_resp
            );
        }
        Ok(order_resp)
    } else {
        Err(format!(
            "Error response status={} symbol={} side={} quantity={} body={}",
            response_status, symbol, side_str, quantity_str, response_body
        ))?
    }
}

//async fn binance_market_order(symbol: &str, side: Side, quantity: f64) -> Result<(), Box<dyn std::error::Error>> {
//    //return binance_market_order_or_test(symbol, side, quantity, false);
//    Ok(())
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path =
        "/api/v3/avgPrice?symbol=BTCUSDT"
        //"/api/v3/depth?symbol=BTCUSDT&limit=5"
        //"/api/v3/exchangeInfo"
    ;

    // Some variant implementations
    let variant = 255;
    match variant {
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
            println!("Get Exchange Information");
            let url = "https://api.binance.us/api/v3/exchangeInfo".to_string();

            // Using value
            let resp_json = reqwest::Client::new()
                .get(url)
                .send()
                .await?
                .text()
                .await?;
            println!("resp_json={:#?}", resp_json);

            let ei: serde_json::Value = serde_json::from_str(&resp_json).unwrap();
            println!("ei={:#?}", ei);
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
                    let response_status = response.status();
                    let response_body = response.text().await?;
                    if response_status == 200 {
                        let account_info: AccountInfo = match serde_json::from_str(&response_body) {
                            Ok(info) => info,
                            Err(e) => panic!(
                                "Error converting body to AccountInfo: e={} body={}",
                                e, response_body
                            ),
                        };
                        println!("account_info={:#?}", account_info);
                    } else {
                        println!("response status={} body={}", response_status, response_body);
                    }
                }
                Err(err) => println!("err: {}", err),
            }
        }
        6 => {
            println!("Order Test success");
            let order_response =
                binance_market_order_or_test("BNBUSD", Side::BUY, 0.03, true).await?;
            println!("Order Test success BNBUSD response={:#?}", order_response);
        }
        7 => {
            println!("Order Test failure");
            binance_market_order_or_test("BNB", Side::BUY, 0.03, true).await?;
            panic!("Should NEVER get here!");
        }
        254 => {
            println!("Buy Order BNBUSD");
            let order_response =
                binance_market_order_or_test("BNBUSD", Side::BUY, 0.04, false).await?;
            println!("Buy Order BNBUSD response={:#?}", order_response);
        }
        255 => {
            println!("Sell Order BNBUSD");
            let order_response =
                binance_market_order_or_test("BNBUSD", Side::SELL, 0.06, false).await?;
            println!("Sell Order BNBUSD response={:#?}", order_response);
        }
        _ => {
            Err("Bad variant")? // From: https://stackoverflow.com/a/55125216/4812090
        }
    }

    Ok(())
}
