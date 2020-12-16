use lazy_static::lazy_static;
use reqwest::{Url, IntoUrl};
use crate::types::Result;
use serde::de::DeserializeOwned;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = {
       reqwest::Client::default()
    };
}

pub struct DockerRegistryClient {
    url: Url
}

impl DockerRegistryClient {
    pub fn new<U: IntoUrl>(url: U) -> Result<Self> {
        Ok(Self { url: url.into_url()? })
    }

    async fn retrieve_digest(&self, repo_name: &str, tag: &str) -> Result<Option<String>> {
        let target = self.target(format!("/v2/{}/manifests/{}", repo_name, tag).as_str())?;
        let response = HTTP_CLIENT.head(target)
            .header("Accept","application/vnd.docker.distribution.manifest.v2+json")
            .send()
            .await?;
        let digest = response.headers().get("Docker-Content-Digest");
        match digest {
            None => Ok(None),
            Some(header_value) => Ok(Some(String::from(header_value.to_str()
                .expect("Could not convert header value to string."))))
        }
    }

    async fn http_get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let target = self.url.join(path)?;
        Ok(self.client.get(target)
            .send()
            .await?
            .json()
            .await?)
    }

    fn target<S: AsRef<str>>(&self, path: &str) -> Result<Url> {
        Ok(self.url.join(path.as_ref())?)
    }
}