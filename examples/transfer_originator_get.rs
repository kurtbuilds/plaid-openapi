#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let originator_client_id = "your originator client id";
    let response = client.transfer_originator_get(originator_client_id).await.unwrap();
    println!("{:#?}", response);
}
