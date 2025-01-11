#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let payment_profile_token = "your payment profile token";
    let response = client
        .sandbox_payment_profile_reset_login(payment_profile_token)
        .await
        .unwrap();
    println!("{:#?}", response);
}
