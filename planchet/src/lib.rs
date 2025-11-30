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
pub mod de;
pub mod models;

use futures::stream::{self, Stream};
use isolang::Language;
use models::{
    CataloguesResponse, Category, CollectedItem, CollectedItemsResponse, CollectionsResponse,
    Grade, IssuersResponse, MintDetail, MintsResponse, NumistaType, OAuthToken, PricesResponse,
    Publication, SearchByImageResponse, SearchTypesResponse, User,
};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_middleware::{ClientBuilder as MiddlewareClientBuilder, ClientWithMiddleware, Middleware, Next};
use http::Extensions;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
use std::fmt;
use tracing::{info_span, trace, Instrument};
/// A specific kind of API error.
#[derive(Debug, PartialEq)]
pub enum KnownApiError {
    /// The provided API key is invalid or has expired (HTTP 401).
    Unauthorized,
    /// The requested resource could not be found (HTTP 404).
    NotFound,
    /// A parameter in the request was invalid or missing (HTTP 400).
    InvalidParameter,
    /// The API rate limit has been exceeded (HTTP 429).
    RateLimitExceeded,
    /// No user is associated with the provided API key (HTTP 501).
    /// This is specific to the `client_credentials` grant type.
    NoUserAssociatedWithApiKey,
}

/// An error returned by the Numista API.
#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
    pub kind: Option<KnownApiError>,
}

/// The error type for this crate.
#[derive(Debug)]
pub enum Error {
    /// The API key was not provided in the `ClientBuilder`.
    ApiKeyMissing,
    /// An error related to the underlying HTTP client or middleware stack.
    Request(Box<dyn std::error::Error + Send + Sync>),
    /// An error from `serde_json`.
    Json(serde_json::Error),
    /// An error returned by the Numista API.
    ApiError(ApiError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiKeyMissing => write!(f, "Numista API key is required"),
            Error::Request(e) => write!(f, "Request error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::ApiError(e) => write!(f, "API error (status {}): {}", e.status, e.message),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(Box::new(err))
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(err: reqwest_middleware::Error) -> Self {
        Error::Request(Box::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

/// A `Result` type alias for this crate's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// The main client for interacting with the Numista API.
#[derive(Debug, Clone)]
pub struct Client {
    client: ClientWithMiddleware,
    base_url: String,
    lang: Option<String>,
}

async fn parse_api_error(response: reqwest::Response) -> Error {
    let status_code = response.status().as_u16();
    let api_error_response = match response.json::<models::ApiError>().await {
        Ok(api_error) => api_error,
        Err(e) => return e.into(),
    };

    let kind = match status_code {
        400 => Some(KnownApiError::InvalidParameter),
        401 => Some(KnownApiError::Unauthorized),
        404 => Some(KnownApiError::NotFound),
        429 => Some(KnownApiError::RateLimitExceeded),
        501 => Some(KnownApiError::NoUserAssociatedWithApiKey),
        _ => None,
    };

    Error::ApiError(ApiError {
        message: api_error_response.error_message,
        status: status_code,
        kind,
    })
}

async fn process_response<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T> {
    if response.status().is_success() {
        return Ok(response.json::<T>().await?);
    }

    Err(parse_api_error(response).await)
}

#[derive(Default)]
struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let span = info_span!(
            "Request",
            method = %req.method(),
            url = %req.url(),
        );

        async move {
            trace!("Request headers: {:?}", req.headers());
            if let Some(body) = req.body() {
                if let Some(bytes) = body.as_bytes() {
                    if let Ok(str_body) = std::str::from_utf8(bytes) {
                        trace!("Request body: {}", str_body);
                    }
                }
            }

            let res = next.run(req, extensions).await;

            match res {
                Ok(response) => {
                    let status = response.status();
                    let headers = response.headers().clone();
                    let body_bytes = match response.bytes().await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            return Err(reqwest_middleware::Error::Reqwest(e));
                        }
                    };

                    trace!("Response status: {}", status);
                    trace!("Response headers: {:?}", headers);
                    if let Ok(str_body) = std::str::from_utf8(&body_bytes) {
                        if !str_body.is_empty() {
                            trace!("Response body: {}", str_body);
                        }
                    }

                    let new_body = reqwest::Body::from(body_bytes);
                    let mut new_response_builder = http::Response::builder()
                        .status(status);
                    *new_response_builder.headers_mut().unwrap() = headers;
                    let new_response = new_response_builder.body(new_body).unwrap();

                    Ok(reqwest::Response::from(new_response))
                }
                Err(e) => {
                    trace!("Request failed: {:?}", e);
                    Err(e)
                }
            }
        }
        .instrument(span)
        .await
    }
}

macro_rules! add_lang_param {
    ($self:expr, $req:expr) => {
        if let Some(ref l) = $self.lang {
            $req = $req.query(&[("lang", l)]);
        }
    };
}

impl Client {
    async fn get_request<T, Q>(&self, path: &str, query: Option<&Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url);
        add_lang_param!(self, req);
        if let Some(q) = query {
            req = req.query(q);
        }
        let response = req.send().await?;
        process_response(response).await
    }

