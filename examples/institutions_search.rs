#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let country_codes = vec![CountryCode::Us];
    let query = "your query";
    let response = client
        .institutions_search(country_codes, query)
        .options(InstitutionsSearchRequestOptions {
            include_auth_metadata: Some(true),
            include_optional_metadata: Some(true),
            include_payment_initiation_metadata: Some(true),
            oauth: Some(true),
            payment_initiation: Some(InstitutionsSearchPaymentInitiationOptions {
                consent_id: Some("your consent id".to_owned()),
                payment_id: Some("your payment id".to_owned()),
            }),
        })
        .products(vec![Products::Assets])
        .await
        .unwrap();
    println!("{:#?}", response);
}
