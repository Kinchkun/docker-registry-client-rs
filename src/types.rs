use crate::error::DockerRegistryError;
use serde::Deserialize;

pub type Result<T, E = DockerRegistryError> = std::result::Result<T, E>;
type RegResult<T> = Result<T, Vec<ResponseError>>;

#[derive(Deserialize)]
pub struct CatalogResponse {
    pub repositories: Vec<String>
}

#[derive(Deserialize)]
pub struct TagsListResponse {
    pub name: String,
    pub tags: Vec<String>
}

#[derive(Deserialize)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
    pub detail: Option<String>
}