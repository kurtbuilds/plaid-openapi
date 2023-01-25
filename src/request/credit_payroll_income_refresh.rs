use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditPayrollIncomeRefreshRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub user_token: Option<String>,
}
impl<'a> CreditPayrollIncomeRefreshRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreditPayrollIncomeRefreshResponse> {
        let mut r = self.http_client.client.post("/credit/payroll_income/refresh");
        if let Some(ref unwrapped) = self.user_token {
            r = r.json(json!({ "user_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreditPayrollIncomeRefreshRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditPayrollIncomeRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}