#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let end_customer_client_id = "your end customer client id";
    let response = client
        .partner_customer_remove(end_customer_client_id)
        .client_id("your client id")
        .secret("your secret")
        .await
        .unwrap();
    println!("{:#?}", response);
}
