use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Fired when a recurring transfer is cancelled by Plaid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringCancelledWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///Plaid’s unique identifier for a recurring transfer.
    pub recurring_transfer_id: String,
    ///`RECURRING_CANCELLED`
    pub webhook_code: String,
    ///`TRANSFER`
    pub webhook_type: String,
}
impl std::fmt::Display for RecurringCancelledWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
