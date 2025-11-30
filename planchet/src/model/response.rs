use serde::Deserialize;

use super::{
    CatalogueDetail, Collection, Grade, IssuerDetail, MintDetail, SearchByImageTypeResult,
    SearchTypeResult,
};

#[derive(Debug, Clone, Deserialize)]
pub struct IssuersResponse {
    pub count: i64,
    pub issuers: Vec<IssuerDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MintsResponse {
    pub count: i64,
    pub mints: Vec<MintDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CataloguesResponse {
    pub count: i64,
    pub catalogues: Vec<CatalogueDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchTypesResponse {
    pub count: i64,
    pub types: Vec<SearchTypeResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionsResponse {
    pub count: i64,
    pub collections: Vec<Collection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchByImageResponse {
    pub count: i64,
    pub types: Vec<SearchByImageTypeResult>,
    pub experimental_tentative_year: Option<i64>,
    pub experimental_tentative_grade: Option<Grade>,
}
