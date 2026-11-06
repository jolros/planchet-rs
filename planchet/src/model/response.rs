use serde::{Deserialize, Serialize};

use super::{
    CatalogueDetail, Collection, Grade, IssuerDetail, MintDetail, ObjectTypeDetail, SalesRecord,
    SearchByImageTypeResult, SearchTypeResult,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct IssuersResponse {
    pub count: i64,
    pub issuers: Vec<IssuerDetail>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MintsResponse {
    pub count: i64,
    pub mints: Vec<MintDetail>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ObjectTypesResponse {
    pub count: i64,
    pub object_types: Vec<ObjectTypeDetail>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CataloguesResponse {
    pub count: i64,
    pub catalogues: Vec<CatalogueDetail>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SearchTypesResponse {
    pub count: i64,
    pub types: Vec<SearchTypeResult>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SalesRecordsResponse {
    pub sales_count: i64,
    pub sales_records: Vec<SalesRecord>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CollectionsResponse {
    pub count: i64,
    pub collections: Vec<Collection>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SearchByImageResponse {
    pub count: i64,
    pub types: Vec<SearchByImageTypeResult>,
    pub experimental_tentative_year: Option<i64>,
    pub experimental_tentative_grade: Option<Grade>,
}
