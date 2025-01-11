#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_transaction_id = "your client transaction id";
    let return_code = "your return code";
    let response = client
        .signal_return_report(client_transaction_id, return_code)
        .returned_at(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}
