use serde::export::Formatter;
use url::ParseError;
use reqwest::Error;

#[derive(Debug)]
pub struct DockerRegistryError {
    kind: ErrorKind,
    message: String
}

#[derive(Debug)]
pub enum ErrorKind {
    ElementNotFound,
    ArgumentError,
    ProtocolError,
    HttpError
}

impl std::fmt::Display for DockerRegistryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for DockerRegistryError {}

impl From<ParseError> for DockerRegistryError {
    fn from(_: ParseError) -> Self {
        unimplemented!()
    }
}

impl From<reqwest::Error> for DockerRegistryError {
    fn from(_: Error) -> Self {
        unimplemented!()
    }
}