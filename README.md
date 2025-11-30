![planchet-rs header](./.github/assets/planchet_rs_header.png)

# planchet

[![Documentation](https://img.shields.io/badge/docs-main-blue)](https://jolros.github.io/planchet-rs/)

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
use planchet::models::SearchTypesParams;
use planchet::ClientBuilder;

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

The `--api-key` argument can be omitted if the `NUMISTA_API_KEY` environment variable is set.

```bash
planchet-cli --api-key <YOUR_API_KEY> <COMMAND> <COMMAND_ARGS>
```

### Commands

```
  dump       Dump the user's collection to the console
  summarize  Summarize the user's collection by issuer
  types      Search the catalogue by types
  type       Get a single type by ID
  help       Invocation instructions
```

## License

This project is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
