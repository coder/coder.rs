use std::env;
use std::error::Error;

use coder::client::{Coder, Executor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("MANAGER_URL").unwrap();
    let api_key = env::var("API_KEY").unwrap();
    let c = Coder::new(url, api_key);

    // let res = c.get().me().execute().await?;
    // dbg!(res);

    // let res = c.get().orgs().execute().await?;
    // dbg!(res);

    // let res = c.get().org("default").members().execute().await?;
    // dbg!(res);

    let res = c
        .get()
        .org("default")
        .member("5f2b6fc0-6a67f0ce39bf4786337adb8f")
        .execute()
        .await?;
    dbg!(res);

    Ok(())
}
