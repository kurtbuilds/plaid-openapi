use serde::{Serialize, Deserialize};
use super::{
    CreditBankIncomeCause, CreditBankIncomeWarningCode, CreditBankIncomeWarningType,
};
///The warning associated with the data that was unavailable for the Bank Income Report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeWarning {
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<CreditBankIncomeCause>,
    /**The warning code identifies a specific kind of warning.
`IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item
`TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item
`ITEM_UNAPPROVED`: User exited flow before giving permission to share data for the Item
`REPORT_DELETED`: Report deleted due to customer or consumer request
`DATA_UNAVAILABLE`: No relevant data was found for the Item*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_code: Option<CreditBankIncomeWarningCode>,
    ///The warning type which will always be `BANK_INCOME_WARNING`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_type: Option<CreditBankIncomeWarningType>,
}
impl std::fmt::Display for CreditBankIncomeWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
