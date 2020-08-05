// TLS imports
#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

use std::cell::RefCell;
use std::error::Error;
use std::sync::Arc;

use hyper::StatusCode;
use hyper::{self, Body};
use hyper::{Client, Request};

use crate::headers::Headers;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub struct Coder {
    url: &'static str,
    token: &'static str,
    client: Arc<Client<HttpsConnector>>,
}

impl Coder {
    pub fn new(url: String, token: String) -> Self {
        Self {
            url: Box::leak(url.into_boxed_str()),
            token: Box::leak(token.into_boxed_str()),
            client: Arc::new(Client::builder().build(HttpsConnector::new())),
        }
    }
}

#[derive(Debug)]
pub struct ApiResponse<T> {
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

const API_PREFIX: &'static str = "/api";

impl Coder {
    pub fn get(&self) -> GetQueryBuilder {
        GetQueryBuilder {
            request: Ok(RefCell::new(
                Request::get(format!("{}{}", self.url, API_PREFIX))
                    .header("Session-Token", self.token)
                    .body(Body::empty())
                    .unwrap(),
            )),
            client: Arc::clone(&self.client),
        }
    }
}

new_builder!(GetQuery);
