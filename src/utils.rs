use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct MinsPrice {
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    mins: u64,
    #[serde(deserialize_with = "de_string_or_number_to_f64")]
    price: f64,
}

// TODO: Could these be combined and generalized into a single
//       generic implemenation over all iX, uX and fX numeric types?
//
// Convert a string or number to u64
pub fn de_string_or_number_to_u64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.parse::<u64>().map_err(de::Error::custom)?;
            // println!("de_string_or_number_to_u64: s={} v={}", s, v);
            v
        }
        Value::Number(num) => {
            let v = num
                .as_u64()
                .ok_or(de::Error::custom("Invalid number as_u64"))?;
            // println!("de_string_or_number_to_u64: num={} v={}", num, v);
            v
        }
        _ => return Err(de::Error::custom("Expecting String or Number")),
    })
}

// Convert a string or number to f64
pub fn de_string_or_number_to_f64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => {
            let v = s.parse::<f64>().map_err(de::Error::custom)?;
            // println!("de_string_or_number_to_f64: s={} v={}", s, v);
            v
        }
        Value::Number(num) => {
            let v = num
                .as_f64()
                .ok_or(de::Error::custom("Invalid number as_f64"))?;
            // println!("de_string_or_number_to_f64: num={} v={}", num, v);
            v
        }
        _ => return Err(de::Error::custom("Expecting String or Number")),
    })
}

#[cfg(test)]
mod tests {
    // rust-anaylzer incorrectly reports test as unresolved:
    //   https://github.com/rust-analyzer/rust-analyzer/issues/6714
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_de_string_or_number_from_numbers() {
        let js = r#"{ "mins": 5, "price": 1.2 }"#;
        let ap: MinsPrice = serde_json::from_str(js).expect("Error de from str");
        assert_eq!(ap.mins, 5u64);
        assert_eq!(ap.price, 1.2f64)
    }

    #[test]
    fn test_de_string_or_number_from_strings() {
        let js = r#"{ "mins": "5", "price": "1.2" }"#;
        let ap: MinsPrice = serde_json::from_str(js).expect("Error de from str");
        assert_eq!(ap.mins, 5u64);
        assert_eq!(ap.price, 1.2f64)
    }

    #[bench]
    fn bench_de_string_from_str_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": "5", "price": "1.2" }"#;
        b.iter(|| {
            let ap: MinsPrice = serde_json::from_str(js).expect("Error de from str");
            test::black_box(ap);
        });
    }

    #[bench]
    fn bench_de_string_from_value_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": "5", "price": "1.2" }"#;
        b.iter(|| {
            let jv = serde_json::from_str(js).expect("Error de from str");
            let ap: MinsPrice = serde_json::from_value(jv).expect("Error de from str");
            test::black_box(ap);
        });
    }

    #[bench]
    /// TODO: Why is this slower than `bench_de_string_from_str_to_struct`
    fn bench_de_number_from_str_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": 5, "price": 1.2 }"#;
        b.iter(|| {
            let ap: MinsPrice = serde_json::from_str(js).expect("Error de from str");
            test::black_box(ap);
        });
    }

    #[bench]
    /// TODO: Why is this slower than `bench_de_string_from_value_to_struct`
    fn bench_de_number_from_value_to_struct(b: &mut Bencher) {
        let js = r#"{ "mins": 5, "price": 1.2 }"#;
        b.iter(|| {
            let jv = serde_json::from_str(js).expect("Error de from str");
            let ap: MinsPrice = serde_json::from_value(jv).expect("Error de from str");
            test::black_box(ap);
        });
    }
}
