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
//!         Err(Error::RateLimitExceeded) => {
//!             eprintln!("Rate limit exceeded. Please try again later.");
//!         }
//!         Err(e) => {
//!             eprintln!("An unexpected error occurred: {}", e);
//!         }
//!     }
//! }
//! ```
pub mod models;

use futures::stream::{self, Stream};
use isolang::Language;
use models::{
    CataloguesResponse, Category, CollectedItem, CollectedItemsResponse, CollectionsResponse,
    Grade, IssuersResponse, MintDetail, MintsResponse, NumistaType, OAuthToken, PricesResponse,
    Publication, SearchByImageResponse, SearchTypesResponse, User,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{de::DeserializeOwned, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;

/// The error type for this crate.
#[derive(Debug)]
pub enum Error {
    /// An error from the underlying HTTP client (`reqwest`).
    Http(reqwest::Error),
    /// The API key was not provided in the `ClientBuilder`.
    ApiKeyMissing,
    /// The provided API key is invalid or has expired (HTTP 401).
    Unauthorized,
    /// The requested resource could not be found (HTTP 404).
    NotFound,
    /// A parameter in the request was invalid or missing (HTTP 400).
    InvalidParameter(String),
    /// The API rate limit has been exceeded (HTTP 429).
    RateLimitExceeded,
    /// No user is associated with the provided API key (HTTP 501).
    /// This is specific to the `client_credentials` grant type.
    NoUserAssociatedWithApiKey,
    /// An error returned by the Numista API that does not map to a specific error variant.
    ApiError { message: String, status: u16 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::ApiKeyMissing => write!(f, "Numista API key is required"),
            Error::Unauthorized => write!(f, "Unauthorized: Invalid or expired API key"),
            Error::NotFound => write!(f, "NotFound: The requested resource was not found"),
            Error::InvalidParameter(msg) => write!(f, "InvalidParameter: {}", msg),
            Error::RateLimitExceeded => write!(f, "RateLimitExceeded: You have exceeded the API rate limit"),
            Error::NoUserAssociatedWithApiKey => write!(f, "NoUserAssociatedWithApiKey: No user is associated with this API key"),
            Error::ApiError { message, status } => {
                write!(f, "API error (status {}): {}", status, message)
            }
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

async fn process_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    if status.is_success() {
        return Ok(response.json::<T>().await?);
    }

    let status_code = status.as_u16();
    let api_error = match response.json::<models::ApiError>().await {
        Ok(api_error) => api_error,
        Err(e) => return Err(e.into()),
    };

    match status_code {
        400 => Err(Error::InvalidParameter(api_error.error_message)),
        401 => Err(Error::Unauthorized),
        404 => Err(Error::NotFound),
        429 => Err(Error::RateLimitExceeded),
        501 => Err(Error::NoUserAssociatedWithApiKey),
        _ => Err(Error::ApiError {
            message: api_error.error_message,
            status: status_code,
        }),
    }
}

macro_rules! add_lang_param {
    ($req:expr, $lang:expr) => {
        if let Some(l) = $lang {
            $req.query(&[("lang", l.to_639_1())])
        } else {
            $req
        }
    };
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
        lang: Option<Language>,
    ) -> Result<NumistaType> {
        let url = format!("{}/types/{}", self.base_url, type_id);
        let req = self.client.get(&url);
        let req = add_lang_param!(req, lang);
        let response = req.send().await?;
        process_response(response).await
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
        lang: Option<Language>,
    ) -> Result<Vec<models::Issue>> {
        let url = format!("{}/types/{}/issues", self.base_url, type_id);
        let req = self.client.get(&url);
        let req = add_lang_param!(req, lang);
        let response = req.send().await?;
        process_response(response).await
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
        lang: Option<Language>,
    ) -> Result<PricesResponse> {
        #[derive(Serialize)]
        struct GetPricesParams<'a> {
            currency: Option<&'a str>,
            lang: Option<&'a str>,
        }

        let url = format!(
            "{}/types/{}/issues/{}/prices",
            self.base_url, type_id, issue_id
        );

        let lang_str = lang.and_then(|l| l.to_639_1());
        let params = GetPricesParams {
            currency,
            lang: lang_str,
        };

        let response = self.client.get(&url).query(&params).send().await?;
        process_response(response).await
    }

    /// Searches for types in the Numista catalogue.
    ///
    /// # Arguments
    ///
    /// * `params` - The search parameters.
    pub async fn search_types(
        &self,
        params: &SearchTypesParams<'_>,
    ) -> Result<SearchTypesResponse> {
        let response = self
            .client
            .get(&format!("{}/types", self.base_url))
            .query(params)
            .send()
            .await?;
        process_response(response).await
    }

    /// Returns a stream of all types matching the search parameters.
    ///
    /// This method will make multiple API calls as needed to fetch all pages.
    ///
    /// # Arguments
    ///
    /// * `params` - The search parameters.
    pub fn stream_all_types<'a>(
        &self,
        params: SearchTypesParams<'a>,
    ) -> impl Stream<Item = Result<models::SearchTypeResult>> + 'a {
        struct State<'a> {
            client: Client,
            params: SearchTypesParams<'a>,
            current_page: i64,
            buffer: std::vec::IntoIter<models::SearchTypeResult>,
            items_fetched: i64,
            total_items: Option<i64>,
        }

        let initial_state = State {
            client: self.clone(),
            params,
            current_page: 1,
            buffer: Vec::new().into_iter(),
            items_fetched: 0,
            total_items: None,
        };

        stream::unfold(initial_state, |mut state| async move {
            // Stop if we have fetched all items OR if the last page was empty.
            if let Some(total) = state.total_items {
                if state.items_fetched >= total {
                    return None;
                }
            }

            // If we have items in the buffer, return the next one
            if let Some(item) = state.buffer.next() {
                state.items_fetched += 1;
                return Some((Ok(item), state));
            }

            // Buffer is empty, fetch the next page
            let mut params = state.params.clone();
            params.page = Some(state.current_page);

            match state.client.search_types(&params).await {
                Ok(response) => {
                    if state.total_items.is_none() {
                        state.total_items = Some(response.count);
                    }

                    // If the page is empty, we're done for good.
                    if response.types.is_empty() {
                        state.total_items = Some(state.items_fetched); // Prevent any further calls
                        return None;
                    }

                    // Increment page number and refill buffer
                    state.current_page += 1;
                    state.buffer = response.types.into_iter();

                    // Return the first item from the new buffer
                    if let Some(item) = state.buffer.next() {
                        state.items_fetched += 1;
                        Some((Ok(item), state))
                    } else {
                        None
                    }
                }
                Err(e) => {
                    // On error, stop streaming and return the error
                    state.total_items = Some(state.items_fetched); // Prevent further calls
                    Some((Err(e), state))
                }
            }
        })
    }

    /// Gets the list of issuers.
    ///
    /// # Arguments
    ///
    /// * `lang` - The language to use for the response.
    pub async fn get_issuers(&self, lang: Option<Language>) -> Result<IssuersResponse> {
        let url = format!("{}/issuers", self.base_url);
        let req = self.client.get(&url);
        let req = add_lang_param!(req, lang);
        let response = req.send().await?;
        process_response(response).await
    }

    /// Gets the list of mints.
    ///
    /// # Arguments
    ///
    /// * `lang` - The language to use for the response.
    pub async fn get_mints(&self, lang: Option<Language>) -> Result<MintsResponse> {
        let url = format!("{}/mints", self.base_url);
        let req = self.client.get(&url);
        let req = add_lang_param!(req, lang);
        let response = req.send().await?;
        process_response(response).await
    }

    /// Gets a single mint.
    ///
    /// # Arguments
    ///
    /// * `mint_id` - The ID of the mint to get.
    /// * `lang` - The language to use for the response.
    pub async fn get_mint(&self, mint_id: i64, lang: Option<Language>) -> Result<MintDetail> {
        let url = format!("{}/mints/{}", self.base_url, mint_id);
        let req = self.client.get(&url);
        let req = add_lang_param!(req, lang);
        let response = req.send().await?;
        process_response(response).await
    }

    /// Gets the list of catalogues.
    pub async fn get_catalogues(&self) -> Result<CataloguesResponse> {
        let response = self
            .client
            .get(&format!("{}/catalogues", self.base_url))
            .send()
            .await?;
        process_response(response).await
    }

    /// Gets a single publication.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the publication to get.
    pub async fn get_publication(&self, id: &str) -> Result<Publication> {
        let response = self
            .client
            .get(&format!("{}/publications/{}", self.base_url, id))
            .send()
            .await?;
        process_response(response).await
    }

    /// Gets a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to get.
    /// * `lang` - The language to use for the response.
    pub async fn get_user(&self, user_id: i64, lang: Option<Language>) -> Result<User> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        let req = self.client.get(&url);
        let req = add_lang_param!(req, lang);
        let response = req.send().await?;
        process_response(response).await
    }

    /// Gets the collections of a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to get the collections for.
    pub async fn get_user_collections(&self, user_id: i64) -> Result<CollectionsResponse> {
        let response = self
            .client
            .get(&format!("{}/users/{}/collections", self.base_url, user_id))
            .send()
            .await?;
        process_response(response).await
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
        let response = self
            .client
            .get(&format!(
                "{}/users/{}/collected_items",
                self.base_url, user_id
            ))
            .query(params)
            .send()
            .await?;
        process_response(response).await
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
        let response = self
            .client
            .post(&format!(
                "{}/users/{}/collected_items",
                self.base_url, user_id
            ))
            .json(item)
            .send()
            .await?;
        process_response(response).await
    }

    /// Gets a single collected item from a user's collection.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    /// * `item_id` - The ID of the item to get.
    pub async fn get_collected_item(&self, user_id: i64, item_id: i64) -> Result<CollectedItem> {
        let response = self
            .client
            .get(&format!(
                "{}/users/{}/collected_items/{}",
                self.base_url, user_id, item_id
            ))
            .send()
            .await?;
        process_response(response).await
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
        let response = self
            .client
            .patch(&format!(
                "{}/users/{}/collected_items/{}",
                self.base_url, user_id, item_id
            ))
            .json(item)
            .send()
            .await?;
        process_response(response).await
    }

    /// Deletes a collected item from a user's collection.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    /// * `item_id` - The ID of the item to delete.
    pub async fn delete_collected_item(&self, user_id: i64, item_id: i64) -> Result<()> {
        let response = self
            .client
            .delete(&format!(
                "{}/users/{}/collected_items/{}",
                self.base_url, user_id, item_id
            ))
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(());
        }

        let status_code = response.status().as_u16();
        let api_error = match response.json::<models::ApiError>().await {
            Ok(api_error) => api_error,
            Err(e) => return Err(e.into()),
        };

        match status_code {
            401 => Err(Error::Unauthorized),
            404 => Err(Error::NotFound),
            429 => Err(Error::RateLimitExceeded),
            _ => Err(Error::ApiError {
                message: api_error.error_message,
                status: status_code,
            }),
        }
    }

    /// Gets an OAuth token.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for getting the token.
    pub async fn get_oauth_token(&self, params: &OAuthTokenParams) -> Result<OAuthToken> {
        let response = self
            .client
            .get(&format!("{}/oauth_token", self.base_url))
            .query(params)
            .send()
            .await?;
        process_response(response).await
    }

    /// Searches for types by image.
    ///
    /// # Arguments
    ///
    /// * `request` - The request body.
    pub async fn search_by_image(
        &self,
        request: &models::SearchByImageRequest,
    ) -> Result<SearchByImageResponse> {
        let response = self
            .client
            .post(&format!("{}/search_by_image", self.base_url))
            .json(request)
            .send()
            .await?;
        process_response(response).await
    }
}

