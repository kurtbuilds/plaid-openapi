
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetsErrorWebhook {
    pub asset_report_id: String,
    pub error: Option<PlaidError>,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for AssetsErrorWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}