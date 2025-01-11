#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .payment_initiation_payment_list()
        .consent_id("your consent id")
        .count(1)
        .cursor(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}
