//! A Rust wrapper for the Numista API.
//!
//! # Examples
//!
//! ## Basic Search
//!
//! ```no_run
//! use planchet::{ClientBuilder, SearchTypesParams};
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
//! ## Streaming and Error Handling
//!
//! This example demonstrates how to use the streaming API to fetch all results
//! for a search and how to handle specific API errors.
//!
//! ```no_run
//! use planchet::{ClientBuilder, Error, SearchTypesParams};
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
pub mod models;

// Re-export public API
pub use client::{Client, ClientBuilder};
pub use error::{ApiError, Error, KnownApiError, Result};
pub use models::request::{
    AddCollectedItem, EditCollectedItem, GetCollectedItemsParams, GradingDetails, ItemPrice,
    OAuthTokenParams, SearchTypesParams,
};
pub use models::{
    CataloguesResponse, Category, CollectedItem, CollectedItemsResponse, CollectionsResponse,
    Grade, IssuersResponse, MintDetail, MintsResponse, NumistaType, OAuthToken, PricesResponse,
    Publication, SearchByImageResponse, SearchTypesResponse, User,
};
