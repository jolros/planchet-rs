use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Issuer {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    pub id: i64,
    pub name: String,
    pub full_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Value {
    pub text: Option<String>,
    pub numeric_value: Option<f64>,
    pub numerator: Option<i64>,
    pub denominator: Option<i64>,
    pub currency: Option<Currency>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RulingAuthority {
    pub id: i64,
    pub name: String,
    pub wikidata_id: Option<String>,
    pub nomisma_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Composition {
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Technique {
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Demonetization {
    pub is_demonetized: bool,
    pub demonetization_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinSide {
    pub engravers: Vec<String>,
    pub designers: Vec<String>,
    pub description: Option<String>,
    pub lettering: Option<String>,
    pub unabridged_legend: Option<String>,
    pub lettering_translation: Option<String>,
    pub picture: Option<String>,
    pub thumbnail: Option<String>,
    pub picture_copyright: Option<String>,
    pub picture_copyright_url: Option<String>,
    pub picture_license_name: Option<String>,
    pub picture_license_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mint {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Reference {
    pub catalogue: Catalogue,
    pub number: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Catalogue {
    pub id: i64,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Coin,
    Banknote,
    Exonumia,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NumistaType {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub category: Category,
    pub issuer: Issuer,
    pub min_year: Option<i64>,
    pub max_year: Option<i64>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub value: Option<Value>,
    pub ruler: Option<Vec<RulingAuthority>>,
    pub shape: Option<String>,
    pub composition: Option<Composition>,
    pub technique: Option<Technique>,
    pub demonetization: Option<Demonetization>,
    pub weight: Option<f64>,
    pub size: Option<f64>,
    pub thickness: Option<f64>,
    pub orientation: Option<String>,
    pub obverse: Option<CoinSide>,
    pub reverse: Option<CoinSide>,
    pub edge: Option<CoinSide>,
    pub watermark: Option<CoinSide>,
    pub mints: Option<Vec<Mint>>,
    pub comments: Option<String>,
    pub tags: Vec<String>,
    pub references: Option<Vec<Reference>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub is_dated: bool,
    pub year: Option<i64>,
    pub gregorian_year: Option<i64>,
    pub min_year: Option<i64>,
    pub max_year: Option<i64>,
    pub mint_letter: Option<String>,
    pub mintage: Option<i64>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchTypesResponse {
    pub count: i64,
    pub types: Vec<SearchTypeResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchTypeResult {
    pub id: i64,
    pub title: String,
    pub category: Category,
    pub issuer: Issuer,
    pub min_year: Option<i64>,
    pub max_year: Option<i64>,
    pub obverse_thumbnail: Option<String>,
    pub reverse_thumbnail: Option<String>,
}