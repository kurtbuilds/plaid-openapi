
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferEventListBankTransferType(pub serde_json::Value);