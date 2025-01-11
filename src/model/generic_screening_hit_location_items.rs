use serde::{Serialize, Deserialize};
use super::{MatchSummary, WatchlistScreeningHitLocations};
///Analyzed location information for the associated hit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericScreeningHitLocationItems {
    ///Summary object reflecting the match result of the associated data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    ///Location information for the associated individual watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<WatchlistScreeningHitLocations>,
}
impl std::fmt::Display for GenericScreeningHitLocationItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
