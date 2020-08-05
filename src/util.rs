use hyper::http::uri::InvalidUriParts;
use hyper::Uri;

pub fn url_join(url: &Uri, paths: &[&str]) -> Result<Uri, InvalidUriParts> {
    let mut parts = url.clone().into_parts();
    let p = parts.path_and_query.take();
    let curr_path = match p {
        Some(ref p) => p.path(),
        None => "",
    };

    // Calculate the capacity we'll need and allocate it up front.
    let cap = paths.iter().map(|p| p.len()).sum::<usize>() + curr_path.len() + paths.len();
    let mut path = String::with_capacity(cap);
    path.push_str(curr_path);

    for p in paths.iter() {
        if !path.ends_with('/') {
            path.push('/');
        }

        path.push_str(p);
    }

    parts.path_and_query = path.parse().ok();
    Uri::from_parts(parts)
}
