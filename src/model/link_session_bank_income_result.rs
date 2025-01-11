use serde::{Serialize, Deserialize};
use super::{CreditSessionBankIncomeStatus, LinkSessionSuccessMetadataInstitution};
///The details of a bank income verification in Link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSessionBankIncomeResult {
    ///An institution object. If the Item was created via Same-Day or Instant micro-deposit verification, will be `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkSessionSuccessMetadataInstitution>,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    pub item_id: String,
    /**Status of the Bank Income Link session.

`APPROVED`: User has approved and verified their income

`NO_DEPOSITS_FOUND`: We attempted, but were unable to find any income in the connected account.

`USER_REPORTED_NO_INCOME`: The user explicitly indicated that they don't receive income in the connected account.

`STARTED`: The user began the bank income portion of the link flow.

`INTERNAL_ERROR`: The user encountered an internal error.*/
    pub status: CreditSessionBankIncomeStatus,
}
impl std::fmt::Display for LinkSessionBankIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
