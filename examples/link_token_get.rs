#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let link_token = "your link token";
    let response = client.link_token_get(link_token).await.unwrap();
    println!("{:#?}", response);
}
