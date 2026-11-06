use crate::model::{Category, Grade, GrantType, SearchOrder};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OAuthTokenParams {
    pub grant_type: GrantType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl OAuthTokenParams {
    pub fn new(grant_type: GrantType) -> Self {
        Self {
            grant_type,
            code: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
        }
    }

    /// Creates parameters for the `client_credentials` grant type.
    pub fn client_credentials() -> Self {
        Self::new(GrantType::ClientCredentials)
    }

    /// Creates parameters for the `authorization_code` grant type.
    pub fn authorization_code<S1, S2, S3, S4>(
        code: S1,
        client_id: S2,
        client_secret: S3,
        redirect_uri: S4,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            grant_type: GrantType::AuthorizationCode,
            code: Some(code.into()),
            client_id: Some(client_id.into()),
            client_secret: Some(client_secret.into()),
            redirect_uri: Some(redirect_uri.into()),
            scope: None,
        }
    }

    pub fn code<S: Into<String>>(mut self, code: S) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn client_id<S: Into<String>>(mut self, client_id: S) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn client_secret<S: Into<String>>(mut self, client_secret: S) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn redirect_uri<S: Into<String>>(mut self, redirect_uri: S) -> Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    pub fn scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.scope = Some(scope.into());
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct GetCollectedItemsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) category: Option<Category>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub(crate) type_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

