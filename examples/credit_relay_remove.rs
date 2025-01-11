#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let relay_token = "your relay token";
    let response = client.credit_relay_remove(relay_token).await.unwrap();
    println!("{:#?}", response);
}
