# planchet

A Rust wrapper for the [Numista API](https://numista.com/api).

This was mostly an excuse to try [Google Jules](https://jules.google) and see what it spit out.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
planchet = "0.1.0"
```

## Usage

```no_run
use planchet::{ClientBuilder, SearchTypesParams};

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .api_key("YOUR_API_KEY".to_string())
        .build()
        .unwrap();

    let params = SearchTypesParams::new().q("victoria");
    let response = client.search_types(&params).await.unwrap();

    println!("Found {} types", response.count);
}
```

## License

This crate is licensed under either of the following, at your option:

* Apache License, Version 2.0
* MIT license
