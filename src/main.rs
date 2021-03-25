#![feature(test)]

extern crate test;

use serde::{Serialize, Deserialize, Deserializer, de};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct AvgPrice {
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    mins: u64,
    #[serde(deserialize_with = "de_string_or_number_to_f64")]
    price: f64,
}

// TODO: Could these be combined and generalized into a single
//       generic implemenation over all uX and fX types?
// Convert a string or number to u64
fn de_string_or_number_to_u64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.parse::<u64>().map_err(de::Error::custom)?;
            println!("de_string_or_number_to_u64: s={} v={}", s, v);
            v
        },
        Value::Number(num) => {
            let v = num.as_u64().ok_or(de::Error::custom("Invalid number'))? as u64"))? as u64;
            println!("de_string_or_number_to_u64: num={} v={}", num, v);
            v
        },
        _ => return Err(de::Error::custom("wrong type"))
    })
}

// Convert a string or number to f64
fn de_string_or_number_to_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.parse::<f64>().map_err(de::Error::custom)?;
            println!("de_string_or_number_to_f64: s={} v={}", s, v);
            v
        },
        Value::Number(num) => {
            let v = num.as_f64().ok_or(de::Error::custom("Invalid number'))? as f64"))? as f64;
            println!("de_string_or_number_to_f64: num={} v={}", num, v);
            v
        },
        _ => return Err(de::Error::custom("wrong type"))
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        //"https://binance.us/api/v3/exchangeInfo"
        //"https://binance.us/api/v3/depth?symbol=BTCUSDT&limit=5"
        "https://binance.us/api/v3/avgPrice?symbol=BTCUSDT"
    ;

    // Some variant implementations
    match 1u8 {
        0 => {
            // Using value
            let resp_json = reqwest::Client::new()
                .get(url)
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
            let resp_json = reqwest::Client::new()
                .get(url)
                .send()
                .await?
                .text()
                .await?;
            println!("resp_json={:#?}", resp_json);

            let avg_price: AvgPrice = serde_json::from_str(&resp_json).unwrap();
            println!("avg_price={:#?}", avg_price);
        }
        2 => {
            // Separate the getting the response and converting to json
            let resp = reqwest::Client::new()
                .get(url)
                .send()
                .await?;
            println!("resp={:#?}", resp);

            let resp_json = resp.text().await?;
            println!("resp_json={:#?}", resp_json);

            let avg_price: AvgPrice = serde_json::from_str(&resp_json).unwrap();
            println!("avg_price={:#?}", avg_price);
        }
        _ => {
            Err("Bad variant")?  // From: https://stackoverflow.com/a/55125216/4812090
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[test]
    fn test_de_string_or_number_to_f64_numbers() {
        let js = r#"{ "mins": 5, "price": 1.2 }"#;
        let ap: AvgPrice = serde_json::from_str(js).expect("Error de from str");
        assert_eq!(ap.mins, 5u64);
        assert_eq!(ap.price, 1.2f64)
    }

    #[test]
    fn test_de_string_or_number_to_f64_strings() {
        let js = r#"{ "mins": "5", "price": "1.2" }"#;
        let ap: AvgPrice = serde_json::from_str(js).expect("Error de from str");
        assert_eq!(ap.mins, 5u64);
        assert_eq!(ap.price, 1.2f64)
    }

    #[bench]
    fn bench_de_string_or_number_to_f64_string_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": "5", "price": "1.2" }"#;
        b.iter(|| {
            let ap: AvgPrice = serde_json::from_str(js).expect("Error de from str");
            test::black_box(ap);
        });
    }

    #[bench]
    fn bench_de_string_or_number_to_f64_string_to_value_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": "5", "price": "1.2" }"#;
        b.iter(|| {
            let jv = serde_json::from_str(js).expect("Error de from str");
            let ap: AvgPrice = serde_json::from_value(jv).expect("Error de from str");
            test::black_box(ap);
        });
    }

    #[bench]
    /// TODO: Why is this slower than `bench_de_string_or_number_to_f64_string_to_struct`?
    fn bench_de_string_or_number_to_f64_numbers_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": 5, "price": 1.2 }"#;
        b.iter(|| {
            let ap: AvgPrice = serde_json::from_str(js).expect("Error de from str");
            test::black_box(ap);
        });
    }

    #[bench]
    /// TODO: Why is this slower than `bench_de_string_or_number_to_f64_value_to_struct`?
    fn bench_de_string_or_number_to_f64_numbers_to_value_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": 5, "price": 1.2 }"#;
        b.iter(|| {
            let jv = serde_json::from_str(js).expect("Error de from str");
            let ap: AvgPrice = serde_json::from_value(jv).expect("Error de from str");
            test::black_box(ap);
        });
    }
}