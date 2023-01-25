#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_tokens = &["your access tokens"];
    let days_requested = 1;
    let response = client
        .asset_report_create(access_tokens, days_requested)
        .options(AssetReportCreateRequestOptions {
            client_report_id: Some("your client report id".to_owned()),
            include_fast_report: Some(true),
            products: Some(vec!["your products".to_owned()]),
            user: Some(AssetReportUser {
                client_user_id: Some("your client user id".to_owned()),
                email: Some("your email".to_owned()),
                first_name: Some("your first name".to_owned()),
                last_name: Some("your last name".to_owned()),
                middle_name: Some("your middle name".to_owned()),
                phone_number: Some("your phone number".to_owned()),
                ssn: Some("your ssn".to_owned()),
            }),
            webhook: Some("your webhook".to_owned()),
        })
        .report_type("your report type")
        .await
        .unwrap();
    println!("{:#?}", response);
}