use rust_decimal::Decimal;

#[derive(Debug, Serialize)]
pub struct OAuthTokenParams {
    pub grant_type: models::GrantType,
    pub code: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct GetCollectedItemsParams {
    category: Option<models::Category>,
    #[serde(rename = "type")]
    type_id: Option<i64>,
    collection: Option<i64>,
}

impl GetCollectedItemsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: models::Category) -> Self {
        self.category = Some(category);
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

}

#[derive(Debug, Serialize)]
pub struct AddCollectedItem {
    #[serde(rename = "type")]
    pub type_id: i64,
    pub issue: Option<i64>,
    pub quantity: Option<i64>,
    pub grade: Option<Grade>,
    pub for_swap: Option<bool>,
    pub private_comment: Option<String>,
    pub public_comment: Option<String>,
    pub price: Option<ItemPrice>,
    pub collection: Option<i64>,
    pub storage_location: Option<String>,
    pub acquisition_place: Option<String>,
    pub acquisition_date: Option<chrono::NaiveDate>,
    pub serial_number: Option<String>,
    pub internal_id: Option<String>,
    pub weight: Option<Decimal>,
    pub size: Option<Decimal>,
    pub axis: Option<i64>,
    pub grading_details: Option<GradingDetails>,
}

#[derive(Debug, Serialize)]
pub struct EditCollectedItem {
    #[serde(rename = "type")]
    pub type_id: Option<i64>,
    pub issue: Option<i64>,
    pub quantity: Option<i64>,
    pub grade: Option<Grade>,
    pub for_swap: Option<bool>,
    pub private_comment: Option<String>,
    pub public_comment: Option<String>,
    pub price: Option<ItemPrice>,
    pub collection: Option<i64>,
    pub storage_location: Option<String>,
    pub acquisition_place: Option<String>,
    pub acquisition_date: Option<chrono::NaiveDate>,
    pub serial_number: Option<String>,
    pub internal_id: Option<String>,
    pub weight: Option<Decimal>,
    pub size: Option<Decimal>,
    pub axis: Option<i64>,
    pub grading_details: Option<GradingDetails>,
}

