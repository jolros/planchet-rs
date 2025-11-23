//! A Rust wrapper for the Numista API.
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

use models::{
    CataloguesResponse, CollectedItem, CollectedItemsResponse, CollectionsResponse,
    IssuersResponse, MintDetail, MintsResponse, NumistaType, OAuthToken, PricesResponse,
    Publication, SearchByImageResponse, SearchTypesResponse, User,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
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

    /// Gets the issues of a type.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The ID of the type to get the issues for.
    /// * `lang` - The language to use for the response.
    pub async fn get_issues(
        &self,
        type_id: i64,
        lang: Option<&str>,
    ) -> Result<Vec<models::Issue>> {
        let mut url = format!("{}/types/{}/issues", self.base_url, type_id);
        if let Some(lang) = lang {
            url.push_str(&format!("?lang={}", lang));
        }

        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Vec<models::Issue>>()
            .await?)
    }

    /// Gets the prices for an issue.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The ID of the type.
    /// * `issue_id` - The ID of the issue.
    /// * `currency` - The currency to get the prices in.
    pub async fn get_prices(
        &self,
        type_id: i64,
        issue_id: i64,
        currency: Option<&str>,
    ) -> Result<PricesResponse> {
        let mut url = format!(
            "{}/types/{}/issues/{}/prices",
            self.base_url, type_id, issue_id
        );
        if let Some(currency) = currency {
            url.push_str(&format!("?currency={}", currency));
        }

        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<PricesResponse>()
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

    /// Gets the list of issuers.
    ///
    /// # Arguments
    ///
    /// * `lang` - The language to use for the response.
    pub async fn get_issuers(&self, lang: Option<&str>) -> Result<IssuersResponse> {
        let mut url = format!("{}/issuers", self.base_url);
        if let Some(lang) = lang {
            url.push_str(&format!("?lang={}", lang));
        }

        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<IssuersResponse>()
            .await?)
    }

    /// Gets the list of mints.
    ///
    /// # Arguments
    ///
    /// * `lang` - The language to use for the response.
    pub async fn get_mints(&self, lang: Option<&str>) -> Result<MintsResponse> {
        let mut url = format!("{}/mints", self.base_url);
        if let Some(lang) = lang {
            url.push_str(&format!("?lang={}", lang));
        }

        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<MintsResponse>()
            .await?)
    }

    /// Gets a single mint.
    ///
    /// # Arguments
    ///
    /// * `mint_id` - The ID of the mint to get.
    /// * `lang` - The language to use for the response.
    pub async fn get_mint(&self, mint_id: i64, lang: Option<&str>) -> Result<MintDetail> {
        let mut url = format!("{}/mints/{}", self.base_url, mint_id);
        if let Some(lang) = lang {
            url.push_str(&format!("?lang={}", lang));
        }

        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<MintDetail>()
            .await?)
    }

    /// Gets the list of catalogues.
    pub async fn get_catalogues(&self) -> Result<CataloguesResponse> {
        Ok(self
            .client
            .get(&format!("{}/catalogues", self.base_url))
            .send()
            .await?
            .json::<CataloguesResponse>()
            .await?)
    }

    /// Gets a single publication.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the publication to get.
    pub async fn get_publication(&self, id: &str) -> Result<Publication> {
        Ok(self
            .client
            .get(&format!("{}/publications/{}", self.base_url, id))
            .send()
            .await?
            .json::<Publication>()
            .await?)
    }

    /// Gets a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to get.
    /// * `lang` - The language to use for the response.
    pub async fn get_user(&self, user_id: i64, lang: Option<&str>) -> Result<User> {
        let mut url = format!("{}/users/{}", self.base_url, user_id);
        if let Some(lang) = lang {
            url.push_str(&format!("?lang={}", lang));
        }

        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<User>()
            .await?)
    }

    /// Gets the collections of a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to get the collections for.
    pub async fn get_user_collections(&self, user_id: i64) -> Result<CollectionsResponse> {
        Ok(self
            .client
            .get(&format!("{}/users/{}/collections", self.base_url, user_id))
            .send()
            .await?
            .json::<CollectionsResponse>()
            .await?)
    }

    /// Gets the collected items of a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to get the collected items for.
    /// * `params` - The search parameters.
    pub async fn get_collected_items(
        &self,
        user_id: i64,
        params: &GetCollectedItemsParams,
    ) -> Result<CollectedItemsResponse> {
        Ok(self
            .client
            .get(&format!(
                "{}/users/{}/collected_items",
                self.base_url, user_id
            ))
            .query(&params.build())
            .send()
            .await?
            .json::<CollectedItemsResponse>()
            .await?)
    }

    /// Adds a collected item to a user's collection.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to add the collected item to.
    /// * `item` - The item to add.
    pub async fn add_collected_item(
        &self,
        user_id: i64,
        item: &AddCollectedItem,
    ) -> Result<CollectedItem> {
        Ok(self
            .client
            .post(&format!(
                "{}/users/{}/collected_items",
                self.base_url, user_id
            ))
            .json(item)
            .send()
            .await?
            .json::<CollectedItem>()
            .await?)
    }

    /// Gets a single collected item from a user's collection.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    /// * `item_id` - The ID of the item to get.
    pub async fn get_collected_item(&self, user_id: i64, item_id: i64) -> Result<CollectedItem> {
        Ok(self
            .client
            .get(&format!(
                "{}/users/{}/collected_items/{}",
                self.base_url, user_id, item_id
            ))
            .send()
            .await?
            .json::<CollectedItem>()
            .await?)
    }

    /// Edits a collected item in a user's collection.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    /// * `item_id` - The ID of the item to edit.
    /// * `item` - The fields to edit.
    pub async fn edit_collected_item(
        &self,
        user_id: i64,
        item_id: i64,
        item: &EditCollectedItem,
    ) -> Result<CollectedItem> {
        Ok(self
            .client
            .patch(&format!(
                "{}/users/{}/collected_items/{}",
                self.base_url, user_id, item_id
            ))
            .json(item)
            .send()
            .await?
            .json::<CollectedItem>()
            .await?)
    }

    /// Deletes a collected item from a user's collection.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    /// * `item_id` - The ID of the item to delete.
    pub async fn delete_collected_item(&self, user_id: i64, item_id: i64) -> Result<()> {
        self.client
            .delete(&format!(
                "{}/users/{}/collected_items/{}",
                self.base_url, user_id, item_id
            ))
            .send()
            .await?;
        Ok(())
    }

    /// Gets an OAuth token.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for getting the token.
    pub async fn get_oauth_token(&self, params: &OAuthTokenParams) -> Result<OAuthToken> {
        Ok(self
            .client
            .get(&format!("{}/oauth_token", self.base_url))
            .query(params)
            .send()
            .await?
            .json::<OAuthToken>()
            .await?)
    }

    /// Searches for types by image.
    ///
    /// # Arguments
    ///
    /// * `request` - The request body.
    pub async fn search_by_image(&self, request: &SearchByImageRequest) -> Result<SearchByImageResponse> {
        Ok(self
            .client
            .post(&format!("{}/search_by_image", self.base_url))
            .json(request)
            .send()
            .await?
            .json::<SearchByImageResponse>()
            .await?)
    }
}

