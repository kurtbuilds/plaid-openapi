
use serde::{Serialize, Deserialize};
use super::TransferSweep;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLedgerDepositResponse {
    pub request_id: String,
    pub sweep: TransferSweep,
}
impl std::fmt::Display for TransferLedgerDepositResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}