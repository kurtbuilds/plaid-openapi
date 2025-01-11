#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .item_application_list()
        .access_token("your access token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