#[derive(Debug, Serialize)]
pub struct ItemPrice {
    pub value: Decimal,
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
pub struct ClientBuilder<'a> {
    api_key: Option<Cow<'a, str>>,
    base_url: Option<Cow<'a, str>>,
    bearer_token: Option<Cow<'a, str>>,
}

impl<'a> ClientBuilder<'a> {
    /// Creates a new `ClientBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key to use for requests.
    pub fn api_key<S: Into<Cow<'a, str>>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL to use for requests.
    ///
    /// This is useful for testing.
    pub fn base_url<S: Into<Cow<'a, str>>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the bearer token to use for requests.
    pub fn bearer_token<S: Into<Cow<'a, str>>>(mut self, bearer_token: S) -> Self {
        self.bearer_token = Some(bearer_token.into());
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

        let base_url = self
            .base_url
            .map(|s| s.into_owned())
            .unwrap_or_else(|| "https://api.numista.com/v3".to_string());

        Ok(Client { client, base_url })
    }
}

fn serialize_lang<S>(lang: &Option<Language>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(l) = lang {
        serializer.serialize_some(l.to_639_1().unwrap())
    } else {
        serializer.serialize_none()
    }
}

/// Parameters for searching for types.
#[derive(Debug, Default, Serialize, Clone)]
pub struct SearchTypesParams<'a> {
    #[serde(serialize_with = "serialize_lang")]
    lang: Option<Language>,
    category: Option<Category>,
    q: Option<Cow<'a, str>>,
    issuer: Option<Cow<'a, str>>,
    catalogue: Option<i64>,
    number: Option<Cow<'a, str>>,
    ruler: Option<i64>,
    material: Option<i64>,
    year: Option<Cow<'a, str>>,
    date: Option<Cow<'a, str>>,
    size: Option<Cow<'a, str>>,
    weight: Option<Cow<'a, str>>,
    page: Option<i64>,
    count: Option<i64>,
}

