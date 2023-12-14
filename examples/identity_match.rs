#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .identity_match(access_token)
        .options(IdentityMatchRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .user(IdentityMatchUser {
            address: Some(AddressDataNotRequired {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            legal_name: Some("your legal name".to_owned()),
            phone_number: Some("your phone number".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}