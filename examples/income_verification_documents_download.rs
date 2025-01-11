#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .income_verification_documents_download()
        .access_token("your access token")
        .document_id("your document id")
        .income_verification_id("your income verification id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
