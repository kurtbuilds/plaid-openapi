#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let asset_report_token = "your asset report token";
    let response = client.asset_report_remove(asset_report_token).await.unwrap();
    println!("{:#?}", response);
}
