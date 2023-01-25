
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum MatchSummaryCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,
    #[serde(rename = "no_input")]
    NoInput,
}