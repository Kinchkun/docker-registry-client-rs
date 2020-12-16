pub mod error;
pub mod types;

pub use types::Result;

mod docker_registry;

use crate::types::*;
use log::info;
use reqwest::header::HeaderValue;
use reqwest::{Client, IntoUrl, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub struct DockerRegistry {
    url: Url,
    client: Client,
}

impl DockerRegistry {
    pub fn new<U: IntoUrl>(url: U) -> Result<Self> {
        Ok(DockerRegistry {
            url: url.into_url()?,
            client: Default::default(),
        })
    }

    pub async fn list_repos(&self) -> Result<Vec<String>> {
        let response: CatalogResponse = self.http_get("/v2/_catalog").await?;
        Ok(response.repositories)
    }

    pub async fn list_tag_per_repo(&self, repo_name: &str) -> Result<Vec<String>> {
        let path = format!("/v2/{}/tags/list", repo_name);
        let response: TagsListResponse = self.http_get(path.as_str()).await?;
        Ok(response.tags)
    }

    pub async fn delete_tag(&self, repo_name: &str, tag: &str) -> Result<()> {
        let digest = self
            .retrieve_digest(repo_name, tag)
            .await?
            .expect("no such tag");
        let target = self
            .url
            .join(format!("/v2/{}/manifests/{}", repo_name, digest).as_str())?;
        self.client.delete(target).send().await?;
        Ok(())
    }

    async fn retrieve_digest(&self, repo_name: &str, tag: &str) -> Result<Option<String>> {
        let target = self
            .url
            .join(format!("/v2/{}/manifests/{}", repo_name, tag).as_str())?;
        let response = self
            .client
            .head(target)
            .header(
                "Accept",
                "application/vnd.docker.distribution.manifest.v2+json",
            )
            .send()
            .await?;
        let digest = response.headers().get("Docker-Content-Digest");
        match digest {
            None => Ok(None),
            Some(header_value) => Ok(Some(String::from(
                header_value
                    .to_str()
                    .expect("Could not convert header value to string."),
            ))),
        }
    }

    async fn http_get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let target = self.url.join(path)?;
        Ok(self.client.get(target).send().await?.json().await?)
    }
}
