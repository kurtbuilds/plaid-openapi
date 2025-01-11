#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let deposit_switch_id = "your deposit switch id";
    let response = client.deposit_switch_token_create(deposit_switch_id).await.unwrap();
    println!("{:#?}", response);
}
