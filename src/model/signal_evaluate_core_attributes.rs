
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalEvaluateCoreAttributes {
    #[serde(rename = "address_change_count_28d")]
    pub address_change_count28_d: Option<i64>,
    #[serde(rename = "address_change_count_90d")]
    pub address_change_count90_d: Option<i64>,
    pub available_balance: Option<f64>,
    pub balance_last_updated: Option<String>,
    #[serde(rename = "credit_transactions_count_10d")]
    pub credit_transactions_count10_d: Option<i64>,
    #[serde(rename = "credit_transactions_count_30d")]
    pub credit_transactions_count30_d: Option<i64>,
    #[serde(rename = "credit_transactions_count_60d")]
    pub credit_transactions_count60_d: Option<i64>,
    #[serde(rename = "credit_transactions_count_90d")]
    pub credit_transactions_count90_d: Option<i64>,
    pub current_balance: Option<f64>,
    pub days_since_first_plaid_connection: Option<i64>,
    #[serde(rename = "days_with_negative_balance_count_90d")]
    pub days_with_negative_balance_count90_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_10d")]
    pub debit_transactions_count10_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_30d")]
    pub debit_transactions_count30_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_60d")]
    pub debit_transactions_count60_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_90d")]
    pub debit_transactions_count90_d: Option<i64>,
    #[serde(rename = "email_change_count_28d")]
    pub email_change_count28_d: Option<i64>,
    #[serde(rename = "email_change_count_90d")]
    pub email_change_count90_d: Option<i64>,
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_30d")]
    pub failed_plaid_non_oauth_authentication_attempts_count30_d: Option<i64>,
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_3d")]
    pub failed_plaid_non_oauth_authentication_attempts_count3_d: Option<i64>,
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_7d")]
    pub failed_plaid_non_oauth_authentication_attempts_count7_d: Option<i64>,
    pub is_savings_or_money_market_account: Option<bool>,
    #[serde(rename = "nsf_overdraft_transactions_count_30d")]
    pub nsf_overdraft_transactions_count30_d: Option<i64>,
    #[serde(rename = "nsf_overdraft_transactions_count_60d")]
    pub nsf_overdraft_transactions_count60_d: Option<i64>,
    #[serde(rename = "nsf_overdraft_transactions_count_7d")]
    pub nsf_overdraft_transactions_count7_d: Option<i64>,
    #[serde(rename = "nsf_overdraft_transactions_count_90d")]
    pub nsf_overdraft_transactions_count90_d: Option<i64>,
    #[serde(rename = "p10_eod_balance_30d")]
    pub p10_eod_balance30_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_31d_to_60d")]
    pub p10_eod_balance31_d_to60_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_60d")]
    pub p10_eod_balance60_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_61d_to_90d")]
    pub p10_eod_balance61_d_to90_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_90d")]
    pub p10_eod_balance90_d: Option<f64>,
    #[serde(rename = "p50_credit_transactions_amount_28d")]
    pub p50_credit_transactions_amount28_d: Option<f64>,
    #[serde(rename = "p50_debit_transactions_amount_28d")]
    pub p50_debit_transactions_amount28_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_30d")]
    pub p50_eod_balance30_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_31d_to_60d")]
    pub p50_eod_balance31_d_to60_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_60d")]
    pub p50_eod_balance60_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_61d_to_90d")]
    pub p50_eod_balance61_d_to90_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_90d")]
    pub p50_eod_balance90_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_30d")]
    pub p90_eod_balance30_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_31d_to_60d")]
    pub p90_eod_balance31_d_to60_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_60d")]
    pub p90_eod_balance60_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_61d_to_90d")]
    pub p90_eod_balance61_d_to90_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_90d")]
    pub p90_eod_balance90_d: Option<f64>,
    #[serde(rename = "p95_credit_transactions_amount_28d")]
    pub p95_credit_transactions_amount28_d: Option<f64>,
    #[serde(rename = "p95_debit_transactions_amount_28d")]
    pub p95_debit_transactions_amount28_d: Option<f64>,
    #[serde(rename = "phone_change_count_28d")]
    pub phone_change_count28_d: Option<i64>,
    #[serde(rename = "phone_change_count_90d")]
    pub phone_change_count90_d: Option<i64>,
    #[serde(rename = "plaid_connections_count_30d")]
    pub plaid_connections_count30_d: Option<i64>,
    #[serde(rename = "plaid_connections_count_7d")]
    pub plaid_connections_count7_d: Option<i64>,
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_30d")]
    pub plaid_non_oauth_authentication_attempts_count30_d: Option<i64>,
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_3d")]
    pub plaid_non_oauth_authentication_attempts_count3_d: Option<i64>,
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_7d")]
    pub plaid_non_oauth_authentication_attempts_count7_d: Option<i64>,
    #[serde(rename = "total_credit_transactions_amount_10d")]
    pub total_credit_transactions_amount10_d: Option<f64>,
    #[serde(rename = "total_credit_transactions_amount_30d")]
    pub total_credit_transactions_amount30_d: Option<f64>,
    #[serde(rename = "total_credit_transactions_amount_60d")]
    pub total_credit_transactions_amount60_d: Option<f64>,
    #[serde(rename = "total_credit_transactions_amount_90d")]
    pub total_credit_transactions_amount90_d: Option<f64>,
    #[serde(rename = "total_debit_transactions_amount_10d")]
    pub total_debit_transactions_amount10_d: Option<f64>,
    #[serde(rename = "total_debit_transactions_amount_30d")]
    pub total_debit_transactions_amount30_d: Option<f64>,
    #[serde(rename = "total_debit_transactions_amount_60d")]
    pub total_debit_transactions_amount60_d: Option<f64>,
    #[serde(rename = "total_debit_transactions_amount_90d")]
    pub total_debit_transactions_amount90_d: Option<f64>,
    pub total_plaid_connections_count: Option<i64>,
    pub transactions_last_updated: Option<String>,
    #[serde(rename = "unauthorized_transactions_count_30d")]
    pub unauthorized_transactions_count30_d: Option<i64>,
    #[serde(rename = "unauthorized_transactions_count_60d")]
    pub unauthorized_transactions_count60_d: Option<i64>,
    #[serde(rename = "unauthorized_transactions_count_7d")]
    pub unauthorized_transactions_count7_d: Option<i64>,
    #[serde(rename = "unauthorized_transactions_count_90d")]
    pub unauthorized_transactions_count90_d: Option<i64>,
}
impl std::fmt::Display for SignalEvaluateCoreAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}