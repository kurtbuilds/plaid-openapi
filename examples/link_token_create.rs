#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::link_token_create::LinkTokenCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_name = "your client name";
    let country_codes = vec![CountryCode::Us];
    let language = "your language";
    let user = LinkTokenCreateRequestUser {
        address: Some(serde_json::json!({})),
        client_user_id: "your client user id".to_owned(),
        date_of_birth: Some(chrono::Utc::now().date_naive()),
        email_address: Some("your email address".to_owned()),
        email_address_verified_time: Some(chrono::Utc::now()),
        id_number: Some(serde_json::json!({})),
        legal_name: Some("your legal name".to_owned()),
        name: Some(serde_json::json!({})),
        phone_number: Some("your phone number".to_owned()),
        phone_number_verified_time: Some(chrono::Utc::now()),
        ssn: Some("your ssn".to_owned()),
    };
    let response = client
        .link_token_create(LinkTokenCreateRequired {
            client_name,
            country_codes,
            language,
            user,
        })
        .access_token("your access token")
        .access_tokens(&["your access tokens"])
        .account_filters(LinkTokenAccountFilters {
            credit: Some(CreditFilter {
                account_subtypes: CreditAccountSubtypes(
                    vec![CreditAccountSubtype::CreditCard],
                ),
            }),
            depository: Some(DepositoryFilter {
                account_subtypes: DepositoryAccountSubtypes(
                    vec![DepositoryAccountSubtype::Checking],
                ),
            }),
            investment: Some(InvestmentFilter {
                account_subtypes: InvestmentAccountSubtypes(
                    vec![InvestmentAccountSubtype::InvestmentAccountSubtype529],
                ),
            }),
            loan: Some(LoanFilter {
                account_subtypes: LoanAccountSubtypes(vec![LoanAccountSubtype::Auto]),
            }),
            other: Some(OtherFilter {
                account_subtypes: OtherAccountSubtypes(vec![OtherAccountSubtype::Other]),
            }),
        })
        .additional_consented_products(vec![Products::Assets])
        .android_package_name("your android package name")
        .auth(LinkTokenCreateRequestAuth {
            auth_type_select_enabled: Some(true),
            automated_microdeposits_enabled: Some(true),
            database_insights_enabled: Some(true),
            database_match_enabled: Some(true),
            flow_type: Some("your flow type".to_owned()),
            instant_match_enabled: Some(true),
            instant_microdeposits_enabled: Some(true),
            reroute_to_credentials: Some("your reroute to credentials".to_owned()),
            same_day_microdeposits_enabled: Some(true),
            sms_microdeposits_verification_enabled: Some(true),
        })
        .base_report(LinkTokenCreateRequestBaseReport {
            client_report_id: Some("your client report id".to_owned()),
            days_requested: 1,
        })
        .card_switch(LinkTokenCreateCardSwitch {
            card_bin: "your card bin".to_owned(),
        })
        .consumer_report_permissible_purpose(serde_json::json!({}))
        .cra_enabled(true)
        .cra_options(LinkTokenCreateRequestCraOptions {
            base_report: Some(LinkTokenCreateRequestCraOptionsBaseReport {
                client_report_id: Some("your client report id".to_owned()),
            }),
            cashflow_insights: Some(LinkTokenCreateRequestCraOptionsCashflowInsights {
                attributes_version: Some(CashflowAttributesVersion::V10),
                plaid_check_score_version: Some(
                    "your plaid check score version".to_owned(),
                ),
            }),
            days_requested: 1,
            days_required: Some(1),
            partner_insights: Some(LinkTokenCreateRequestCraOptionsPartnerInsights {
                prism_products: Some(vec![PrismProduct::Insights]),
                prism_versions: Some(PrismVersions {
                    cashscore: Some(PrismCashScoreVersion(serde_json::json!({}))),
                    firstdetect: Some(PrismFirstDetectVersion(serde_json::json!({}))),
                    insights: Some(PrismInsightsVersion(serde_json::json!({}))),
                }),
            }),
        })
        .credit_partner_insights(LinkTokenCreateRequestCreditPartnerInsights {
            days_requested: Some(1),
            prism_products: Some(vec![PrismProduct::Insights]),
        })
        .deposit_switch(LinkTokenCreateRequestDepositSwitch {
            deposit_switch_id: "your deposit switch id".to_owned(),
        })
        .employment(LinkTokenCreateRequestEmployment {
            bank_employment: Some(LinkTokenCreateRequestEmploymentBankIncome {
                days_requested: 1,
            }),
            employment_source_types: Some(vec![EmploymentSourceType::Bank]),
        })
        .enable_multi_item_link(true)
        .eu_config(LinkTokenEuConfig {
            headless: Some(true),
        })
        .financekit_supported(true)
        .hosted_link(LinkTokenCreateHostedLink {
            completion_redirect_uri: Some("your completion redirect uri".to_owned()),
            delivery_method: Some(HostedLinkDeliveryMethod::Sms),
            is_mobile_app: Some(true),
            url_lifetime_seconds: Some(1),
        })
        .identity(LinkTokenCreateIdentity {
            account_ids: Some(vec!["your account ids".to_owned()]),
            is_document_upload: Some(true),
            parsing_configs: Some(vec![IncomeVerificationDocParsingConfig::Ocr]),
        })
        .identity_verification(LinkTokenCreateRequestIdentityVerification {
            consent: Some(true),
            gave_consent: Some(true),
            template_id: "your template id".to_owned(),
        })
        .income_verification(LinkTokenCreateRequestIncomeVerification {
            access_tokens: Some(vec!["your access tokens".to_owned()]),
            asset_report_id: Some("your asset report id".to_owned()),
            bank_income: Some(LinkTokenCreateRequestIncomeVerificationBankIncome {
                days_requested: 1,
                enable_multiple_items: Some(true),
            }),
            income_source_types: Some(vec![IncomeVerificationSourceType::Bank]),
            income_verification_id: Some("your income verification id".to_owned()),
            payroll_income: Some(LinkTokenCreateRequestIncomeVerificationPayrollIncome {
                flow_types: Some(
                    vec![IncomeVerificationPayrollFlowType::PayrollDigitalIncome],
                ),
                is_update_mode: Some(true),
                item_id_to_update: Some("your item id to update".to_owned()),
                parsing_config: Some(vec![IncomeVerificationDocParsingConfig::Ocr]),
            }),
            stated_income_sources: Some(
                vec![
                    LinkTokenCreateRequestUserStatedIncomeSource { category :
                    Some(UserStatedIncomeSourceCategory::Other), employer :
                    Some("your employer".to_owned()), pay_annual : Some(1.0),
                    pay_frequency : Some(UserStatedIncomeSourceFrequency::Unknown),
                    pay_per_cycle : Some(1.0), pay_type :
                    Some(UserStatedIncomeSourcePayType::Unknown) }
                ],
            ),
        })
        .institution_data(LinkTokenCreateInstitutionData {
            routing_number: Some("your routing number".to_owned()),
        })
        .institution_id("your institution id")
        .investments(LinkTokenInvestments {
            allow_manual_entry: Some(true),
            allow_unverified_crypto_wallets: Some(true),
        })
        .investments_auth(LinkTokenInvestmentsAuth {
            manual_entry_enabled: Some(true),
            masked_number_match_enabled: Some(true),
            stated_account_number_enabled: Some(true),
        })
        .link_customization_name("your link customization name")
        .optional_products(vec![Products::Assets])
        .payment_configuration(LinkTokenCreateRequestPaymentConfiguration {
            amount: "your amount".to_owned(),
            description: Some("your description".to_owned()),
        })
        .payment_initiation(LinkTokenCreateRequestPaymentInitiation {
            consent_id: Some("your consent id".to_owned()),
            payment_id: Some("your payment id".to_owned()),
        })
        .products(vec![Products::Assets])
        .redirect_uri("your redirect uri")
        .required_if_supported_products(vec![Products::Assets])
        .statements(LinkTokenCreateRequestStatements {
            end_date: chrono::Utc::now().date_naive(),
            start_date: chrono::Utc::now().date_naive(),
        })
        .transactions(LinkTokenTransactions {
            days_requested: Some(1),
        })
        .transfer(LinkTokenCreateRequestTransfer {
            authorization_id: Some("your authorization id".to_owned()),
            intent_id: Some("your intent id".to_owned()),
            payment_profile_token: Some("your payment profile token".to_owned()),
        })
        .update(LinkTokenCreateRequestUpdate {
            account_selection_enabled: Some(true),
            item_ids: Some(vec!["your item ids".to_owned()]),
            reauthorization_enabled: Some(true),
            user: Some(true),
        })
        .user_token("your user token")
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
