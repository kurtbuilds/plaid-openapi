#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let processor = "your processor";
    let response = client
        .processor_token_create(access_token, account_id, processor)
        .await
        .unwrap();
    println!("{:#?}", response);
}
