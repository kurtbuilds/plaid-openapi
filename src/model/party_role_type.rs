
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PartyRoleType {
    Borrower,
}