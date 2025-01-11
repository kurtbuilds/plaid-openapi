use serde::{Serialize, Deserialize};
/**Value is one of the following:
`buy`: Buying an investment
`sell`: Selling an investment
`cancel`: A cancellation of a pending transaction
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity which modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer

For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InvestmentTransactionType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "transfer")]
    Transfer,
}
