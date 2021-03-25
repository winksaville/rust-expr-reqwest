# Experiment with crate reqwest

https://lib.rs/crates/reqwest
https://github.com/seanmonstar/reqwest

## Get binance.us avgPrice

Also added code to run tests and benchmarks. I've found
that using serde_json::Value as returned by `.json()` and
instead just using `.text()` is faster.

So I've added code that allows me to select different
variations and variation 1 is "faster" than "0" and "2"
is "1" but broken up so I have greater understanding.

```
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
```
So now you can `run`, `test` and `bench`:
```
wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/expr-reqwest`
resp_json="{\"mins\":5,\"price\":\"52029.52608640\"}"
de_string_or_number_to_u64: num=5 v=5
de_string_or_number_to_f64: s=52029.52608640 v=52029.5260864
avg_price=AvgPrice {
    mins: 5,
    price: 52029.5260864,
}

wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cargo test
   Compiling expr-reqwest v0.1.0 (/home/wink/prgs/rust/projects/expr-reqwest)
    Finished test [unoptimized + debuginfo] target(s) in 1.15s
     Running unittests (target/debug/deps/expr_reqwest-58c00914332c417f)

running 6 tests
test tests::bench_de_string_or_number_to_f64_numbers_to_struct ... ok
test tests::bench_de_string_or_number_to_f64_numbers_to_value_to_struct ... ok
test tests::bench_de_string_or_number_to_f64_string_to_struct ... ok
test tests::bench_de_string_or_number_to_f64_string_to_value_to_struct ... ok
test tests::test_de_string_or_number_to_f64_numbers ... ok
test tests::test_de_string_or_number_to_f64_strings ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cargo bench
    Finished bench [optimized] target(s) in 0.03s
     Running unittests (target/release/deps/expr_reqwest-4e75f85d90ef1199)

running 6 tests
test tests::test_de_string_or_number_to_f64_numbers ... ignored
test tests::test_de_string_or_number_to_f64_strings ... ignored
test tests::bench_de_string_or_number_to_f64_numbers_to_struct          ... bench:         534 ns/iter (+/- 11)
test tests::bench_de_string_or_number_to_f64_numbers_to_value_to_struct ... bench:         733 ns/iter (+/- 26)
test tests::bench_de_string_or_number_to_f64_string_to_struct           ... bench:         315 ns/iter (+/- 12)
test tests::bench_de_string_or_number_to_f64_string_to_value_to_struct  ... bench:         538 ns/iter (+/- 17)

test result: ok. 0 passed; 0 failed; 2 ignored; 4 measured; 0 filtered out; finished in 2.04s
```

## Initial commits

A "regular" dynamic lib build works with `-gnu` target:
```
wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cat .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = [
    #"-C", "target-feature=+crt-static"
]

[build]
target = "x86_64-unknown-linux-gnu"
```
And `cargo clen`, `cargo build --release` `cargo run --release` works:
```
wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cargo clean
wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cargo build --release
   Compiling autocfg v1.0.1
   Compiling libc v0.2.91
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.24
   Compiling unicode-xid v0.2.1
...
   Compiling tokio-util v0.6.5
   Compiling tokio-native-tls v0.3.0
   Compiling h2 v0.3.1
   Compiling hyper v0.14.4
   Compiling hyper-tls v0.5.0
   Compiling reqwest v0.11.2
   Compiling expr-reqwest v0.1.0 (/home/wink/prgs/rust/projects/expr-reqwest)
    Finished release [optimized] target(s) in 18.56s

wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.03s
     Running `target/x86_64-unknown-linux-gnu/release/expr-reqwest`
{
    "origin": "23.119.164.150",
}
```
And the size is 3M:
```
wink@3900x:~/prgs/rust/projects/expr-reqwest/target
$ cargo size --release
    Finished release [optimized] target(s) in 0.03s
   text    data     bss     dec     hex filename
3124700  287064    1240 3413004  34140c expr-reqwest
```
And are the dynamic libs:
```
wink@3900x:~/prgs/rust/projects/expr-reqwest/target
$ ldd x86_64-unknown-linux-gnu/release/expr-reqwest
        linux-vdso.so.1 (0x00007ffdedffe000)
        libssl.so.1.1 => /usr/lib/libssl.so.1.1 (0x00007faf69e9d000)
        libcrypto.so.1.1 => /usr/lib/libcrypto.so.1.1 (0x00007faf69bbf000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007faf69ba5000)
        libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007faf69b84000)
        libm.so.6 => /usr/lib/libm.so.6 (0x00007faf69a3f000)
        libdl.so.2 => /usr/lib/libdl.so.2 (0x00007faf69a38000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007faf69869000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007faf6a28f000)
```
But, I like to try to use tatic library and have
no dynmic linking and `-musl` target is suggested.
But it didn't work.
```
[target.x86_64-unknown-linux-musl]
rustflags = [
    "-C", "target-feature=+crt-static"
]

[build]
target = "x86_64-unknown-linux-gnu"
```
I get the following error:
```
wink@3900x:~/prgs/rust/projects/expr-reqwest (main)
$ cargo build
   Compiling autocfg v1.0.1
   Compiling libc v0.2.91
   Compiling cfg-if v1.0.0
...
   Compiling socket2 v0.3.19
   Compiling unicode-normalization v0.1.17
   Compiling parking_lot v0.11.1
error: failed to run custom build command for `openssl-sys v0.9.61`

Caused by:
  process didn't exit successfully: `/home/wink/prgs/rust/projects/expr-reqwest/target/debug/build/openssl-sys-983334fca549b9d8/build-script-main` (exit code: 101)
  --- stdout
  cargo:rustc-cfg=const_fn
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_LIB_DIR
  X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
  OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_INCLUDE_DIR
  X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
  OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_DIR
  X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_DIR
  OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_x86_64-unknown-linux-musl
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_x86_64_unknown_linux_musl
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-musl
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_musl
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-musl
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_musl
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  run pkg_config fail: "pkg-config has not been configured to support cross-compilation.\n\n                Install a sysroot for the target platform and configure it via\n                PKG_CONFIG_SYSROOT_DIR and PKG_CONFIG_PATH, or install a\n                cross-compiling wrapper for pkg-config and set it via\n                PKG_CONFIG environment variable."

  --- stderr
  thread 'main' panicked at '

  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.

  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

  If you're in a situation where you think the directory *should* be found
  automatically, please open a bug at https://github.com/sfackler/rust-openssl
  and include information about your system as well as this message.

  $HOST = x86_64-unknown-linux-gnu
  $TARGET = x86_64-unknown-linux-musl
  openssl-sys = 0.9.61

  ', /home/wink/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.61/build/find_normal.rs:174:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
