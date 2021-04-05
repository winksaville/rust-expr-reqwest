// TODO: In binance_signature see if HMAC::mac can be incremental
// TODO: Add tests that contain multi-byte utf8 characters and make any necessary changes
// TODO: Add url/uri/percent encoding: https://en.wikipedia.org/wiki/Percent-encoding

use hmac_sha256::HMAC;

pub fn query_vec_u8(query_params: &Vec<(&str, &str)>) -> Vec<u8> {
    let mut qs = Vec::<u8>::with_capacity(1024);

    for (i, kv) in query_params.iter().enumerate() {
        let (k, v) = kv;
        // println!("query_str: i={}: k={:?} v={:?}", i, k, v);
        let kv_pair = format!("{}={}", k, v);

        if i > 0 {
            // println!("query_str: append i={} '&'", i);
            qs.append(&mut vec![b'&']);
        }
        qs.append(&mut kv_pair.into_bytes());
    }

    // println!("query_str: qs=\"{}\"", String::from_utf8(qs.clone()).unwrap());
    qs
}

pub fn binance_signature(
    sig_key: &[u8],
    qs: &Vec<u8>,
    body: &Vec<u8>,
) -> [u8; 32] {
    // println!("binance_signature: qs=\"{}\"", String::from_utf8(qs.clone()).unwrap());
    // println!("binance_signature: body=\"{}\"", String::from_utf8(body.clone()).unwrap());
    let mut qs_and_body = qs.clone();
    qs_and_body.append(&mut body.clone());
    // println!("binance_signature: qs_and_body=\"{}\"", String::from_utf8(qs_and_body.clone()).unwrap());

    let signature = HMAC::mac(&qs_and_body, sig_key);
    // println!("binance_signature: {:02x?}", signature);

    signature
}

#[cfg(test)]
mod test {
    // rust-anaylzer incorrectly reports test as unresolved:
    //   https://github.com/rust-analyzer/rust-analyzer/issues/6714
    extern crate test;

    use super::*;
    //use test::Bencher;

    #[test]
    fn test_query_vec_u8_no_data() {
        let query_params = vec![];
        let expected = b"";

        // Create the query Vec<u8> from parameters
        let qs = query_vec_u8(&query_params);
        // println!("test_query_vec_u8_no_data: expected {:02x?}", expected);
        // println!("test_query_vec_u8_no_data: qs {:02x?}", qs);

        // Validate
        assert_eq!(qs.len(), expected.len());
        assert_eq!(qs, expected);
    }

    #[test]
    fn test_query_vec_u8() {
        // query string from:
        //   https://github.com/binance-us/binance-official-api-docs/blob/5a1dd14437bd3be4631778e78d3203014cf30b63/rest-api.md#example-1-as-a-request-body
        let expected = b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";

        let query_params = vec![
            ("symbol", "LTCBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ];

        // Create the query Vec<u8> from parameters
        let qs = query_vec_u8(&query_params);
        // println!("test_query_vec_u8: es {:02x?}", es);
        // println!("test_query_vec_u8: qs {:02x?}", qs);

        // Validate
        assert_eq!(qs.len(), expected.len());
        assert_eq!(qs, expected);
    }

    #[test]
    fn test_binance_example() {
        // Data, sig_key and expected from:
        //   https://github.com/binance-us/binance-official-api-docs/blob/5a1dd14437bd3be4631778e78d3203014cf30b63/rest-api.md#example-1-as-a-request-body
        let data = b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let sig_key = b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let expected = hex!("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71");

        // Calculate the signature from the data and sig_key
        let signature = HMAC::mac(data, sig_key);
        // println!("signature ={:02x?}", signature);

        // Validate
        assert_eq!(signature.len(), 32);
        assert_eq!(signature, expected);
    }

    #[test]
    fn test_binance_signature_no_query_string_no_body() {
        let sig_key = b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";

        // Expected is "self" calculated so NOT indpendently verified
        let expected = hex!("18f82ab1c4ba20d60cb86ebc4cab5b54ddb974cdf7832421345148e7a7f9466e");

        let qs = Vec::<u8>::new();
        let body = Vec::<u8>::new();

        // Calculate the signature from the data and key
        let signature = binance_signature(sig_key, &qs, &body);
        // println!("signature:         {:02x?}", signature);

        // Validate
        assert_eq!(signature.len(), 32);
        assert_eq!(signature, expected);
    }

    #[test]
    fn test_binance_signature_body_only() {
        // query_params, sig_key and expected from:
        //   https://github.com/binance-us/binance-official-api-docs/blob/5a1dd14437bd3be4631778e78d3203014cf30b63/rest-api.md#example-1-as-a-request-body
        let query_params = vec![
            ("symbol", "LTCBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ];
        let sig_key = b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let expected = hex!("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71");

        let qs = Vec::<u8>::new();
        let body = query_vec_u8(&query_params);

        // Calculate the signature from the data and key
        let signature = binance_signature(sig_key, &qs, &body);
        // println!("signature:         {:02x?}", signature);

        // Validate
        assert_eq!(signature.len(), 32);
        assert_eq!(signature, expected);
    }

    #[test]
    fn test_binance_signature_query_string_only() {
        // query_params, sig_key and expected from:
        //   https://github.com/binance-us/binance-official-api-docs/blob/5a1dd14437bd3be4631778e78d3203014cf30b63/rest-api.md#example-2-as-a-query-string
        let query_params = vec![
            ("symbol", "LTCBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ];
        let sig_key = b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let expected = hex!("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71");

        //let query_params = vec![("symbol", "LTCBTC")];

        let qs = query_vec_u8(&query_params);
        let body = Vec::new();

        // Calculate the signature from the data and key
        let signature = binance_signature(sig_key, &qs, &body);
        // println!("signature:         {:02x?}", signature);

        // Validate
        assert_eq!(signature.len(), 32);
        assert_eq!(signature, expected);
    }

    #[test]
    fn test_binance_signature_query_string_and_body() {
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

        //let query_params = vec![("symbol", "LTCBTC")];

        let qs = query_vec_u8(&query_params);
        let body = query_vec_u8(&body_params);

        // Calculate the signature from the data and key
        let signature = binance_signature(sig_key, &qs, &body);
        // println!("signature:         {:02x?}", signature);

        // Validate
        assert_eq!(signature.len(), 32);
        assert_eq!(signature, expected);
    }
}
