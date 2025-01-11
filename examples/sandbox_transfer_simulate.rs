#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let event_type = "your event type";
    let transfer_id = "your transfer id";
    let response = client
        .sandbox_transfer_simulate(event_type, transfer_id)
        .failure_reason(TransferFailure {
            ach_return_code: Some("your ach return code".to_owned()),
            description: Some("your description".to_owned()),
        })
        .test_clock_id("your test clock id")
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
