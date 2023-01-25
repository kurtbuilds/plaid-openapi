
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCancelRequest {
    pub transfer_id: String,
}
impl std::fmt::Display for TransferCancelRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}