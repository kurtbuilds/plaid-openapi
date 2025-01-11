#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let gave_consent = true;
    let is_shareable = true;
    let template_id = "your template id";
    let response = client
        .identity_verification_create(gave_consent, is_shareable, template_id)
        .client_user_id("your client user id")
        .is_idempotent(true)
        .user(IdentityVerificationCreateRequestUser {
            address: Some(UserAddress {
                city: Some("your city".to_owned()),
                country: "your country".to_owned(),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
                street2: Some("your street 2".to_owned()),
            }),
            client_user_id: Some("your client user id".to_owned()),
            date_of_birth: Some(chrono::Utc::now().date_naive()),
            email_address: Some("your email address".to_owned()),
            id_number: Some(UserIdNumber {
                type_: IdNumberType::ArDni,
                value: "your value".to_owned(),
            }),
            name: Some(IdentityVerificationRequestUserName {
                family_name: "your family name".to_owned(),
                given_name: "your given name".to_owned(),
            }),
            phone_number: Some("your phone number".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
