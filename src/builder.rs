use hyper::{Body, Request};
use std::collections::HashMap;
use url::Url;

pub(crate) struct Builder {
    pub query: HashMap<&'static str, String>,
    pub url: Url,
    pub req: Request<Body>,
}

impl Builder {
    #[inline]
    pub(crate) fn build(mut self) -> Request<Body> {
        self.url.query_pairs_mut().extend_pairs(self.query);
        *self.req.uri_mut() = self.url.to_string().parse().unwrap();
        self.req
    }
}
