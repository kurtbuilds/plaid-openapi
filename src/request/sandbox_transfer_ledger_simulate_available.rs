use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_ledger_simulate_available`].

On request success, this will return a [`SandboxTransferLedgerSimulateAvailableResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferLedgerSimulateAvailableRequest {
    pub ledger_id: Option<String>,
    pub test_clock_id: Option<String>,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, SandboxTransferLedgerSimulateAvailableRequest> {
    ///Set the value of the ledger_id field.
    pub fn ledger_id(mut self, ledger_id: &str) -> Self {
        self.params.ledger_id = Some(ledger_id.to_owned());
        self
    }
    ///Set the value of the test_clock_id field.
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferLedgerSimulateAvailableRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferLedgerSimulateAvailableResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/ledger/simulate_available";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.ledger_id {
                r = r.json(serde_json::json!({ "ledger_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(serde_json::json!({ "test_clock_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(serde_json::json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Simulate converting pending balance to available balance

Use the `/sandbox/transfer/ledger/simulate_available` endpoint to simulate converting pending balance to available balance for all originators in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferledgersimulate_available>.*/
    pub fn sandbox_transfer_ledger_simulate_available(
        &self,
    ) -> FluentRequest<'_, SandboxTransferLedgerSimulateAvailableRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferLedgerSimulateAvailableRequest {
                ledger_id: None,
                test_clock_id: None,
                webhook: None,
            },
        }
    }
}
