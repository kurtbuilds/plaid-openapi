#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let transfer_intent_id = "your transfer intent id";
    let response = client.transfer_intent_get(transfer_intent_id).await.unwrap();
    println!("{:#?}", response);
}
