use serde::{Serialize, Deserialize};
use super::WalletTransactionStatus;
///PaymentInitiationPaymentReverseResponse defines the response schema for `/payment_initation/payment/reverse`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseResponse {
    ///A unique ID identifying the refund
    pub refund_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**The status of the transaction.

`AUTHORISING`: The transaction is being processed for validation and compliance.

`INITIATED`: The transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed and is considered complete. This is only applicable for debit transactions.

`SETTLED`: The transaction has settled and funds are available for use. This is only applicable for credit transactions. A transaction will typically settle within seconds to several days, depending on which payment rail is used.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: WalletTransactionStatus,
}
impl std::fmt::Display for PaymentInitiationPaymentReverseResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
