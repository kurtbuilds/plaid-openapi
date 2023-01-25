
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentConstraints {
    pub max_payment_amount: PaymentConsentMaxPaymentAmount,
    pub periodic_amounts: Vec<PaymentConsentPeriodicAmount>,
    pub valid_date_time: Option<PaymentConsentValidDateTime>,
}
impl std::fmt::Display for PaymentInitiationConsentConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}