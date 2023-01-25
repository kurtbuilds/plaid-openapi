
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
}
impl std::fmt::Display for DocumentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}