use crate::models::{Category, Grade, GrantType};
use chrono;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct OAuthTokenParams {
    pub grant_type: GrantType,
    pub code: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct GetCollectedItemsParams {
    pub(crate) category: Option<Category>,
    #[serde(rename = "type")]
    pub(crate) type_id: Option<i64>,
    pub(crate) collection: Option<i64>,
}

impl GetCollectedItemsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: Category) -> Self {
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
pub struct AddCollectedItemParams {
    #[serde(rename = "type")]
    pub type_id: i64,
    pub issue: Option<i64>,
    pub quantity: Option<i64>,
    pub grade: Option<Grade>,
    pub for_swap: Option<bool>,
    pub private_comment: Option<String>,
    pub public_comment: Option<String>,
    pub price: Option<ItemPriceParams>,
    pub collection: Option<i64>,
    pub storage_location: Option<String>,
    pub acquisition_place: Option<String>,
    pub acquisition_date: Option<chrono::NaiveDate>,
    pub serial_number: Option<String>,
    pub internal_id: Option<String>,
    pub weight: Option<Decimal>,
    pub size: Option<Decimal>,
    pub axis: Option<i64>,
    pub grading_details: Option<GradingDetailsParams>,
}

#[derive(Debug, Serialize)]
pub struct EditCollectedItemParams {
    #[serde(rename = "type")]
    pub type_id: Option<i64>,
    pub issue: Option<i64>,
    pub quantity: Option<i64>,
    pub grade: Option<Grade>,
    pub for_swap: Option<bool>,
    pub private_comment: Option<String>,
    pub public_comment: Option<String>,
    pub price: Option<ItemPriceParams>,
    pub collection: Option<i64>,
    pub storage_location: Option<String>,
    pub acquisition_place: Option<String>,
    pub acquisition_date: Option<chrono::NaiveDate>,
    pub serial_number: Option<String>,
    pub internal_id: Option<String>,
    pub weight: Option<Decimal>,
    pub size: Option<Decimal>,
    pub axis: Option<i64>,
    pub grading_details: Option<GradingDetailsParams>,
}

#[derive(Debug, Serialize)]
pub struct ItemPriceParams {
    pub value: Decimal,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct GradingDetailsParams {
    pub grading_company: Option<i64>,
    pub slab_grade: Option<i64>,
    pub slab_number: Option<String>,
    pub cac_sticker: Option<String>,
    pub grading_designations: Option<Vec<i64>>,
    pub grading_strike: Option<i64>,
    pub grading_surface: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchByImageParams {
    pub category: Option<Category>,
    pub images: Vec<Image>,
    pub max_results: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MimeType {
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub mime_type: MimeType,
    /// The image data, Base64-encoded.
    pub image_data: String,
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
