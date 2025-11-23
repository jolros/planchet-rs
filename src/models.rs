use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Mark {
    pub id: i64,
    pub title: Option<String>,
    pub picture: Option<String>,
    pub letters: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Signature {
    pub signer_name: String,
    pub signer_title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GradePrice {
    pub grade: String,
    pub price: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemPrice {
    pub value: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PricesResponse {
    pub currency: String,
    pub prices: Vec<GradePrice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssuerDetail {
    pub code: String,
    pub name: String,
    pub flag: Option<String>,
    pub wikidata_id: Option<String>,
    pub parent: Option<Issuer>,
    pub level: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssuersResponse {
    pub count: i64,
    pub issuers: Vec<IssuerDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MintDetail {
    pub id: i64,
    pub name: Option<String>,
    pub local_name: Option<String>,
    pub place: Option<String>,
    pub country: Option<Issuer>,
    pub start_year: Option<i64>,
    pub end_year: Option<i64>,
    pub nomisma_id: Option<String>,
    pub wikidata_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MintsResponse {
    pub count: i64,
    pub mints: Vec<MintDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CatalogueDetail {
    pub id: i64,
    pub code: String,
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub isbn13: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CataloguesResponse {
    pub count: i64,
    pub catalogues: Vec<CatalogueDetail>,
}

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
    pub mints: Option<Vec<MintDetail>>,
    pub printers: Option<Vec<Printer>>,
    pub series: Option<String>,
    pub commemorated_topic: Option<String>,
    pub comments: Option<String>,
    pub related_types: Option<Vec<RelatedType>>,
    pub tags: Vec<String>,
    pub references: Option<Vec<Reference>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Printer {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelatedType {
    pub id: i64,
    pub title: String,
    pub category: Category,
    pub issuer: Issuer,
    pub min_year: Option<i64>,
    pub max_year: Option<i64>,
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
    pub marks: Option<Vec<Mark>>,
    pub signatures: Option<Vec<Signature>>,
    pub references: Option<Vec<Reference>>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct Publication {
    pub id: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub title: String,
    pub translated_title: Option<String>,
    pub volume_number: Option<String>,
    pub subtitle: Option<String>,
    pub translated_subtitle: Option<String>,
    pub edition: Option<String>,
    pub languages: Vec<String>,
    pub year: Option<i64>,
    pub page_count: Option<i64>,
    pub pages: Option<String>,
    pub cover: Option<String>,
    pub isbn10: Option<String>,
    pub isbn13: Option<String>,
    pub issn: Option<String>,
    pub oclc_number: Option<String>,
    pub contributors: Option<Vec<Contributor>>,
    pub publishers: Option<Vec<Publisher>>,
    pub publication_places: Option<Vec<PublicationPlace>>,
    pub part_of: Option<Vec<PublicationPart>>,
    pub bibliographical_notice: Option<String>,
    pub homepage_url: Option<String>,
    pub download_urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Contributor {
    pub role: String,
    pub name: String,
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Publisher {
    pub name: String,
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicationPlace {
    pub name: String,
    pub geonames_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicationPart {
    #[serde(rename = "type")]
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub volume_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionsResponse {
    pub count: i64,
    pub collections: Vec<Collection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectedItem {
    pub id: i64,
    pub quantity: i64,
    #[serde(rename = "type")]
    pub type_info: CollectedItemType,
    pub issue: Option<Issue>,
    pub for_swap: bool,
    pub grade: Option<String>,
    pub private_comment: Option<String>,
    pub public_comment: Option<String>,
    pub price: Option<ItemPrice>,
    pub collection: Option<Collection>,
    pub pictures: Option<Vec<Picture>>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct CollectedItemType {
    pub id: i64,
    pub title: String,
    pub category: Category,
    pub issuer: Option<Issuer>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Picture {
    pub url: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GradingDetails {
    pub grading_company: Option<GradingCompany>,
    pub slab_grade: Option<SlabGrade>,
    pub slab_number: Option<String>,
    pub cac_sticker: Option<String>,
    pub grading_designations: Option<Vec<GradingDesignation>>,
    pub grading_strike: Option<GradingStrike>,
    pub grading_surface: Option<GradingSurface>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GradingCompany {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SlabGrade {
    pub id: i64,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GradingDesignation {
    pub id: i64,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GradingStrike {
    pub id: i64,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GradingSurface {
    pub id: i64,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectedItemsResponse {
    pub item_count: i64,
    pub item_for_swap_count: i64,
    pub item_type_count: i64,
    pub item_type_for_swap_count: i64,
    pub items: Vec<CollectedItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchByImageResponse {
    pub count: i64,
    pub types: Vec<SearchByImageTypeResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchByImageTypeResult {
    pub id: i64,
    pub title: String,
    pub category: Category,
    pub issuer: Issuer,
    pub min_year: Option<i64>,
    pub max_year: Option<i64>,
    pub obverse_thumbnail: Option<String>,
    pub reverse_thumbnail: Option<String>,
    pub similarity_distance: f64,
}