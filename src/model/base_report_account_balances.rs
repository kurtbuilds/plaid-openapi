use serde::{Serialize, Deserialize};
use super::BaseReportAverageMonthlyBalances;
///Base Report information about an account's balances
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAccountBalances {
    /**The amount of funds available to be withdrawn from the account, as determined by the financial institution.

For `credit`-type accounts, the `available` balance typically equals the `limit` less the `current` balance, less any pending outflows plus any pending inflows.

For `depository`-type accounts, the `available` balance typically equals the `current` balance less any pending outflows plus any pending inflows. For `depository`-type accounts, the `available` balance does not include the overdraft limit.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the `available` balance is the total cash available to withdraw as presented by the institution.

Note that not all institutions calculate the `available`  balance. In the event that `available` balance is unavailable, Plaid will return an `available` balance value of `null`.

Available balance may be cached and is not guaranteed to be up-to-date in realtime unless the value was returned by `/accounts/balance/get`.

If `current` is `null` this field is guaranteed not to be `null`.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<f64>,
    ///The average historical balance for the entire report
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_balance: Option<f64>,
    ///The average historical balance of each calendar month
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_monthly_balances: Option<Vec<BaseReportAverageMonthlyBalances>>,
    /**The total amount of funds in or owed by the account.

For `credit`-type accounts, a positive balance indicates the amount owed; a negative amount indicates the lender owing the account holder.

For `loan`-type accounts, the current balance is the principal remaining on the loan, except in the case of student loan accounts at Sallie Mae (`ins_116944`). For Sallie Mae student loans, the account's balance includes both principal and any outstanding interest.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the current balance is the total value of assets as presented by the institution.

Note that balance information may be cached unless the value was returned by `/accounts/balance/get`; if the Item is enabled for Transactions, the balance will be at least as recent as the most recent Transaction update. If you require realtime balance information, use the `available` balance as provided by `/accounts/balance/get`.

When returned by `/accounts/balance/get`, this field may be `null`. When this happens, `available` is guaranteed not to be `null`.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    ///The ISO-4217 currency code of the balance. Always null if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /**Timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.

This field is only used and expected when the institution is `ins_128026` (Capital One) and the Item contains one or more accounts with a non-depository account type, in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For Capital One depository accounts as well as all other account types on all other institutions, this field is ignored. See [account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full list of account types.

If the balance that is pulled is older than the given timestamp for Items with this field required, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_datetime: Option<chrono::DateTime<chrono::Utc>>,
    /**For `credit`-type accounts, this represents the credit limit.

For `depository`-type accounts, this represents the pre-arranged overdraft limit, which is common for current (checking) accounts in Europe.

In North America, this field is typically only available for `credit`-type accounts.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    ///The average historical balance from the most recent 30 days
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_thirty_day_average_balance: Option<f64>,
    /**The unofficial currency code associated with the balance. Always null if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `unofficial_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for BaseReportAccountBalances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
