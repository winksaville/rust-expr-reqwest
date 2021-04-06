# Experiment with crate reqwest

https://lib.rs/crates/reqwest
https://github.com/seanmonstar/reqwest

## Runs an order/test!

You must set an environment variables API_KEY and SECRET_KEY
```
export API_KEY=sx6bFVKosMPoNsX3OHadmYfXGxkU9s7XRKIw8FwvNIGLFBJcciUaOxxrzoBkwPKq
export SECRET_KEY=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Also for debugging, it expects a proxy server at https://localhost:8080.
I use mitmdump from the mitmproxy and set the SSLKEYLOGFILE for debugging:
```
SSLKEYLOGFILE=/home/wink/mitm.keylogfile.txt mitmdump --verbose
```

You can then use `cargo run` to run the code:
```
$ cargo run
   Compiling expr-reqwest v0.1.0 (/home/wink/prgs/rust/projects/expr-reqwest)
    Finished dev [unoptimized + debuginfo] target(s) in 1.69s
     Running `target/debug/expr-reqwest`
signature_string=087367c43216239eef7e411e6ec1a28ab6a81146329a7623a85a55aeb5a2b12f
signature_params=[("signature", "087367c43216239eef7e411e6ec1a28ab6a81146329a7623a85a55aeb5a2b12f")]
query_string=symbol=BTCUSD&side=BUY&type=MARKET&quantity=0.0002&recvWindow=5000&timestamp=1617764940524&signature=087367c43216239eef7e411e6ec1a28ab6a81146329a7623a85a55aeb5a2b12f
req_builder=RequestBuilder {
    method: POST,
    url: Url {
        scheme: "https",
        username: "",
        password: None,
        host: Some(
            Domain(
                "api.binance.us",
            ),
        ),
        port: None,
        path: "/api/v3/order/test",
        query: None,
        fragment: None,
    },
    headers: {
        "x-mbx-apikey": "sx6bFVKosMPoNsX3OHadmYfXGxkU9s7XRKIw8FwvNIGLFBJcciUaOxxrzoBkwPKq",
    },
}
resp=Ok(
    Response {
        url: Url {
            scheme: "https",
            username: "",
            password: None,
            host: Some(
                Domain(
                    "api.binance.us",
                ),
            ),
            port: None,
            path: "/api/v3/order/test",
            query: None,
            fragment: None,
        },
        status: 200,
        headers: {
            "content-type": "application/json;charset=UTF-8",
            "content-length": "2",
            "connection": "keep-alive",
            "date": "Wed, 07 Apr 2021 03:09:00 GMT",
            "server": "nginx",
            "x-mbx-uuid": "d0cb0b3c-a89e-4d85-9e31-dc643d933520",
            "x-mbx-used-weight": "1",
            "x-mbx-used-weight-1m": "1",
            "strict-transport-security": "max-age=31536000; includeSubdomains",
            "x-frame-options": "SAMEORIGIN",
            "x-xss-protection": "1; mode=block",
            "x-content-type-options": "nosniff",
            "content-security-policy": "default-src 'self'",
            "x-content-security-policy": "default-src 'self'",
            "x-webkit-csp": "default-src 'self'",
            "cache-control": "no-cache, no-store, must-revalidate",
            "pragma": "no-cache",
            "expires": "0",
            "access-control-allow-origin": "*",
            "access-control-allow-methods": "GET, HEAD, OPTIONS",
            "x-cache": "Miss from cloudfront",
            "via": "1.1 2de9b6504a97ad8423645370927ef0cf.cloudfront.net (CloudFront)",
            "x-amz-cf-pop": "SFO20-C1",
            "x-amz-cf-id": "jsSFivR8f6YjBtohkYZ9H6nvtyviENrBF9qN3upBs2kPMh_S37KCtg==",
        },
    },
)
resp_json="{}"
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
