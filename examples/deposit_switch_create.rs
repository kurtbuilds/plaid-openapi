#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let target_access_token = "your target access token";
    let target_account_id = "your target account id";
    let response = client
        .deposit_switch_create(target_access_token, target_account_id)
        .country_code("your country code")
        .options(DepositSwitchCreateRequestOptions {
            transaction_item_access_tokens: Some(
                vec!["your transaction item access tokens".to_owned()],
            ),
            webhook: Some("your webhook".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
