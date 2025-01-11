#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .credit_payroll_income_risk_signals_get()
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
