use serde::{Serialize, Deserialize};
use super::TransferBalance;
///Defines the response schema for `/transfer/balance/get`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceGetResponse {
    ///Information about the balance held with Plaid.
    pub balance: TransferBalance,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
