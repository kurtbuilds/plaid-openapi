use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Contains the state of a SMS same-day microdeposits verification session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedMmdVerificationWebhook {
    ///The external account ID of the affected account
    pub account_id: String,
    ///The Plaid environment the webhook was sent from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<WebhookEnvironmentValues>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///The final status of the same-day microdeposits verification. Will always be `MANUALLY_VERIFIED` or `VERIFICATION_FAILED`.
    pub status: String,
    ///`SMS_MICRODEPOSITS_VERIFICATION`
    pub webhook_code: String,
    ///`AUTH`
    pub webhook_type: String,
}
impl std::fmt::Display for HostedMmdVerificationWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
