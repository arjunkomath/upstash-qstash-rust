# [Upstash QStash](https://docs.upstash.com/qstash) for Rust

[![crates.io](https://img.shields.io/crates/v/upstash-qstash)](https://crates.io/crates/upstash-qstash)
[![docs.rs](https://docs.rs/upstash-qstash/badge.svg)](https://docs.rs/upstash-qstash/)
![MIT OR Apache-2.0](https://img.shields.io/crates/l/upstash-qstash)

At the moment this library supports a subset of features, I'm hoping to add more in the future.

You can find the docs [here](https://docs.rs/upstash-qstash).

## Installation
```
[dependencies]
upstash-qstash = "0.1.2"
```

## Usage
```rust
use qstash::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let qstash_client = upstash_qstash::Client::new("your-token".to_owned()).expect("Init failed");
    let body = serde_json::json!({
        "key1": "value1",
        "key2": "value2"
    });
    match qstash_client
        .publish_json(
            "url-or-token".to_owned(),
            &body,
        )
        .await
    {
        Ok(result) => println!("Published {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
```