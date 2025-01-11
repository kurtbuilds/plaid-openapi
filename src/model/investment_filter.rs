use serde::{Serialize, Deserialize};
use super::InvestmentAccountSubtypes;
///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: InvestmentAccountSubtypes,
}
impl std::fmt::Display for InvestmentFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
