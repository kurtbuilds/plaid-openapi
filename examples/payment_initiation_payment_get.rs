#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let payment_id = "your payment id";
    let response = client.payment_initiation_payment_get(payment_id).await.unwrap();
    println!("{:#?}", response);
}