#[derive(Debug, Serialize)]
pub struct SearchByImageRequest {
    pub category: Option<String>,
    pub images: Vec<Image>,
    pub max_results: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Image {
    pub mime_type: String,
    pub image_data: String,
}

#[derive(Debug, Serialize)]
pub struct OAuthTokenParams {
    pub grant_type: String,
    pub code: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct GetCollectedItemsParams {
    category: Option<String>,
    #[serde(rename = "type")]
    type_id: Option<i64>,
    collection: Option<i64>,
}

impl GetCollectedItemsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    pub fn type_id(mut self, type_id: i64) -> Self {
        self.type_id = Some(type_id);
        self
    }

    pub fn collection(mut self, collection: i64) -> Self {
        self.collection = Some(collection);
        self
    }

    fn build(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(category) = &self.category {
            params.push(("category", category.clone()));
        }
        if let Some(type_id) = &self.type_id {
            params.push(("type", type_id.to_string()));
        }
        if let Some(collection) = &self.collection {
            params.push(("collection", collection.to_string()));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct AddCollectedItem {
    #[serde(rename = "type")]
    pub type_id: i64,
    pub issue: Option<i64>,
    pub quantity: Option<i64>,
    pub grade: Option<String>,
    pub for_swap: Option<bool>,
    pub private_comment: Option<String>,
    pub public_comment: Option<String>,
    pub price: Option<ItemPrice>,
    pub collection: Option<i64>,
    pub storage_location: Option<String>,
    pub acquisition_place: Option<String>,
    pub acquisition_date: Option<String>,
    pub serial_number: Option<String>,
    pub internal_id: Option<String>,
    pub weight: Option<f64>,
    pub size: Option<f64>,
    pub axis: Option<i64>,
    pub grading_details: Option<GradingDetails>,
}

#[derive(Debug, Serialize)]
pub struct EditCollectedItem {
    #[serde(rename = "type")]
    pub type_id: Option<i64>,
    pub issue: Option<i64>,
    pub quantity: Option<i64>,
    pub grade: Option<String>,
    pub for_swap: Option<bool>,
    pub private_comment: Option<String>,
    pub public_comment: Option<String>,
    pub price: Option<ItemPrice>,
    pub collection: Option<i64>,
    pub storage_location: Option<String>,
    pub acquisition_place: Option<String>,
    pub acquisition_date: Option<String>,
    pub serial_number: Option<String>,
    pub internal_id: Option<String>,
    pub weight: Option<f64>,
    pub size: Option<f64>,
    pub axis: Option<i64>,
    pub grading_details: Option<GradingDetails>,
}

#[derive(Debug, Serialize)]
pub struct ItemPrice {
    pub value: f64,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct GradingDetails {
    pub grading_company: Option<i64>,
    pub slab_grade: Option<i64>,
    pub slab_number: Option<String>,
    pub cac_sticker: Option<String>,
    pub grading_designations: Option<Vec<i64>>,
    pub grading_strike: Option<i64>,
    pub grading_surface: Option<i64>,
}

/// A builder for creating a `Client`.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    bearer_token: Option<String>,
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

    /// Sets the bearer token to use for requests.
    pub fn bearer_token(mut self, bearer_token: String) -> Self {
        self.bearer_token = Some(bearer_token);
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

        if let Some(bearer_token) = self.bearer_token {
            let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap();
            auth_value.set_sensitive(true);
            headers.insert("Authorization", auth_value);
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

        let response = client.get_type(420, None).await.unwrap();

        mock.assert();
        assert_eq!(response.id, 420);
        assert_eq!(response.title, "5 Cents - Victoria");
        assert_eq!(
            response.type_name.unwrap(),
            "Standard circulation coin"
        );
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
        let response = client.search_types(&params).await.unwrap();

        mock.assert();
        assert_eq!(response.count, 1);
        assert_eq!(response.types.len(), 1);
        assert_eq!(response.types[0].id, 420);
    }

    #[tokio::test]
    async fn get_issues_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/types/420/issues")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id": 1, "is_dated": true}]"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_issues(420, None).await.unwrap();

        mock.assert();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].id, 1);
    }

