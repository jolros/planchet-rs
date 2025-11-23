//! A Rust wrapper for the Numista API.
//!
//! This crate provides a simple and ergonomic way to interact with the Numista API.
//!
//! # Examples
//!
//! ```no_run
//! use planchet::{ClientBuilder, SearchTypesParams};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ClientBuilder::new()
//!         .api_key("YOUR_API_KEY".to_string())
//!         .build()
//!         .unwrap();
//!
//!     let params = SearchTypesParams::new().q("victoria");
//!     let response = client.search_types(&params).await.unwrap();
//!
//!     println!("Found {} types", response.count);
//! }
//! ```
pub mod models;

use models::{NumistaType, SearchTypesResponse};
use reqwest::header::{HeaderMap, HeaderValue};
use std::fmt;

/// The error type for this crate.
#[derive(Debug)]
pub enum Error {
    /// An error from the underlying HTTP client.
    Http(reqwest::Error),
    /// The API key was not provided.
    ApiKeyMissing,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::ApiKeyMissing => write!(f, "Numista API key is required"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

/// A `Result` type alias for this crate's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// The main client for interacting with the Numista API.
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

impl Client {
    /// Gets a single type from the Numista catalogue.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The ID of the type to get.
    /// * `lang` - The language to use for the response.
    pub async fn get_type(
        &self,
        type_id: i64,
        lang: Option<&str>,
    ) -> Result<NumistaType> {
        let mut url = format!("{}/types/{}", self.base_url, type_id);
        if let Some(lang) = lang {
            url.push_str(&format!("?lang={}", lang));
        }

        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<NumistaType>()
            .await?)
    }

    /// Searches for types in the Numista catalogue.
    ///
    /// # Arguments
    ///
    /// * `params` - The search parameters.
    pub async fn search_types(
        &self,
        params: &SearchTypesParams,
    ) -> Result<SearchTypesResponse> {
        Ok(self
            .client
            .get(&format!("{}/types", self.base_url))
            .query(&params.build())
            .send()
            .await?
            .json::<SearchTypesResponse>()
            .await?)
    }
}

/// A builder for creating a `Client`.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
}

impl ClientBuilder {
    /// Creates a new `ClientBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key to use for requests.
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the base URL to use for requests.
    ///
    /// This is useful for testing.
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Builds the `Client`.
    pub fn build(self) -> Result<Client> {
        let mut headers = HeaderMap::new();
        if let Some(api_key) = self.api_key {
            let mut auth_value = HeaderValue::from_str(&api_key).unwrap();
            auth_value.set_sensitive(true);
            headers.insert("Numista-API-Key", auth_value);
        } else {
            return Err(Error::ApiKeyMissing);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let base_url = self.base_url.unwrap_or_else(|| "https://api.numista.com/v3".to_string());

        Ok(Client { client, base_url })
    }
}

/// Parameters for searching for types.
#[derive(Debug, Default)]
pub struct SearchTypesParams {
    lang: Option<String>,
    category: Option<String>,
    q: Option<String>,
    issuer: Option<String>,
    catalogue: Option<i64>,
    number: Option<String>,
    ruler: Option<i64>,
    material: Option<i64>,
    year: Option<String>,
    date: Option<String>,
    size: Option<String>,
    weight: Option<String>,
    page: Option<i64>,
    count: Option<i64>,
}

impl SearchTypesParams {
    /// Creates a new `SearchTypesParams`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the language to use for the search.
    pub fn lang(mut self, lang: &str) -> Self {
        self.lang = Some(lang.to_string());
        self
    }

    /// Sets the category to search in.
    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    /// Sets the search query.
    pub fn q(mut self, q: &str) -> Self {
        self.q = Some(q.to_string());
        self
    }

    /// Sets the issuer to search for.
    pub fn issuer(mut self, issuer: &str) -> Self {
        self.issuer = Some(issuer.to_string());
        self
    }

    /// Sets the catalogue to search in.
    pub fn catalogue(mut self, catalogue: i64) -> Self {
        self.catalogue = Some(catalogue);
        self
    }

    /// Sets the number to search for in a catalogue.
    pub fn number(mut self, number: &str) -> Self {
        self.number = Some(number.to_string());
        self
    }

