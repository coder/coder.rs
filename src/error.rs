use serde::Deserialize;
use thiserror::Error as ErrorImpl;

#[derive(ErrorImpl, Debug)]
pub enum Error {
    #[error("hyper error: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("hyper http error: {0}")]
    HyperHttp(#[from] hyper::http::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("url parse error: {0}")]
    UrlParse(#[from] url::ParseError),
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: ApiErrorInner,
}

#[derive(Deserialize, Debug)]
pub struct ApiErrorInner {
    pub msg: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub details: Box<serde_json::value::RawValue>,
}

#[cfg(test)]
mod test {
    use crate::client::test::client;
    use crate::client::Executor;

    mod image {
        use super::*;

        #[tokio::test]
        async fn test_api_error() {
            let c = client();

            let res = c
                .users()
                .get("some random id")
                .execute()
                .await
                .expect("send request")
                .response;

            assert!(res.is_err());
            dbg!(res);
        }
    }
}
