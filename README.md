# planchet

Interact with the [Numista API](https://numista.com) in Rust.

This repository contains two crates:

* `planchet`: A Rust wrapper for the API.
* `planchet-cli`: A command-line interface for the `planchet` library.

This was mostly an excuse to try [Google Jules](https://jules.google) and see what it spit out.

**This is an exploratory test and a learning exercise. It won't be stable, likely be error-prone, and shouldn't be used outside of this experiment.**

## `planchet` library

### Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
planchet = { git = "https://github.com/jolros/planchet-rs.git", branch = "main" }
```

### Usage

```rust
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

## `planchet-cli`

### Installation

```bash
cargo install --git https://github.com/jolros/planchet-rs.git planchet-cli
```

### Usage

```bash
planchet-cli --api-key <YOUR_API_KEY> --user-id <USER_ID> <COMMAND>
```

### Commands

#### `dump`

Dumps the user's collection to the console, sorted by issuer name, year, and title.

```bash
$ planchet-cli --api-key my-secret-key --user-id 123 dump
Canada - 5 Cents - Victoria (1858)
Canada - 1 Cent - George V (1920)
```

#### `summarize`

Summarizes the user's collection by issuer, showing the total number of items, the oldest item, and the newest item.

```bash
$ planchet-cli --api-key my-secret-key --user-id 123 summarize
+--------+-------------+-------------+-------------+
| Issuer | Total Items | Oldest Item | Newest Item |
+--------+-------------+-------------+-------------+
| Canada | 2           | 1858        | 1920        |
+--------+-------------+-------------+-------------+
```

## License

This project is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
