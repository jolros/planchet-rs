# planchet

A Rust wrapper for the [Numista API](https://numista.com/api).

This was mostly an excuse to try [Google Jules](https://jules.google) and see what it spit out.

**This is an exploratory test and a learning exercise. It won't be stable, likely be error-prone, and shouldn't be used outside of this experiment.**

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

### Get issues for a type

```no_run
use planchet::ClientBuilder;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .api_key("YOUR_API_KEY".to_string())
        .build()
        .unwrap();

    let response = client.get_issues(420, None).await.unwrap();

    println!("Found {} issues", response.len());
}
```

### Get prices for an issue

```no_run
use planchet::ClientBuilder;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .api_key("YOUR_API_KEY".to_string())
        .build()
        .unwrap();

    let response = client.get_prices(420, 123, None).await.unwrap();

    println!("Found {} prices", response.prices.len());
}
```

### Get a user

```no_run
use planchet::ClientBuilder;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .api_key("YOUR_API_KEY".to_string())
        .build()
        .unwrap();

    let response = client.get_user(1, None).await.unwrap();

    println!("Found user {}", response.username);
}
```

## License

This crate is licensed under either of the following, at your option:

* Apache License, Version 2.0
* MIT license
