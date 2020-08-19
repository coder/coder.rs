use hyper::HeaderMap;

#[derive(Debug)]
pub struct Headers(HeaderMap);

impl From<HeaderMap> for Headers {
    fn from(h: HeaderMap) -> Headers {
        Headers(h)
    }
}

impl std::ops::Deref for Headers {
    type Target = HeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
