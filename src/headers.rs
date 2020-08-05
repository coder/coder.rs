use hyper::HeaderMap;

#[derive(Debug)]
pub struct Headers(HeaderMap);

impl From<HeaderMap> for Headers {
    fn from(h: HeaderMap) -> Headers {
        Headers(h)
    }
}