    #[tokio::test]
    async fn get_prices_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/types/420/issues/123/prices")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"currency": "USD", "prices": []}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_prices(420, 123, None).await.unwrap();

        mock.assert();
        assert_eq!(response.currency, "USD");
    }

    #[tokio::test]
    async fn get_issuers_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/issuers")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count": 1, "issuers": [{"code": "canada", "name": "Canada"}]}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_issuers(None).await.unwrap();

        mock.assert();
        assert_eq!(response.count, 1);
        assert_eq!(response.issuers.len(), 1);
        assert_eq!(response.issuers[0].code, "canada");
    }

    #[tokio::test]
    async fn get_mints_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/mints")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count": 1, "mints": [{"id": 1}]}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_mints(None).await.unwrap();

        mock.assert();
        assert_eq!(response.count, 1);
        assert_eq!(response.mints.len(), 1);
        assert_eq!(response.mints[0].id, 1);
    }

    #[tokio::test]
    async fn get_mint_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/mints/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_mint(1, None).await.unwrap();

        mock.assert();
        assert_eq!(response.id, 1);
    }

    #[tokio::test]
    async fn get_catalogues_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/catalogues")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count": 1, "catalogues": [{"id": 1, "code": "KM", "title": "Test", "author": "Test", "publisher": "Test"}]}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_catalogues().await.unwrap();

        mock.assert();
        assert_eq!(response.count, 1);
        assert_eq!(response.catalogues.len(), 1);
        assert_eq!(response.catalogues[0].id, 1);
    }

    #[tokio::test]
    async fn get_publication_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/publications/L106610")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "L106610", "url": "", "type": "volume", "title": "Test", "languages": []}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_publication("L106610").await.unwrap();

        mock.assert();
        assert_eq!(response.id, "L106610");
    }

    #[tokio::test]
    async fn get_user_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/users/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"username": "test"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_user(1, None).await.unwrap();

        mock.assert();
        assert_eq!(response.username, "test");
    }

    #[tokio::test]
    async fn get_user_collections_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/users/1/collections")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count": 1, "collections": [{"id": 1, "name": "Test"}]}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_user_collections(1).await.unwrap();

        mock.assert();
        assert_eq!(response.count, 1);
        assert_eq!(response.collections.len(), 1);
        assert_eq!(response.collections[0].id, 1);
    }

    #[tokio::test]
    async fn get_collected_items_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/users/1/collected_items")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"item_count": 1, "item_for_swap_count": 0, "item_type_count": 1, "item_type_for_swap_count": 0, "items": [{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}]}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let params = GetCollectedItemsParams::new();
        let response = client.get_collected_items(1, &params).await.unwrap();

        mock.assert();
        assert_eq!(response.item_count, 1);
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].id, 1);
    }

    #[tokio::test]
    async fn add_collected_item_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("POST", "/users/1/collected_items")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let item = AddCollectedItem {
            type_id: 1,
            issue: None,
            quantity: None,
            grade: None,
            for_swap: None,
            private_comment: None,
            public_comment: None,
            price: None,
            collection: None,
            storage_location: None,
            acquisition_place: None,
            acquisition_date: None,
            serial_number: None,
            internal_id: None,
            weight: None,
            size: None,
            axis: None,
            grading_details: None,
        };
        let response = client.add_collected_item(1, &item).await.unwrap();

        mock.assert();
        assert_eq!(response.id, 1);
    }

    #[tokio::test]
    async fn get_collected_item_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/users/1/collected_items/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_collected_item(1, 1).await.unwrap();

        mock.assert();
        assert_eq!(response.id, 1);
    }

    #[tokio::test]
    async fn edit_collected_item_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("PATCH", "/users/1/collected_items/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1, "quantity": 1, "type": {"id": 1, "title": "Test", "category": "coin"}, "for_swap": false}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let item = EditCollectedItem {
            type_id: None,
            issue: None,
            quantity: None,
            grade: None,
            for_swap: None,
            private_comment: None,
            public_comment: None,
            price: None,
            collection: None,
            storage_location: None,
            acquisition_place: None,
            acquisition_date: None,
            serial_number: None,
            internal_id: None,
            weight: None,
            size: None,
            axis: None,
            grading_details: None,
        };
        let response = client.edit_collected_item(1, 1, &item).await.unwrap();

        mock.assert();
        assert_eq!(response.id, 1);
    }

    #[tokio::test]
    async fn delete_collected_item_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("DELETE", "/users/1/collected_items/1")
            .with_status(204)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.delete_collected_item(1, 1).await;

        mock.assert();
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_oauth_token_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/oauth_token?grant_type=client_credentials")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"access_token": "test", "token_type": "bearer", "expires_in": 3600, "user_id": 1}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let params = OAuthTokenParams {
            grant_type: "client_credentials".to_string(),
            code: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
        };
        let response = client.get_oauth_token(&params).await.unwrap();

        mock.assert();
        assert_eq!(response.access_token, "test");
    }
}
