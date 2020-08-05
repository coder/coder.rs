use std::env;
use std::error::Error;

use coder::client::Coder;
use coder::client::Executor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("MANAGER_URL").unwrap();
    let api_key = env::var("API_KEY").unwrap();
    let c = Coder::new(url, api_key);

    // let res = c.get().me().execute().await?;
    let res = c
        .get()
        .user("5e876cf4-10abe9b2e54eb609c5ec1870")
        .execute()
        .await?;
    dbg!(res);
    let res = c.get().users().execute().await?;
    dbg!(res);

    Ok(())
}
