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
