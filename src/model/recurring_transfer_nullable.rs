use serde::{Serialize, Deserialize};
///Represents a recurring transfer within the Transfers API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringTransferNullable {}
impl std::fmt::Display for RecurringTransferNullable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}