#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .auth_get(access_token)
        .options(AuthGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
