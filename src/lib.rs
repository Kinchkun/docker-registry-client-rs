use reqwest::{IntoUrl, Url, Client};
use serde::Deserialize;

pub type Result<T, E = std::boxed::Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub struct DockerRegistry {
    url: Url,
    client: Client
}


#[derive(Deserialize)]
struct CatalogResponse {
    repositories: Vec<String>
}

impl DockerRegistry {
    pub fn new<U: IntoUrl>(url: U) -> Result<Self> {
        Ok(DockerRegistry { url: url.into_url()?, client: Default::default() })
    }

    pub async fn list_repos(&self) -> Result<Vec<String>> {
        let target = self.url.join("/v2/_catalog")?;
        let response: CatalogResponse = self.client.get(target)
            .send()
            .await?
            .json()
            .await?;
        Ok(response.repositories)
    }
}

