#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .transactions_sync(access_token)
        .count(1)
        .cursor("your cursor")
        .options(TransactionsSyncRequestOptions {
            days_requested: Some(1),
            include_logo_and_counterparty_beta: Some(true),
            include_original_description: Some(true),
            include_personal_finance_category: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