    /// Sets the ruler to search for.
    pub fn ruler(mut self, ruler: i64) -> Self {
        self.ruler = Some(ruler);
        self
    }

    /// Sets the material to search for.
    pub fn material(mut self, material: i64) -> Self {
        self.material = Some(material);
        self
    }

    /// Sets the year to search for.
    pub fn year(mut self, year: &str) -> Self {
        self.year = Some(year.to_string());
        self
    }

    /// Sets the date to search for.
    pub fn date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }

    /// Sets the size to search for.
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the weight to search for.
    pub fn weight(mut self, weight: &str) -> Self {
        self.weight = Some(weight.to_string());
        self
    }

    /// Sets the page to return.
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results per page.
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    fn build(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(lang) = &self.lang {
            params.push(("lang", lang.clone()));
        }
        if let Some(category) = &self.category {
            params.push(("category", category.clone()));
        }
        if let Some(q) = &self.q {
            params.push(("q", q.clone()));
        }
        if let Some(issuer) = &self.issuer {
            params.push(("issuer", issuer.clone()));
        }
        if let Some(catalogue) = &self.catalogue {
            params.push(("catalogue", catalogue.to_string()));
        }
        if let Some(number) = &self.number {
            params.push(("number", number.clone()));
        }
        if let Some(ruler) = &self.ruler {
            params.push(("ruler", ruler.to_string()));
        }
        if let Some(material) = &self.material {
            params.push(("material", material.to_string()));
        }
        if let Some(year) = &self.year {
            params.push(("year", year.clone()));
        }
        if let Some(date) = &self.date {
            params.push(("date", date.clone()));
        }
        if let Some(size) = &self.size {
            params.push(("size", size.clone()));
        }
        if let Some(weight) = &self.weight {
            params.push(("weight", weight.clone()));
        }
        if let Some(page) = &self.page {
            params.push(("page", page.to_string()));
        }
        if let Some(count) = &self.count {
            params.push(("count", count.to_string()));
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_client_test() {
        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .build();
        assert!(client.is_ok());
    }

    #[test]
    fn build_client_missing_api_key_test() {
        let client = ClientBuilder::new().build();
        assert!(client.is_err());
        match client.err().unwrap() {
            Error::ApiKeyMissing => (),
            _ => panic!("Expected ApiKeyMissing error"),
        }
    }

    #[tokio::test]
    async fn get_type_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/types/420")
          .with_status(200)
          .with_header("content-type", "application/json")
          .with_body(r#"{
              "id": 420,
              "url": "https://en.numista.com/catalogue/pieces420.html",
              "title": "5 Cents - Victoria",
              "category": "coin",
              "issuer": {
                "code": "canada",
                "name": "Canada"
              },
              "min_year": 1858,
              "max_year": 1901,
              "type": "Standard circulation coin",
              "demonetization": {
                  "is_demonetized": false
              },
              "tags": []
            }"#)
          .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_type(420, None).await;

        mock.assert();
        assert!(response.is_ok());
        let numista_type = response.unwrap();
        assert_eq!(numista_type.id, 420);
        assert_eq!(numista_type.title, "5 Cents - Victoria");
        assert_eq!(numista_type.type_name.unwrap(), "Standard circulation coin");
    }

    #[tokio::test]
    async fn search_types_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/types?q=victoria")
          .with_status(200)
          .with_header("content-type", "application/json")
          .with_body(r#"{
              "count": 1,
              "types": [
                {
                  "id": 420,
                  "title": "5 Cents - Victoria",
                  "category": "coin",
                  "issuer": {
                    "code": "canada",
                    "name": "Canada"
                  },
                  "min_year": 1858,
                  "max_year": 1901
                }
              ]
            }"#)
          .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let params = SearchTypesParams::new().q("victoria");
        let response = client.search_types(&params).await;

        mock.assert();
        assert!(response.is_ok());
        let search_response = response.unwrap();
        assert_eq!(search_response.count, 1);
        assert_eq!(search_response.types.len(), 1);
        assert_eq!(search_response.types[0].id, 420);
    }
}