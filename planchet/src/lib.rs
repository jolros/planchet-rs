//! A Rust wrapper for the Numista API.
//!
//! # Examples
//!
//! ## Basic Search
//!
//! ```no_run
//! use planchet::model::SearchTypesParams;
//! use planchet::ClientBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ClientBuilder::new()
//!         .api_key("YOUR_API_KEY")
//!         .build()
//!         .unwrap();
//!
//!     let params = SearchTypesParams::new().q("victoria").year_range(1850, 1900);
//!     let response = client.search_types(&params).await.unwrap();
//!
//!     println!("Found {} types", response.count);
//! }
//! ```
//!
//! ## Adding a Collected Item
//!
//! ```no_run
//! use planchet::model::{AddCollectedItemParams, Grade};
//! use planchet::ClientBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ClientBuilder::new()
//!         .api_key("YOUR_API_KEY")
//!         .build()
//!         .unwrap();
//!
//!     let item = AddCollectedItemParams::new(12345) // Type ID
//!         .quantity(1)
//!         .grade(Grade::Xf)
//!         .private_comment("Bought at local coin show");
//!
//!     let result = client.add_collected_item(123, &item).await; // User ID
//!
//!     match result {
//!         Ok(item) => println!("Added item with ID: {}", item.id),
//!         Err(e) => eprintln!("Error adding item: {}", e),
//!     }
//! }
//! ```
//!
//! ## Streaming and Error Handling
//!
//! This example demonstrates how to use the streaming API to fetch all results
//! for a search and how to handle specific API errors.
//!
//! ```no_run
//! use planchet::model::SearchTypesParams;
//! use planchet::{ClientBuilder, Error};
//! use futures::stream::TryStreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ClientBuilder::new()
//!         .api_key("YOUR_API_KEY")
//!         .build()
//!         .unwrap();
//!
//!     let params = SearchTypesParams::new().q("galleon");
//!
//!     let results = client.stream_all_types(params)
//!         .try_collect::<Vec<_>>()
//!         .await;
//!
//!     match results {
//!         Ok(types) => {
//!             println!("Successfully fetched {} types.", types.len());
//!         }
//!         Err(Error::ApiError(e)) if e.kind == Some(planchet::KnownApiError::RateLimitExceeded) => {
//!             eprintln!("Rate limit exceeded. Please try again later.");
//!         }
//!         Err(e) => {
//!             eprintln!("An unexpected error occurred: {}", e);
//!         }
//!     }
//! }
//! ```
pub mod client;
pub mod de;
pub mod error;
pub mod model;

// Re-export public API
pub use client::{Client, ClientBuilder};
pub use error::{ApiError, Error, KnownApiError, Result};