    /// Gets a single type from the Numista catalogue.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The ID of the type to get.
    pub async fn get_type(&self, type_id: i64) -> Result<NumistaType> {
        self.get_request(&format!("/types/{}", type_id), None::<&()>)
            .await
    }

    /// Gets the issues of a type.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The ID of the type to get the issues for.
    pub async fn get_issues(&self, type_id: i64) -> Result<Vec<models::Issue>> {
        self.get_request(&format!("/types/{}/issues", type_id), None::<&()>)
            .await
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
        #[derive(Serialize)]
        struct GetPricesParams<'a> {
            currency: Option<&'a str>,
        }

        let params = GetPricesParams { currency };

        self.get_request(
            &format!("/types/{}/issues/{}/prices", type_id, issue_id),
            Some(&params),
        )
        .await
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
        self.get_request("/types", Some(params)).await
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
    pub async fn get_issuers(&self) -> Result<IssuersResponse> {
        self.get_request("/issuers", None::<&()>).await
    }

    /// Gets the list of mints.
    pub async fn get_mints(&self) -> Result<MintsResponse> {
        self.get_request("/mints", None::<&()>).await
    }

    /// Gets a single mint.
    ///
    /// # Arguments
    ///
    /// * `mint_id` - The ID of the mint to get.
    pub async fn get_mint(&self, mint_id: i64) -> Result<MintDetail> {
        self.get_request(&format!("/mints/{}", mint_id), None::<&()>)
            .await
    }

    /// Gets the list of catalogues.
    pub async fn get_catalogues(&self) -> Result<CataloguesResponse> {
        self.get_request("/catalogues", None::<&()>).await
    }

    /// Gets a single publication.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the publication to get.
    pub async fn get_publication(&self, id: &str) -> Result<Publication> {
        self.get_request(&format!("/publications/{}", id), None::<&()>)
            .await
    }

    /// Gets a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to get.
    pub async fn get_user(&self, user_id: i64) -> Result<User> {
        self.get_request(&format!("/users/{}", user_id), None::<&()>)
            .await
    }

    /// Gets the collections of a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to get the collections for.
    pub async fn get_user_collections(&self, user_id: i64) -> Result<CollectionsResponse> {
        self.get_request(&format!("/users/{}/collections", user_id), None::<&()>)
            .await
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
        self.get_request(
            &format!("/users/{}/collected_items", user_id),
            Some(params),
        )
        .await
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
        let url = format!("{}/users/{}/collected_items", self.base_url, user_id);
        let mut req = self.client.post(&url);
        add_lang_param!(self, req);
        let response = req
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(item)?)
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
        self.get_request(
            &format!("/users/{}/collected_items/{}", user_id, item_id),
            None::<&()>,
        )
        .await
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
        let url = format!(
            "{}/users/{}/collected_items/{}",
            self.base_url, user_id, item_id
        );
        let mut req = self.client.patch(&url);
        add_lang_param!(self, req);
        let response = req
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(item)?)
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
        let url = format!(
            "{}/users/{}/collected_items/{}",
            self.base_url, user_id, item_id
        );
        let mut req = self.client.delete(&url);
        add_lang_param!(self, req);
        let response = req.send().await?;

        if response.status().is_success() {
            return Ok(());
        }

        Err(parse_api_error(response).await)
    }

    /// Gets an OAuth token.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for getting the token.
    pub async fn get_oauth_token(&self, params: &OAuthTokenParams) -> Result<OAuthToken> {
        self.get_request("/oauth_token", Some(params)).await
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
        let url = format!("{}/search_by_image", self.base_url);
        let mut req = self.client.post(&url);
        add_lang_param!(self, req);
        let response = req
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(request)?)
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
    lang: Option<Language>,
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

    /// Sets the language to use for requests.
    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    /// Sets the language code to use for requests.
    pub fn lang_code<S: Into<Cow<'a, str>>>(mut self, lang_code: S) -> Self {
        if let Some(l) = Language::from_639_1(&lang_code.into().to_lowercase()) {
            self.lang = Some(l);
        }
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
            let mut auth_value =
                HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap();
            auth_value.set_sensitive(true);
            headers.insert("Authorization", auth_value);
        }

        let reqwest_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let client = MiddlewareClientBuilder::new(reqwest_client)
            .with(LoggingMiddleware)
            .build();

        let base_url = self
            .base_url
            .map(|s| s.into_owned())
            .unwrap_or_else(|| "https://api.numista.com/v3".to_string());

        let lang = self.lang.and_then(|l| l.to_639_1().map(|s| s.to_string()));

        Ok(Client {
            client,
            base_url,
            lang,
        })
    }
}