macro_rules! impl_collected_item_common_setters {
    () => {
        pub fn issue(mut self, issue: i64) -> Self {
            self.issue = Some(issue);
            self
        }

        pub fn quantity(mut self, quantity: i64) -> Self {
            self.quantity = Some(quantity);
            self
        }

        pub fn grade(mut self, grade: Grade) -> Self {
            self.grade = Some(grade);
            self
        }

        pub fn for_swap(mut self, for_swap: bool) -> Self {
            self.for_swap = Some(for_swap);
            self
        }

        pub fn private_comment<S: Into<String>>(mut self, private_comment: S) -> Self {
            self.private_comment = Some(private_comment.into());
            self
        }

        pub fn public_comment<S: Into<String>>(mut self, public_comment: S) -> Self {
            self.public_comment = Some(public_comment.into());
            self
        }

        pub fn price(mut self, price: ItemPriceParams) -> Self {
            self.price = Some(price);
            self
        }

        pub fn collection(mut self, collection: i64) -> Self {
            self.collection = Some(collection);
            self
        }

        pub fn storage_location<S: Into<String>>(mut self, storage_location: S) -> Self {
            self.storage_location = Some(storage_location.into());
            self
        }

        pub fn acquisition_place<S: Into<String>>(mut self, acquisition_place: S) -> Self {
            self.acquisition_place = Some(acquisition_place.into());
            self
        }

        pub fn acquisition_date(mut self, acquisition_date: chrono::NaiveDate) -> Self {
            self.acquisition_date = Some(acquisition_date);
            self
        }

        pub fn serial_number<S: Into<String>>(mut self, serial_number: S) -> Self {
            self.serial_number = Some(serial_number.into());
            self
        }

        pub fn internal_id<S: Into<String>>(mut self, internal_id: S) -> Self {
            self.internal_id = Some(internal_id.into());
            self
        }

        pub fn weight(mut self, weight: Decimal) -> Self {
            self.weight = Some(weight);
            self
        }

        pub fn size(mut self, size: Decimal) -> Self {
            self.size = Some(size);
            self
        }

        pub fn axis(mut self, axis: i64) -> Self {
            self.axis = Some(axis);
            self
        }

        pub fn grading_details(mut self, grading_details: GradingDetailsParams) -> Self {
            self.grading_details = Some(grading_details);
            self
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddCollectedItemParams {
    #[serde(rename = "type")]
    pub type_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_swap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<ItemPriceParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grading_details: Option<GradingDetailsParams>,
}

impl AddCollectedItemParams {
    pub fn new(type_id: i64) -> Self {
        Self {
            type_id,
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
        }
    }

    impl_collected_item_common_setters!();
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct EditCollectedItemParams {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_swap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<ItemPriceParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grading_details: Option<GradingDetailsParams>,
}

impl EditCollectedItemParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn type_id(mut self, type_id: i64) -> Self {
        self.type_id = Some(type_id);
        self
    }

    impl_collected_item_common_setters!();
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ItemPriceParams {
    pub value: Decimal,
    pub currency: String,
}

impl ItemPriceParams {
    pub fn new<S: Into<String>>(value: Decimal, currency: S) -> Self {
        Self {
            value,
            currency: currency.into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct GradingDetailsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grading_company: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slab_grade: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slab_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cac_sticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grading_designations: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grading_strike: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grading_surface: Option<i64>,
}

impl GradingDetailsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grading_company(mut self, grading_company: i64) -> Self {
        self.grading_company = Some(grading_company);
        self
    }

    pub fn slab_grade(mut self, slab_grade: i64) -> Self {
        self.slab_grade = Some(slab_grade);
        self
    }

    pub fn slab_number<S: Into<String>>(mut self, slab_number: S) -> Self {
        self.slab_number = Some(slab_number.into());
        self
    }

    pub fn cac_sticker<S: Into<String>>(mut self, cac_sticker: S) -> Self {
        self.cac_sticker = Some(cac_sticker.into());
        self
    }

    pub fn grading_designations(mut self, grading_designations: Vec<i64>) -> Self {
        self.grading_designations = Some(grading_designations);
        self
    }

    pub fn grading_strike(mut self, grading_strike: i64) -> Self {
        self.grading_strike = Some(grading_strike);
        self
    }

    pub fn grading_surface(mut self, grading_surface: i64) -> Self {
        self.grading_surface = Some(grading_surface);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SearchByImageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    pub images: Vec<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip)]
    pub activate_experimental_features: Option<bool>,
}

impl SearchByImageParams {
    pub fn new(images: Vec<Image>) -> Self {
        Self {
            category: None,
            images,
            max_results: None,
            activate_experimental_features: None,
        }
    }

    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    pub fn max_results(mut self, max_results: i64) -> Self {
        self.max_results = Some(max_results);
        self
    }

    pub fn activate_experimental_features(
        mut self,
        activate_experimental_features: bool,
    ) -> Self {
        self.activate_experimental_features = Some(activate_experimental_features);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MimeType {
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Image {
    pub mime_type: MimeType,
    /// The image data, Base64-encoded.
    pub image_data: String,
}

/// Parameters for searching for types.
#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct SearchTypesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) object_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) catalogue: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ruler: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) material: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order: Option<SearchOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<i64>,
}

impl SearchTypesParams {
    /// Creates a new `SearchTypesParams`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the category to search in.
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Sets the object type ID to search for.
    pub fn object_type(mut self, object_type: i64) -> Self {
        self.object_type = Some(object_type);
        self
    }

    /// Sets the search query.
    pub fn q<S: Into<String>>(mut self, q: S) -> Self {
        self.q = Some(q.into());
        self
    }

    /// Sets the issuer to search for.
    pub fn issuer<S: Into<String>>(mut self, issuer: S) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Sets the catalogue to search in.
    pub fn catalogue(mut self, catalogue: i64) -> Self {
        self.catalogue = Some(catalogue);
        self
    }

    /// Sets the number to search for in a catalogue.
    pub fn number<S: Into<String>>(mut self, number: S) -> Self {
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
        self.year = Some(year.to_string());
        self
    }

    /// Sets the year to a range of years.
    pub fn year_range(mut self, min: i32, max: i32) -> Self {
        self.year = Some(format!("{}-{}", min, max));
        self
    }

    /// Sets the date to a single year.
    pub fn date(mut self, year: i32) -> Self {
        self.date = Some(year.to_string());
        self
    }

    /// Sets the date to a range of years.
    pub fn date_range(mut self, min: i32, max: i32) -> Self {
        self.date = Some(format!("{}-{}", min, max));
        self
    }

    /// Sets the size to search for.
    pub fn size<S: Into<String>>(mut self, size: S) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the weight to search for.
    pub fn weight<S: Into<String>>(mut self, weight: S) -> Self {
        self.weight = Some(weight.into());
        self
    }

    /// Sets the sort order for the search results.
    pub fn order(mut self, order: SearchOrder) -> Self {
        self.order = Some(order);
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

/// Parameters for getting sales records of a type.
#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct GetSalesRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) issue_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<i64>,
}

impl GetSalesRecordsParams {
    /// Creates a new `GetSalesRecordsParams`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the issue ID to filter sales records by.
    pub fn issue_id(mut self, issue_id: i64) -> Self {
        self.issue_id = Some(issue_id);
        self
    }

    /// Sets the currency (3-letter ISO 4217 code) to convert sales prices into.
    pub fn currency<S: Into<String>>(mut self, currency: S) -> Self {
        self.currency = Some(currency.into());
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
