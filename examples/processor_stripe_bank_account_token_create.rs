#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let response = client
        .processor_stripe_bank_account_token_create(access_token, account_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
