
use serde::{Serialize, Deserialize};
use super::{SignalEvaluateCoreAttributes, SignalScores, SignalWarning};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalEvaluateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_attributes: Option<SignalEvaluateCoreAttributes>,
    pub request_id: String,
    pub scores: SignalScores,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<SignalWarning>>,
}
impl std::fmt::Display for ProcessorSignalEvaluateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}