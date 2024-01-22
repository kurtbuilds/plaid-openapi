use serde::{Serialize, Deserialize};
/**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy CSV file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories. If you are migrating to personal finance categories from the legacy categories, also refer to the [`migration guide`](https://plaid.com/docs/transactions/pfc-migration/).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalFinanceCategory {
    /**A description of how confident we are that the provided categories accurately describe the transaction intent.

`VERY_HIGH`: We are more than 98% confident that this category reflects the intent of the transaction.
`HIGH`: We are more than 90% confident that this category reflects the intent of the transaction.
`MEDIUM`: We are moderately confident that this category reflects the intent of the transaction.
`LOW`: This category may reflect the intent, but there may be other categories that are more accurate.
`UNKNOWN`: We don’t know the confidence level for this category.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<String>,
    ///A granular category conveying the transaction's intent. This field can also be used as a unique identifier for the category.
    pub detailed: String,
    ///A high level category that communicates the broad category of the transaction.
    pub primary: String,
}
impl std::fmt::Display for PersonalFinanceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}