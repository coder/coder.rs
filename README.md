# coder.rs

`coder.rs` is a pure Rust wrapper around the [Coder](https://coder.com) On-Prem
API.

## Installation

Coder.rs has been tested to work on Rust 1.40+

Add this to your `Cargo.toml`'s `[dependencies]` section:

```toml
coder = { version = "0.2", features = ["rustls"] }
```

## Usage

Coder provides the [`coder::Coder`](https://docs.rs/coder/latest/coder/client/struct.Coder.html)
struct for creating requests.

```rust
use std::env;
use std::error::Error;

use coder::client::{Coder, Executor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("MANAGER_URL").unwrap();
    let api_key = env::var("API_KEY").unwrap();
    let c = Coder::new(url, api_key);

    let res = c.get().me().execute().await?;
    dbg!(res);

    Ok(())
}

// [src/bin/main.rs:19] res = ApiResponse {
//     headers: Headers(
//         {
//             "server": "openresty/1.15.8.2",
//             "date": "Wed, 05 Aug 2020 05:05:11 GMT",
//             "content-type": "application/json",
//             "content-length": "653",
//             "vary": "Accept-Encoding",
//             "vary": "Origin",
//             "strict-transport-security": "max-age=15724800; includeSubDomains",
//             "coder-version": "1.9.0-rc1-220-gd2a04f83a",
//             "x-envoy-upstream-service-time": "20",
//         },
//     ),
//     status_code: 200,
//     response: Ok(
//         User {
//             id: "5e876cf4-10abe9b2e54eb609c5ec1870",
//             name: "Colin Adler",
//             username: "colin",
//             email: "colin@coder.com",
//             dotfiles_git_uri: "",
//             roles: [
//                 "site-manager",
//                 "site-auditor",
//             ],
//             avatar_hash: "28707dc83fdcba2cacaa3ad5e381b34b7cb37b74",
//             key_regenerated_at: 2020-04-03T17:05:56.964782Z,
//             created_at: 2020-04-03T17:05:56.964782Z,
//             updated_at: 2020-05-29T18:10:33.532351Z,
//         },
//     ),
// }
```

## Features

* `rustls` - Uses the [`rustls`](https://docs.rs/rustls/) pure Rust TLS implementation. (default)
* `rust-native-tls` - Uses [`native-tls`](https://docs.rs/native-tls/) for TLS which links against the OS default.