use crate::error::{ApiError, Error, Result};
use crate::models::{
    self,
    request::{
        AddCollectedItemParams, EditCollectedItemParams, GetCollectedItemsParams, OAuthTokenParams,
        SearchByImageParams, SearchTypesParams,
    },
    response::{
        CataloguesResponse, CollectionsResponse, IssuersResponse, MintsResponse,
        SearchByImageResponse, SearchTypesResponse,
    },
    CollectedItem, CollectedItems, GradePrices, MintDetail, NumistaType, OAuthToken, Publication,
    User,
};
use futures::stream::{self, Stream};
use http::Extensions;
use isolang::Language;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_middleware::{ClientBuilder as MiddlewareClientBuilder, ClientWithMiddleware, Middleware, Next};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{info_span, trace, Instrument};

/// The main client for interacting with the Numista API.
#[derive(Debug, Clone)]
pub struct Client {
    client: ClientWithMiddleware,
    base_url: String,
    lang: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ApiErrorResponse {
    error_message: String,
}

async fn parse_api_error(response: reqwest::Response) -> Error {
    let status_code = response.status().as_u16();
    let api_error_response = match response.json::<ApiErrorResponse>().await {
        Ok(api_error) => api_error,
        Err(e) => return e.into(),
    };

    Error::ApiError(ApiError {
        message: api_error_response.error_message,
        status: status_code,
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
    ) -> Result<GradePrices> {
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
        params: &SearchTypesParams,
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
        params: SearchTypesParams,
    ) -> impl Stream<Item = Result<models::SearchTypeResult>> + 'a {
        struct State {
            client: Client,
            params: SearchTypesParams,
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
            params = params.page(state.current_page);

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
    ) -> Result<CollectedItems> {
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
        item: &AddCollectedItemParams,
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
        item: &EditCollectedItemParams,
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
        request: &SearchByImageParams,
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

/// A builder for creating a `Client`.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    bearer_token: Option<String>,
    lang: Option<Language>,
}

impl ClientBuilder {
    /// Creates a new `ClientBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key to use for requests.
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL to use for requests.
    ///
    /// This is useful for testing.
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the bearer token to use for requests.
    pub fn bearer_token<S: Into<String>>(mut self, bearer_token: S) -> Self {
        self.bearer_token = Some(bearer_token.into());
        self
    }

    /// Sets the language to use for requests.
    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    /// Sets the language code to use for requests.
    pub fn lang_code<S: Into<String>>(mut self, lang_code: S) -> Self {
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
            .unwrap_or_else(|| "https://api.numista.com/v3".to_string());

        let lang = self.lang.and_then(|l| l.to_639_1().map(|s| s.to_string()));

        Ok(Client {
            client,
            base_url,
            lang,
        })
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
}
