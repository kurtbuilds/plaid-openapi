
use serde::{Serialize, Deserialize};
use super::FdxNotificationPayload;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdxNotificationsRequired {
    pub category: String,
    #[serde(rename = "notificationId")]
    pub notification_id: String,
    #[serde(rename = "notificationPayload")]
    pub notification_payload: FdxNotificationPayload,
    #[serde(rename = "sentOn")]
    pub sent_on: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for FdxNotificationsRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}