use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_application_list`].

On request success, this will return a [`ItemApplicationListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationListRequest {
    pub access_token: Option<String>,
}
impl FluentRequest<'_, ItemApplicationListRequest> {
    ///Set the value of the access_token field.
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemApplicationListRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ItemApplicationListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/application/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_token {
                r = r.json(serde_json::json!({ "access_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    ///List a user’s connected applications
    pub fn item_application_list(
        &self,
    ) -> FluentRequest<'_, ItemApplicationListRequest> {
        FluentRequest {
            client: self,
            params: ItemApplicationListRequest {
                access_token: None,
            },
        }
    }
}
