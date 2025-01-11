#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let country_codes = vec![CountryCode::Us];
    let institution_id = "your institution id";
    let response = client
        .institutions_get_by_id(country_codes, institution_id)
        .options(InstitutionsGetByIdRequestOptions {
            include_auth_metadata: Some(true),
            include_optional_metadata: Some(true),
            include_payment_initiation_metadata: Some(true),
            include_status: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
