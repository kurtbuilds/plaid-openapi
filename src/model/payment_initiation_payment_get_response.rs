
use serde::{Serialize, Deserialize};
use super::PaymentInitiationPayment;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetResponse {
    #[serde(flatten)]
    pub payment_initiation_payment: PaymentInitiationPayment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationPaymentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PaymentInitiationPaymentGetResponse {
    type Target = PaymentInitiationPayment;
    fn deref(&self) -> &Self::Target {
        &self.payment_initiation_payment
    }
}
impl std::ops::DerefMut for PaymentInitiationPaymentGetResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payment_initiation_payment
    }
}