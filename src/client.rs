// TLS imports
#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

use std::error::Error;
use std::sync::Arc;

use http::uri::PathAndQuery;
use hyper::StatusCode;
use hyper::{self, Body};
use hyper::{Client, Request};

use crate::headers::Headers;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Coder {
    uri: &'static str,
    token: &'static str,
    pub(crate) client: Arc<Client<HttpsConnector>>,
}

const API_PREFIX: &'static str = "/api";

impl Coder {
    pub fn new(uri: String, token: String) -> Result<Self, Box<dyn Error>> {
        let uri = uri.parse::<hyper::Uri>()?;
        let mut parts = uri.into_parts();
        parts.path_and_query = Some(PathAndQuery::from_static(API_PREFIX));
        let uri = hyper::Uri::from_parts(parts).unwrap();

        Ok(Self {
            uri: Box::leak(uri.to_string().into_boxed_str()),
            token: Box::leak(token.into_boxed_str()),
            client: Arc::new(Client::builder().build(HttpsConnector::new())),
        })
    }
}

#[derive(Debug)]
pub struct ApiResponse<T: DeserializeOwned> {
    pub headers: Headers,
    pub status_code: StatusCode,
    pub response: Result<T, ApiError>,
}

#[derive(Deserialize, Debug)]
pub struct ApiError(pub serde_json::Value);

#[async_trait]
pub trait Executor {
    type T: DeserializeOwned;

    async fn execute(self) -> Result<ApiResponse<Self::T>, Box<dyn Error>>;
}

impl Coder {
    /// Returns a populated request for creating custom queries.
    pub fn new_request(&self) -> Result<Request<Body>, Box<dyn Error>> {
        Ok(Request::builder()
            .method(hyper::Method::GET)
            .uri(self.uri)
            .header("User-Agent", format!("coder.rs {}", VERSION))
            .header("Session-Token", self.token)
            .body(Body::empty())?)
    }
}

#[cfg(test)]
pub(crate) mod test {
    pub(crate) mod ids {
        pub const ENV_ID: &'static str = "5ed15061-d7d3db1d91600a4fed28f6ed";
        pub const IMAGE_ID: &'static str = "5ea8a569-596e6afd9301c23f8dabd87c";
        pub const IMAGE_TAG_ID: &'static str = "latest";
        pub const MEMBER_ID: &'static str = "5e876cf4-10abe9b2e54eb609c5ec1870";
        pub const ORG_ID: &'static str = "default";
        pub const REG_ID: &'static str = "5ea8a565-bdec42be59ffe9cf6b131e7c";
        pub const SERVICE_ID: &'static str = "5f15b3a2-57f7a823e4d379409978edbf";
        pub const USER_ID: &'static str = "5e876cf4-10abe9b2e54eb609c5ec1870";
    }

    use super::*;
    use std::env;

    pub(crate) fn client() -> Coder {
        let url = env::var("MANAGER_URL").expect("no MANAGER_URL env provided");
        let api_key = env::var("API_KEY").expect("no API_KEY env provided");
        Coder::new(url, api_key).unwrap()
    }
}
