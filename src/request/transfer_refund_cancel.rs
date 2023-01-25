use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRefundCancelRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub refund_id: String,
}
impl<'a> TransferRefundCancelRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransferRefundCancelResponse> {
        let mut r = self.http_client.client.post("/transfer/refund/cancel");
        r = r.json(json!({ "refund_id" : self.refund_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for TransferRefundCancelRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferRefundCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}