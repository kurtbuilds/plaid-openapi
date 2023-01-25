
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndividualScreeningHitNames {
    pub full: String,
    pub is_primary: bool,
    pub weak_alias_determination: String,
}
impl std::fmt::Display for IndividualScreeningHitNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}