impl<'a> SearchTypesParams<'a> {
    /// Creates a new `SearchTypesParams`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the language to use for the search.
    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    /// Sets the category to search in.
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Sets the search query.
    pub fn q<S: Into<Cow<'a, str>>>(mut self, q: S) -> Self {
        self.q = Some(q.into());
        self
    }

    /// Sets the issuer to search for.
    pub fn issuer<S: Into<Cow<'a, str>>>(mut self, issuer: S) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Sets the catalogue to search in.
    pub fn catalogue(mut self, catalogue: i64) -> Self {
        self.catalogue = Some(catalogue);
        self
    }

    /// Sets the number to search for in a catalogue.
    pub fn number<S: Into<Cow<'a, str>>>(mut self, number: S) -> Self {
        self.number = Some(number.into());
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

    /// Sets the year to a single year.
    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year.to_string().into());
        self
    }

    /// Sets the year to a range of years.
    pub fn year_range(mut self, min: i32, max: i32) -> Self {
        self.year = Some(format!("{}-{}", min, max).into());
        self
    }

    /// Sets the date to a single year.
    pub fn date(mut self, year: i32) -> Self {
        self.date = Some(year.to_string().into());
        self
    }

    /// Sets the date to a range of years.
    pub fn date_range(mut self, min: i32, max: i32) -> Self {
        self.date = Some(format!("{}-{}", min, max).into());
        self
    }

    /// Sets the size to search for.
    pub fn size<S: Into<Cow<'a, str>>>(mut self, size: S) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the weight to search for.
    pub fn weight<S: Into<Cow<'a, str>>>(mut self, weight: S) -> Self {
        self.weight = Some(weight.into());
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

}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use serde_json;

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
          .match_query(mockito::Matcher::UrlEncoded("lang".into(), "de".into()))
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

        let response = client.get_type(420, Some(Language::from_639_1("de").unwrap())).await.unwrap();

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

        let mock = server.mock("GET", "/types")
          .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
            mockito::Matcher::UrlEncoded("lang".into(), "es".into()),
          ]))
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

        let params = SearchTypesParams::new().q("victoria").lang(Language::from_639_1("es").unwrap());
        let response = client.search_types(&params).await.unwrap();

        mock.assert();
        assert_eq!(response.count, 1);
        assert_eq!(response.types.len(), 1);
        assert_eq!(response.types[0].id, 420);
    }

    #[test]
    fn search_types_params_year_date_test() {
        let params = SearchTypesParams::new().year(2000);
        assert_eq!(params.year.unwrap(), "2000");

        let params = SearchTypesParams::new().year_range(1990, 2005);
        assert_eq!(params.year.unwrap(), "1990-2005");

        let params = SearchTypesParams::new().date(1999);
        assert_eq!(params.date.unwrap(), "1999");

        let params = SearchTypesParams::new().date_range(1980, 1985);
        assert_eq!(params.date.unwrap(), "1980-1985");
    }

    #[tokio::test]
    async fn stream_all_types_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        server
            .mock("GET", "/types")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
                mockito::Matcher::UrlEncoded("page".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "count": 2,
                "types": [
                    { "id": 1, "title": "Type 1", "category": "coin", "issuer": {"code": "a", "name": "A"}, "min_year": 1, "max_year": 2 }
                ]
            }"#,
            )
            .create();

        server
            .mock("GET", "/types")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
                mockito::Matcher::UrlEncoded("page".into(), "2".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "count": 2,
                "types": [
                    { "id": 2, "title": "Type 2", "category": "coin", "issuer": {"code": "b", "name": "B"}, "min_year": 3, "max_year": 4 }
                ]
            }"#,
            )
            .create();

        server
            .mock("GET", "/types")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
                mockito::Matcher::UrlEncoded("page".into(), "3".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "count": 2,
                "types": []
            }"#,
            )
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key")
            .base_url(url)
            .build()
            .unwrap();

        let params = SearchTypesParams::new().q("victoria");
        let stream = client.stream_all_types(params);

        let results: Vec<Result<models::SearchTypeResult>> = stream.collect().await;
        let results: Result<Vec<models::SearchTypeResult>> = results.into_iter().collect();
        let results = results.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, 1);
        assert_eq!(results[1].id, 2);
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
            .api_key("test_key")
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

        let response = client.get_prices(420, 123, None, None).await.unwrap();

        mock.assert();
        assert_eq!(response.currency, iso_currency::Currency::USD);
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
            .with_body(r#"{"id": "L106610", "url": "https://example.com", "type": "volume", "title": "Test", "languages": []}"#)
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

        let mock = server.mock("GET", "/oauth_token")
            .match_query(mockito::Matcher::UrlEncoded("grant_type".into(), "client_credentials".into()))
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
            grant_type: models::GrantType::ClientCredentials,
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

    #[tokio::test]
    async fn search_by_image_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("POST", "/search_by_image")
            .match_body(mockito::Matcher::Json(serde_json::json!({
                "category": null,
                "images": [
                    {
                        "mime_type": "image/jpeg",
                        "image_data": "jpeg_data"
                    },
                    {
                        "mime_type": "image/png",
                        "image_data": "png_data"
                    }
                ],
                "max_results": null
            })))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count": 0, "types": []}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let request = models::SearchByImageRequest {
            category: None,
            images: vec![
                models::Image {
                    mime_type: models::MimeType::Jpeg,
                    image_data: "jpeg_data".to_string(),
                },
                models::Image {
                    mime_type: models::MimeType::Png,
                    image_data: "png_data".to_string(),
                },
            ],
            max_results: None,
        };
        client.search_by_image(&request).await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn unauthorized_error_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("GET", "/types/420")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error_message": "Invalid API key"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_type(420, None).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::Unauthorized => (),
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[tokio::test]
    async fn not_found_error_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("GET", "/types/999999")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error_message": "Not found"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key")
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_type(999999, None).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::NotFound => (),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn invalid_parameter_error_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("GET", "/types")
            .match_query(mockito::Matcher::UrlEncoded("q".into(), "a".repeat(101)))
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error_message": "Invalid parameter"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key")
            .base_url(url)
            .build()
            .unwrap();

        let params = SearchTypesParams::new().q("a".repeat(101));
        let response = client.search_types(&params).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::InvalidParameter(msg) => assert_eq!(msg, "Invalid parameter"),
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[tokio::test]
    async fn rate_limit_exceeded_error_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("GET", "/types/123")
            .with_status(429)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error_message": "Rate limit exceeded"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key")
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_type(123, None).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::RateLimitExceeded => (),
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }

    #[tokio::test]
    async fn no_user_associated_error_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("GET", "/oauth_token")
            .match_query(mockito::Matcher::UrlEncoded(
                "grant_type".into(),
                "client_credentials".into(),
            ))
            .with_status(501)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error_message": "No user associated"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key")
            .base_url(url)
            .build()
            .unwrap();

        let params = OAuthTokenParams {
            grant_type: models::GrantType::ClientCredentials,
            code: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
        };
        let response = client.get_oauth_token(&params).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::NoUserAssociatedWithApiKey => (),
            _ => panic!("Expected NoUserAssociatedWithApiKey error"),
        }
    }

    #[tokio::test]
    async fn generic_api_error_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("GET", "/types/420")
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error_message": "Internal Server Error"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_type(420, None).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::ApiError { message, status } => {
                assert_eq!(message, "Internal Server Error");
                assert_eq!(status, 500);
            }
            _ => panic!("Expected a generic ApiError"),
        }
    }
}