/// Parameters for searching for types.
#[derive(Debug, Default, Serialize, Clone)]
pub struct SearchTypesParams<'a> {
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
    async fn get_publication_full_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/publications/L106610")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
              "id": "L106610",
              "url": "https://numista.com/L106610",
              "type": "volume",
              "title": "Cast Chinese Coins",
              "bibliographical_notice": "David Hartill; 2017. <em>Cast Chinese Coins</em> (2<sup>nd</sup> Edition). Self-published, London, United Kingdom.",
              "edition": "2nd Edition",
              "languages": [
                "en"
              ],
              "year": "2017",
              "page_count": 453,
              "cover": "softcover",
              "isbn10": "1787194949",
              "isbn13": "9781787194946",
              "oclc_number": "1000342699",
              "contributors": [
                {
                  "role": "author",
                  "name": "David Hartill",
                  "id": "369"
                }
              ],
              "publishers": [
                {
                  "name": "Self-published",
                  "id": "93"
                }
              ],
              "publication_places": [
                {
                  "name": "London, United Kingdom",
                  "geonames_id": "2643743"
                }
              ],
              "part_of": [
                {
                  "type": "volume_group",
                  "id": "L111322",
                  "title": "Cast Chinese Coins"
                }
              ]
            }"#)
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
            .lang_code("de")
            .build()
            .unwrap();

        let response = client.get_type(420).await.unwrap();

        mock.assert();
        assert_eq!(response.id, 420);
        assert_eq!(response.title, "5 Cents - Victoria");
        assert_eq!(
            response.type_name.unwrap(),
            "Standard circulation coin"
        );
    }

    #[tokio::test]
    async fn get_type_full_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/types/99700")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":99700,"url":"https:\/\/en.numista.com\/99700","title":"\u00bc Dollar \"Washington Quarter\" (George Rogers Clark National Historical Park, Indiana)","category":"coin","issuer":{"code":"etats-unis","name":"United States"},"min_year":2017,"max_year":2017,"type":"Circulating commemorative coins","ruler":[{"id":4720,"name":"Federal republic","wikidata_id":"Q30"}],"value":{"text":"\u00bc Dollar ","numeric_value":0.25,"numerator":1,"denominator":4,"currency":{"id":59,"name":"Dollar","full_name":"Dollar (1785-date)"}},"demonetization":{"is_demonetized":false},"size":24.3,"thickness":1.75,"shape":"Round","composition":{"text":"Copper-nickel clad copper"},"technique":{"text":"Milled"},"obverse":{"engravers":["William Cousins"],"designers":["John Flanagan"],"description":"The portrait in left profile of George Washington, the first President of the United States from 1789 to 1797, is accompanied with the motto \"IN GOD WE TRUST\" and the lettering \"LIBERTY\" surrounded with the denomination and the inscription \"UNITED STATES OF AMERICA\"","lettering":"UNITED STATES OF AMERICA\r\nIN \r\nGOD WE \r\nTRUST\r\nLIBERTY  P\r\nJF  WC\r\nQUARTER DOLLAR","lettering_scripts":[{"name":"Latin"}],"picture":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5044-original.jpg","thumbnail":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5044-180.jpg","picture_copyright":"Image courtesy of United States Mint"},"reverse":{"engravers":["Frank Morris","Michael Gaudioso"],"description":"George Rogers Clark leading his men through the flooded plains approaching Fort Sackville (frontier settlement of Vincennes).","lettering":"GEORGE ROGERS CLARK\r\nMG\r\nFM\r\nINDIANA   2017   E PLURIBUS UNUM","lettering_scripts":[{"name":"Latin"}],"picture":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5045-original.jpg","thumbnail":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/5045-180.jpg","picture_copyright":"United States Mint","picture_copyright_url":"http:\/\/www.usmint.gov"},"series":"United States Mint's \"America the Beautiful\" Quarters Program","commemorated_topic":"George Rogers Clark National Historical Park, Indiana","tags":["Firearms","War","Park"],"references":[{"catalogue":{"id":3,"code":"KM"},"number":"657"}],"weight":5.67,"orientation":"coin","edge":{"description":"Reeded","picture":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/4024-original.jpg","thumbnail":"https:\/\/en.numista.com\/catalogue\/photos\/etats-unis\/4024-180.jpg","picture_copyright":"Cyrillius"},"mints":[{"id":"10","name":"United States Mint of Denver"},{"id":"11","name":"United States Mint of Philadelphia"},{"id":"12","name":"United States Mint of San Francisco"}]}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_type(99700).await.unwrap();

        mock.assert();
        assert_eq!(response.id, 99700);
        assert_eq!(response.url.unwrap().as_str(), "https://en.numista.com/99700");
        assert_eq!(response.title, "¼ Dollar \"Washington Quarter\" (George Rogers Clark National Historical Park, Indiana)");
        assert_eq!(response.category.to_string(), "Coin");
        let issuer = response.issuer.unwrap();
        assert_eq!(issuer.code, "etats-unis");
        assert_eq!(issuer.name, "United States");
        assert_eq!(response.min_year.unwrap(), 2017);
        assert_eq!(response.max_year.unwrap(), 2017);
        assert_eq!(response.type_name.unwrap(), "Circulating commemorative coins");
        let ruler = response.ruler.unwrap();
        assert_eq!(ruler.len(), 1);
        assert_eq!(ruler[0].id, 4720);
        assert_eq!(ruler[0].name, "Federal republic");
        assert_eq!(ruler[0].wikidata_id.as_ref().unwrap(), "Q30");
        let value = response.value.unwrap();
        assert_eq!(value.text.unwrap(), "¼ Dollar ");
        assert_eq!(value.numeric_value.unwrap(), Decimal::new(25, 2));
        assert_eq!(value.numerator.unwrap(), 1);
        assert_eq!(value.denominator.unwrap(), 4);
        let currency = value.currency.unwrap();
        assert_eq!(currency.id, 59);
        assert_eq!(currency.name, "Dollar");
        assert_eq!(currency.full_name, "Dollar (1785-date)");
        assert_eq!(response.demonetization.unwrap().is_demonetized, false);
        assert_eq!(response.size.unwrap(), Decimal::new(243, 1));
        assert_eq!(response.thickness.unwrap(), Decimal::new(175, 2));
        assert_eq!(response.shape.unwrap(), "Round");
        assert_eq!(response.composition.unwrap().text.unwrap(), "Copper-nickel clad copper");
        assert_eq!(response.technique.unwrap().text.unwrap(), "Milled");
        let obverse = response.obverse.unwrap();
        assert_eq!(obverse.engravers.unwrap(), vec!["William Cousins"]);
        assert_eq!(obverse.designers.unwrap(), vec!["John Flanagan"]);
        assert_eq!(obverse.description.unwrap(), "The portrait in left profile of George Washington, the first President of the United States from 1789 to 1797, is accompanied with the motto \"IN GOD WE TRUST\" and the lettering \"LIBERTY\" surrounded with the denomination and the inscription \"UNITED STATES OF AMERICA\"");
        assert_eq!(obverse.lettering.unwrap(), "UNITED STATES OF AMERICA\r\nIN \r\nGOD WE \r\nTRUST\r\nLIBERTY  P\r\nJF  WC\r\nQUARTER DOLLAR");
        let obverse_lettering_scripts = obverse.lettering_scripts.unwrap();
        assert_eq!(obverse_lettering_scripts.len(), 1);
        assert_eq!(obverse_lettering_scripts[0].name, "Latin");
        assert_eq!(obverse.picture.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5044-original.jpg");
        assert_eq!(obverse.thumbnail.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5044-180.jpg");
        assert_eq!(obverse.picture_copyright.unwrap(), "Image courtesy of United States Mint");
        let reverse = response.reverse.unwrap();
        assert_eq!(reverse.engravers.unwrap(), vec!["Frank Morris", "Michael Gaudioso"]);
        assert_eq!(reverse.description.unwrap(), "George Rogers Clark leading his men through the flooded plains approaching Fort Sackville (frontier settlement of Vincennes).");
        assert_eq!(reverse.lettering.unwrap(), "GEORGE ROGERS CLARK\r\nMG\r\nFM\r\nINDIANA   2017   E PLURIBUS UNUM");
        let reverse_lettering_scripts = reverse.lettering_scripts.unwrap();
        assert_eq!(reverse_lettering_scripts.len(), 1);
        assert_eq!(reverse_lettering_scripts[0].name, "Latin");
        assert_eq!(reverse.picture.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5045-original.jpg");
        assert_eq!(reverse.thumbnail.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/5045-180.jpg");
        assert_eq!(reverse.picture_copyright.unwrap(), "United States Mint");
        assert_eq!(reverse.picture_copyright_url.unwrap().as_str(), "http://www.usmint.gov/");
        assert_eq!(response.series.unwrap(), "United States Mint's \"America the Beautiful\" Quarters Program");
        assert_eq!(response.commemorated_topic.unwrap(), "George Rogers Clark National Historical Park, Indiana");
        assert_eq!(response.tags.unwrap(), vec!["Firearms", "War", "Park"]);
        let references = response.references.unwrap();
        assert_eq!(references.len(), 1);
        assert_eq!(references[0].catalogue.id, 3);
        assert_eq!(references[0].catalogue.code, "KM");
        assert_eq!(references[0].number, "657");
        assert_eq!(response.weight.unwrap(), Decimal::new(567, 2));
        assert_eq!(response.orientation.unwrap(), models::Orientation::Coin);
        let edge = response.edge.unwrap();
        assert_eq!(edge.description.unwrap(), "Reeded");
        assert_eq!(edge.picture.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/4024-original.jpg");
        assert_eq!(edge.thumbnail.unwrap().as_str(), "https://en.numista.com/catalogue/photos/etats-unis/4024-180.jpg");
        assert_eq!(edge.picture_copyright.unwrap(), "Cyrillius");
        let mints = response.mints.unwrap();
        assert_eq!(mints.len(), 3);
        assert_eq!(mints[0].id, 10);
        assert_eq!(mints[0].name, "United States Mint of Denver");
        assert_eq!(mints[1].id, 11);
        assert_eq!(mints[1].name, "United States Mint of Philadelphia");
        assert_eq!(mints[2].id, 12);
        assert_eq!(mints[2].name, "United States Mint of San Francisco");
    }

    #[tokio::test]
    async fn search_types_test() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server.mock("GET", "/types")
          .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("q".into(), "victoria".into()),
            mockito::Matcher::UrlEncoded("lang".into(), "es".into()),
            mockito::Matcher::UrlEncoded("category".into(), "coin".into()),
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
            .lang_code("es")
            .build()
            .unwrap();

        let params = SearchTypesParams::new()
            .q("victoria")
            .category(Category::Coin);
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

        let response = client.get_issues(420).await.unwrap();

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

        let response = client.get_issuers().await.unwrap();

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

        let response = client.get_mints().await.unwrap();

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
            .with_body(r#"{"id": "1"}"#)
            .create();

        let client = ClientBuilder::new()
            .api_key("test_key".to_string())
            .base_url(url)
            .build()
            .unwrap();

        let response = client.get_mint(1).await.unwrap();

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

        let response = client.get_user(1).await.unwrap();

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

        let response = client.get_type(420).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::ApiError(e) => {
                assert_eq!(e.status, 401);
                assert_eq!(e.kind, Some(KnownApiError::Unauthorized));
            }
            _ => panic!("Expected ApiError"),
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

        let response = client.get_type(999999).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::ApiError(e) => {
                assert_eq!(e.status, 404);
                assert_eq!(e.kind, Some(KnownApiError::NotFound));
            }
            _ => panic!("Expected ApiError"),
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
            Error::ApiError(e) => {
                assert_eq!(e.status, 400);
                assert_eq!(e.message, "Invalid parameter");
                assert_eq!(e.kind, Some(KnownApiError::InvalidParameter));
            }
            _ => panic!("Expected ApiError"),
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

        let response = client.get_type(123).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::ApiError(e) => {
                assert_eq!(e.status, 429);
                assert_eq!(e.kind, Some(KnownApiError::RateLimitExceeded));
            }
            _ => panic!("Expected ApiError"),
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
            Error::ApiError(e) => {
                assert_eq!(e.status, 501);
                assert_eq!(e.kind, Some(KnownApiError::NoUserAssociatedWithApiKey));
            }
            _ => panic!("Expected ApiError"),
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

        let response = client.get_type(420).await;

        mock.assert();
        assert!(response.is_err());
        match response.err().unwrap() {
            Error::ApiError(e) => {
                assert_eq!(e.message, "Internal Server Error");
                assert_eq!(e.status, 500);
                assert!(e.kind.is_none());
            }
            _ => panic!("Expected a generic ApiError"),
        }
    }
}
