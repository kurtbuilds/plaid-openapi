#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let transfer_id = "your transfer id";
    let response = client
        .transfer_cancel(transfer_id)
        .originator_client_id("your originator client id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
