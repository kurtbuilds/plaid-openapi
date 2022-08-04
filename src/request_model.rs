use serde::{Serialize, Deserialize};
use crate::model;
pub struct ItemApplicationListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: Option<String>,
}
impl<'a> ItemApplicationListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemApplicationListResponse> {
        let mut r = self.client.post("/item/application/list");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemApplicationScopesUpdateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub application_id: String,
    pub scopes: serde_json::Value,
    pub state: String,
    pub context: String,
}
impl<'a> ItemApplicationScopesUpdateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::ItemApplicationScopesUpdateResponse> {
        let mut r = self.client.post("/item/application/scopes/update");
        let res = r
            .json(
                json!(
                    { "access_token" : access_token, "application_id" : application_id,
                    "scopes" : scopes, "state" : state, "context" : context }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ApplicationGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub application_id: String,
}
impl<'a> ApplicationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ApplicationGetResponse> {
        let mut r = self.client.post("/application/get");
        let res = r
            .json(json!({ "application_id" : application_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
}
impl<'a> ItemGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemGetResponse> {
        let mut r = self.client.post("/item/get");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AuthGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> AuthGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AuthGetResponse> {
        let mut r = self.client.post("/auth/get");
        let res = r
            .json(json!({ "access_token" : access_token, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub options: serde_json::Value,
    pub access_token: String,
    pub start_date: String,
    pub end_date: String,
}
impl<'a> TransactionsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsGetResponse> {
        let mut r = self.client.post("/transactions/get");
        let res = r
            .json(
                json!(
                    { "options" : options, "access_token" : access_token, "start_date" :
                    start_date, "end_date" : end_date }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsRefreshRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
}
impl<'a> TransactionsRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsRefreshResponse> {
        let mut r = self.client.post("/transactions/refresh");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsRecurringGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub account_ids: Vec<String>,
}
impl<'a> TransactionsRecurringGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsRecurringGetResponse> {
        let mut r = self.client.post("/transactions/recurring/get");
        let res = r
            .json(json!({ "access_token" : access_token, "account_ids" : account_ids }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsSyncRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub cursor: String,
    pub count: i64,
}
impl<'a> TransactionsSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsSyncResponse> {
        let mut r = self.client.post("/transactions/sync");
        let res = r
            .json(
                json!(
                    { "access_token" : access_token, "cursor" : cursor, "count" : count }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InstitutionsGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub count: i64,
    pub offset: i64,
    pub country_codes: Vec<CountryCode>,
    pub options: serde_json::Value,
}
impl<'a> InstitutionsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InstitutionsGetResponse> {
        let mut r = self.client.post("/institutions/get");
        let res = r
            .json(
                json!(
                    { "count" : count, "offset" : offset, "country_codes" :
                    country_codes, "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InstitutionsSearchRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub query: String,
    pub products: Option<Vec<Products>>,
    pub country_codes: Vec<CountryCode>,
    pub options: serde_json::Value,
}
impl<'a> InstitutionsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InstitutionsSearchResponse> {
        let mut r = self.client.post("/institutions/search");
        let res = r
            .json(
                json!(
                    { "query" : query, "products" : products, "country_codes" :
                    country_codes, "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InstitutionsGetByIdRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub institution_id: String,
    pub country_codes: Vec<CountryCode>,
    pub options: serde_json::Value,
}
impl<'a> InstitutionsGetByIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InstitutionsGetByIdResponse> {
        let mut r = self.client.post("/institutions/get_by_id");
        let res = r
            .json(
                json!(
                    { "institution_id" : institution_id, "country_codes" : country_codes,
                    "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemRemoveRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
}
impl<'a> ItemRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemRemoveResponse> {
        let mut r = self.client.post("/item/remove");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AccountsGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> AccountsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AccountsGetResponse> {
        let mut r = self.client.post("/accounts/get");
        let res = r
            .json(json!({ "access_token" : access_token, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CategoriesGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
}
impl<'a> CategoriesGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CategoriesGetResponse> {
        let mut r = self.client.post("/categories/get");
        let res = r
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub institution_id: String,
    pub options: serde_json::Value,
}
impl<'a> SandboxProcessorTokenCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxProcessorTokenCreateResponse> {
        let mut r = self.client.post("/sandbox/processor_token/create");
        let res = r
            .json(json!({ "institution_id" : institution_id, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxPublicTokenCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub institution_id: String,
    pub initial_products: Vec<Products>,
    pub options: serde_json::Value,
}
impl<'a> SandboxPublicTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxPublicTokenCreateResponse> {
        let mut r = self.client.post("/sandbox/public_token/create");
        let res = r
            .json(
                json!(
                    { "institution_id" : institution_id, "initial_products" :
                    initial_products, "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxItemFireWebhookRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub webhook_code: String,
}
impl<'a> SandboxItemFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxItemFireWebhookResponse> {
        let mut r = self.client.post("/sandbox/item/fire_webhook");
        let res = r
            .json(
                json!({ "access_token" : access_token, "webhook_code" : webhook_code }),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AccountsBalanceGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> AccountsBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AccountsGetResponse> {
        let mut r = self.client.post("/accounts/balance/get");
        let res = r
            .json(json!({ "access_token" : access_token, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IdentityGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> IdentityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IdentityGetResponse> {
        let mut r = self.client.post("/identity/get");
        let res = r
            .json(json!({ "access_token" : access_token, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorAuthGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub processor_token: String,
}
impl<'a> ProcessorAuthGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorAuthGetResponse> {
        let mut r = self.client.post("/processor/auth/get");
        let res = r
            .json(json!({ "processor_token" : processor_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorBankTransferCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub idempotency_key: String,
    pub processor_token: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub iso_currency_code: String,
    pub description: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub custom_tag: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub origination_account_id: Option<String>,
}
impl<'a> ProcessorBankTransferCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::ProcessorBankTransferCreateResponse> {
        let mut r = self.client.post("/processor/bank_transfer/create");
        let res = r
            .json(
                json!(
                    { "idempotency_key" : idempotency_key, "processor_token" :
                    processor_token, "type" : type_, "network" : network, "amount" :
                    amount, "iso_currency_code" : iso_currency_code, "description" :
                    description, "ach_class" : ach_class, "user" : user, "custom_tag" :
                    custom_tag, "metadata" : metadata, "origination_account_id" :
                    origination_account_id }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorIdentityGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub processor_token: String,
}
impl<'a> ProcessorIdentityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorIdentityGetResponse> {
        let mut r = self.client.post("/processor/identity/get");
        let res = r
            .json(json!({ "processor_token" : processor_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorBalanceGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub processor_token: String,
    pub options: serde_json::Value,
}
impl<'a> ProcessorBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorBalanceGetResponse> {
        let mut r = self.client.post("/processor/balance/get");
        let res = r
            .json(json!({ "processor_token" : processor_token, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemWebhookUpdateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub webhook: Option<String>,
}
impl<'a> ItemWebhookUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemWebhookUpdateResponse> {
        let mut r = self.client.post("/item/webhook/update");
        let res = r
            .json(json!({ "access_token" : access_token, "webhook" : webhook }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemAccessTokenInvalidateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
}
impl<'a> ItemAccessTokenInvalidateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemAccessTokenInvalidateResponse> {
        let mut r = self.client.post("/item/access_token/invalidate");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WebhookVerificationKeyGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub key_id: String,
}
impl<'a> WebhookVerificationKeyGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WebhookVerificationKeyGetResponse> {
        let mut r = self.client.post("/webhook_verification_key/get");
        let res = r
            .json(json!({ "key_id" : key_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct LiabilitiesGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> LiabilitiesGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::LiabilitiesGetResponse> {
        let mut r = self.client.post("/liabilities/get");
        let res = r
            .json(json!({ "access_token" : access_token, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationRecipientCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub name: String,
    pub iban: Option<String>,
    pub bacs: Option<serde_json::Value>,
    pub address: Option<serde_json::Value>,
}
impl<'a> PaymentInitiationRecipientCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationRecipientCreateResponse> {
        let mut r = self.client.post("/payment_initiation/recipient/create");
        let res = r
            .json(
                json!(
                    { "name" : name, "iban" : iban, "bacs" : bacs, "address" : address }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentReverseRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub payment_id: String,
}
impl<'a> PaymentInitiationPaymentReverseRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentReverseResponse> {
        let mut r = self.client.post("/payment_initiation/payment/reverse");
        let res = r
            .json(json!({ "payment_id" : payment_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationRecipientGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub recipient_id: String,
}
impl<'a> PaymentInitiationRecipientGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationRecipientGetResponse> {
        let mut r = self.client.post("/payment_initiation/recipient/get");
        let res = r
            .json(json!({ "recipient_id" : recipient_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationRecipientListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
}
impl<'a> PaymentInitiationRecipientListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationRecipientListResponse> {
        let mut r = self.client.post("/payment_initiation/recipient/list");
        let res = r
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub recipient_id: String,
    pub reference: String,
    pub amount: serde_json::Value,
    pub schedule: serde_json::Value,
    pub options: Option<serde_json::Value>,
}
impl<'a> PaymentInitiationPaymentCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentCreateResponse> {
        let mut r = self.client.post("/payment_initiation/payment/create");
        let res = r
            .json(
                json!(
                    { "recipient_id" : recipient_id, "reference" : reference, "amount" :
                    amount, "schedule" : schedule, "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreatePaymentTokenRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub payment_id: String,
}
impl<'a> CreatePaymentTokenRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentTokenCreateResponse> {
        let mut r = self.client.post("/payment_initiation/payment/token/create");
        let res = r
            .json(json!({ "payment_id" : payment_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxItemResetLoginRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
}
impl<'a> SandboxItemResetLoginRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxItemResetLoginResponse> {
        let mut r = self.client.post("/sandbox/item/reset_login");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxItemSetVerificationStatusRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub account_id: String,
    pub verification_status: String,
}
impl<'a> SandboxItemSetVerificationStatusRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxItemSetVerificationStatusResponse> {
        let mut r = self.client.post("/sandbox/item/set_verification_status");
        let res = r
            .json(
                json!(
                    { "access_token" : access_token, "account_id" : account_id,
                    "verification_status" : verification_status }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemPublicTokenExchangeRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub public_token: String,
}
impl<'a> ItemPublicTokenExchangeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemPublicTokenExchangeResponse> {
        let mut r = self.client.post("/item/public_token/exchange");
        let res = r
            .json(json!({ "public_token" : public_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemCreatePublicTokenRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
}
impl<'a> ItemCreatePublicTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemPublicTokenCreateResponse> {
        let mut r = self.client.post("/item/public_token/create");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub payment_id: String,
}
impl<'a> PaymentInitiationPaymentGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentGetResponse> {
        let mut r = self.client.post("/payment_initiation/payment/get");
        let res = r
            .json(json!({ "payment_id" : payment_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}
impl<'a> PaymentInitiationPaymentListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentListResponse> {
        let mut r = self.client.post("/payment_initiation/payment/list");
        let res = r
            .json(json!({ "count" : count, "cursor" : cursor }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_tokens: Vec<AccessToken>,
    pub days_requested: i64,
    pub options: serde_json::Value,
}
impl<'a> AssetReportCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportCreateResponse> {
        let mut r = self.client.post("/asset_report/create");
        let res = r
            .json(
                json!(
                    { "access_tokens" : access_tokens, "days_requested" : days_requested,
                    "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRefreshRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub asset_report_token: String,
    pub days_requested: Option<i64>,
    pub options: serde_json::Value,
}
impl<'a> AssetReportRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportRefreshResponse> {
        let mut r = self.client.post("/asset_report/refresh");
        let res = r
            .json(
                json!(
                    { "asset_report_token" : asset_report_token, "days_requested" :
                    days_requested, "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRemoveRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub asset_report_token: String,
}
impl<'a> AssetReportRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportRemoveResponse> {
        let mut r = self.client.post("/asset_report/remove");
        let res = r
            .json(json!({ "asset_report_token" : asset_report_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportFilterRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub asset_report_token: String,
    pub account_ids_to_exclude: Vec<String>,
}
impl<'a> AssetReportFilterRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportFilterResponse> {
        let mut r = self.client.post("/asset_report/filter");
        let res = r
            .json(
                json!(
                    { "asset_report_token" : asset_report_token, "account_ids_to_exclude"
                    : account_ids_to_exclude }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub asset_report_token: String,
    pub include_insights: bool,
}
impl<'a> AssetReportGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportGetResponse> {
        let mut r = self.client.post("/asset_report/get");
        let res = r
            .json(
                json!(
                    { "asset_report_token" : asset_report_token, "include_insights" :
                    include_insights }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportAuditCopyCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub asset_report_token: String,
    pub auditor_id: String,
}
impl<'a> AssetReportAuditCopyCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::AssetReportAuditCopyCreateResponse> {
        let mut r = self.client.post("/asset_report/audit_copy/create");
        let res = r
            .json(
                json!(
                    { "asset_report_token" : asset_report_token, "auditor_id" :
                    auditor_id }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportAuditCopyRemoveRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyRemoveRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::AssetReportAuditCopyRemoveResponse> {
        let mut r = self.client.post("/asset_report/audit_copy/remove");
        let res = r
            .json(json!({ "audit_copy_token" : audit_copy_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InvestmentsHoldingsGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> InvestmentsHoldingsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvestmentsHoldingsGetResponse> {
        let mut r = self.client.post("/investments/holdings/get");
        let res = r
            .json(json!({ "access_token" : access_token, "options" : options }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InvestmentsTransactionsGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub start_date: String,
    pub end_date: String,
    pub options: serde_json::Value,
}
impl<'a> InvestmentsTransactionsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::InvestmentsTransactionsGetResponse> {
        let mut r = self.client.post("/investments/transactions/get");
        let res = r
            .json(
                json!(
                    { "access_token" : access_token, "start_date" : start_date,
                    "end_date" : end_date, "options" : options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub account_id: String,
    pub processor: String,
}
impl<'a> ProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorTokenCreateResponse> {
        let mut r = self.client.post("/processor/token/create");
        let res = r
            .json(
                json!(
                    { "access_token" : access_token, "account_id" : account_id,
                    "processor" : processor }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub account_id: String,
}
impl<'a> ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::ProcessorStripeBankAccountTokenCreateResponse> {
        let mut r = self.client.post("/processor/stripe/bank_account_token/create");
        let res = r
            .json(json!({ "access_token" : access_token, "account_id" : account_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorApexProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub account_id: String,
}
impl<'a> ProcessorApexProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorTokenCreateResponse> {
        let mut r = self.client.post("/processor/apex/processor_token/create");
        let res = r
            .json(json!({ "access_token" : access_token, "account_id" : account_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub target_access_token: String,
    pub target_account_id: String,
    pub country_code: Option<String>,
    pub options: serde_json::Value,
}
impl<'a> DepositSwitchCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchCreateResponse> {
        let mut r = self.client.post("/deposit_switch/create");
        let res = r
            .json(
                json!(
                    { "target_access_token" : target_access_token, "target_account_id" :
                    target_account_id, "country_code" : country_code, "options" : options
                    }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemImportRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub products: Vec<Products>,
    pub user_auth: serde_json::Value,
    pub options: serde_json::Value,
}
impl<'a> ItemImportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemImportResponse> {
        let mut r = self.client.post("/item/import");
        let res = r
            .json(
                json!(
                    { "products" : products, "user_auth" : user_auth, "options" : options
                    }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchTokenCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchTokenCreateResponse> {
        let mut r = self.client.post("/deposit_switch/token/create");
        let res = r
            .json(json!({ "deposit_switch_id" : deposit_switch_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct LinkTokenCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub client_name: String,
    pub language: String,
    pub country_codes: Vec<CountryCode>,
    pub user: serde_json::Value,
    pub products: Vec<Products>,
    pub webhook: String,
    pub access_token: String,
    pub link_customization_name: String,
    pub redirect_uri: String,
    pub android_package_name: String,
    pub account_filters: serde_json::Value,
    pub eu_config: serde_json::Value,
    pub institution_id: String,
    pub payment_initiation: serde_json::Value,
    pub deposit_switch: serde_json::Value,
    pub income_verification: serde_json::Value,
    pub auth: serde_json::Value,
    pub transfer: serde_json::Value,
    pub update: serde_json::Value,
}
impl<'a> LinkTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::LinkTokenCreateResponse> {
        let mut r = self.client.post("/link/token/create");
        let res = r
            .json(
                json!(
                    { "client_name" : client_name, "language" : language, "country_codes"
                    : country_codes, "user" : user, "products" : products, "webhook" :
                    webhook, "access_token" : access_token, "link_customization_name" :
                    link_customization_name, "redirect_uri" : redirect_uri,
                    "android_package_name" : android_package_name, "account_filters" :
                    account_filters, "eu_config" : eu_config, "institution_id" :
                    institution_id, "payment_initiation" : payment_initiation,
                    "deposit_switch" : deposit_switch, "income_verification" :
                    income_verification, "auth" : auth, "transfer" : transfer, "update" :
                    update }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct LinkTokenGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub link_token: String,
}
impl<'a> LinkTokenGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::LinkTokenGetResponse> {
        let mut r = self.client.post("/link/token/get");
        let res = r
            .json(json!({ "link_token" : link_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportAuditCopyGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportGetResponse> {
        let mut r = self.client.post("/asset_report/audit_copy/get");
        let res = r
            .json(json!({ "audit_copy_token" : audit_copy_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchGetResponse> {
        let mut r = self.client.post("/deposit_switch/get");
        let res = r
            .json(json!({ "deposit_switch_id" : deposit_switch_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub transfer_id: String,
}
impl<'a> TransferGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferGetResponse> {
        let mut r = self.client.post("/transfer/get");
        let res = r
            .json(json!({ "transfer_id" : transfer_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferGetResponse> {
        let mut r = self.client.post("/bank_transfer/get");
        let res = r
            .json(json!({ "bank_transfer_id" : bank_transfer_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferAuthorizationCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub account_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub device: serde_json::Value,
    pub origination_account_id: String,
    pub iso_currency_code: String,
}
impl<'a> TransferAuthorizationCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::TransferAuthorizationCreateResponse> {
        let mut r = self.client.post("/transfer/authorization/create");
        let res = r
            .json(
                json!(
                    { "access_token" : access_token, "account_id" : account_id, "type" :
                    type_, "network" : network, "amount" : amount, "ach_class" :
                    ach_class, "user" : user, "device" : device, "origination_account_id"
                    : origination_account_id, "iso_currency_code" : iso_currency_code }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub idempotency_key: String,
    pub access_token: String,
    pub account_id: String,
    pub authorization_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub description: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub origination_account_id: Option<String>,
    pub iso_currency_code: String,
}
impl<'a> TransferCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferCreateResponse> {
        let mut r = self.client.post("/transfer/create");
        let res = r
            .json(
                json!(
                    { "idempotency_key" : idempotency_key, "access_token" : access_token,
                    "account_id" : account_id, "authorization_id" : authorization_id,
                    "type" : type_, "network" : network, "amount" : amount, "description"
                    : description, "ach_class" : ach_class, "user" : user, "metadata" :
                    metadata, "origination_account_id" : origination_account_id,
                    "iso_currency_code" : iso_currency_code }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub idempotency_key: String,
    pub access_token: String,
    pub account_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub iso_currency_code: String,
    pub description: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub custom_tag: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub origination_account_id: Option<String>,
}
impl<'a> BankTransferCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferCreateResponse> {
        let mut r = self.client.post("/bank_transfer/create");
        let res = r
            .json(
                json!(
                    { "idempotency_key" : idempotency_key, "access_token" : access_token,
                    "account_id" : account_id, "type" : type_, "network" : network,
                    "amount" : amount, "iso_currency_code" : iso_currency_code,
                    "description" : description, "ach_class" : ach_class, "user" : user,
                    "custom_tag" : custom_tag, "metadata" : metadata,
                    "origination_account_id" : origination_account_id }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: i64,
    pub offset: i64,
    pub origination_account_id: Option<String>,
}
impl<'a> TransferListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferListResponse> {
        let mut r = self.client.post("/transfer/list");
        let res = r
            .json(
                json!(
                    { "start_date" : start_date, "end_date" : end_date, "count" : count,
                    "offset" : offset, "origination_account_id" : origination_account_id
                    }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: i64,
    pub offset: i64,
    pub origination_account_id: Option<String>,
    pub direction: Option<String>,
}
impl<'a> BankTransferListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferListResponse> {
        let mut r = self.client.post("/bank_transfer/list");
        let res = r
            .json(
                json!(
                    { "start_date" : start_date, "end_date" : end_date, "count" : count,
                    "offset" : offset, "origination_account_id" : origination_account_id,
                    "direction" : direction }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferCancelRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub transfer_id: String,
}
impl<'a> TransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferCancelResponse> {
        let mut r = self.client.post("/transfer/cancel");
        let res = r
            .json(json!({ "transfer_id" : transfer_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferCancelRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferCancelResponse> {
        let mut r = self.client.post("/bank_transfer/cancel");
        let res = r
            .json(json!({ "bank_transfer_id" : bank_transfer_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferEventListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transfer_id: Option<String>,
    pub account_id: Option<String>,
    pub transfer_type: Option<String>,
    pub event_types: Vec<TransferEventType>,
    pub sweep_id: String,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
}
impl<'a> TransferEventListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferEventListResponse> {
        let mut r = self.client.post("/transfer/event/list");
        let res = r
            .json(
                json!(
                    { "start_date" : start_date, "end_date" : end_date, "transfer_id" :
                    transfer_id, "account_id" : account_id, "transfer_type" :
                    transfer_type, "event_types" : event_types, "sweep_id" : sweep_id,
                    "count" : count, "offset" : offset, "origination_account_id" :
                    origination_account_id }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferEventListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub bank_transfer_id: Option<String>,
    pub account_id: Option<String>,
    pub bank_transfer_type: Option<String>,
    pub event_types: Vec<BankTransferEventType>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub direction: Option<String>,
}
impl<'a> BankTransferEventListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferEventListResponse> {
        let mut r = self.client.post("/bank_transfer/event/list");
        let res = r
            .json(
                json!(
                    { "start_date" : start_date, "end_date" : end_date,
                    "bank_transfer_id" : bank_transfer_id, "account_id" : account_id,
                    "bank_transfer_type" : bank_transfer_type, "event_types" :
                    event_types, "count" : count, "offset" : offset,
                    "origination_account_id" : origination_account_id, "direction" :
                    direction }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferEventSyncRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub after_id: i64,
    pub count: Option<i64>,
}
impl<'a> TransferEventSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferEventSyncResponse> {
        let mut r = self.client.post("/transfer/event/sync");
        let res = r
            .json(json!({ "after_id" : after_id, "count" : count }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferEventSyncRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub after_id: i64,
    pub count: Option<i64>,
}
impl<'a> BankTransferEventSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferEventSyncResponse> {
        let mut r = self.client.post("/bank_transfer/event/sync");
        let res = r
            .json(json!({ "after_id" : after_id, "count" : count }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferSweepGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub sweep_id: String,
}
impl<'a> TransferSweepGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferSweepGetResponse> {
        let mut r = self.client.post("/transfer/sweep/get");
        let res = r
            .json(json!({ "sweep_id" : sweep_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferSweepGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub sweep_id: String,
}
impl<'a> BankTransferSweepGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferSweepGetResponse> {
        let mut r = self.client.post("/bank_transfer/sweep/get");
        let res = r
            .json(json!({ "sweep_id" : sweep_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferSweepListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: i64,
}
impl<'a> TransferSweepListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferSweepListResponse> {
        let mut r = self.client.post("/transfer/sweep/list");
        let res = r
            .json(
                json!(
                    { "start_date" : start_date, "end_date" : end_date, "count" : count,
                    "offset" : offset }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferSweepListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub origination_account_id: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub count: Option<i64>,
}
impl<'a> BankTransferSweepListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferSweepListResponse> {
        let mut r = self.client.post("/bank_transfer/sweep/list");
        let res = r
            .json(
                json!(
                    { "origination_account_id" : origination_account_id, "start_time" :
                    start_time, "end_time" : end_time, "count" : count }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferBalanceGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub origination_account_id: Option<String>,
}
impl<'a> BankTransferBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferBalanceGetResponse> {
        let mut r = self.client.post("/bank_transfer/balance/get");
        let res = r
            .json(json!({ "origination_account_id" : origination_account_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferMigrateAccountRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub account_number: String,
    pub routing_number: String,
    pub account_type: String,
}
impl<'a> BankTransferMigrateAccountRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::BankTransferMigrateAccountResponse> {
        let mut r = self.client.post("/bank_transfer/migrate_account");
        let res = r
            .json(
                json!(
                    { "account_number" : account_number, "routing_number" :
                    routing_number, "account_type" : account_type }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferIntentCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub account_id: Option<String>,
    pub mode: String,
    pub amount: String,
    pub description: String,
    pub ach_class: String,
    pub origination_account_id: Option<String>,
    pub user: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub iso_currency_code: String,
}
impl<'a> TransferIntentCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferIntentCreateResponse> {
        let mut r = self.client.post("/transfer/intent/create");
        let res = r
            .json(
                json!(
                    { "account_id" : account_id, "mode" : mode, "amount" : amount,
                    "description" : description, "ach_class" : ach_class,
                    "origination_account_id" : origination_account_id, "user" : user,
                    "metadata" : metadata, "iso_currency_code" : iso_currency_code }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferIntentGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub transfer_intent_id: String,
}
impl<'a> TransferIntentGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferIntentGetResponse> {
        let mut r = self.client.post("/transfer/intent/get");
        let res = r
            .json(json!({ "transfer_intent_id" : transfer_intent_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferRepaymentListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: i64,
}
impl<'a> TransferRepaymentListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferRepaymentListResponse> {
        let mut r = self.client.post("/transfer/repayment/list");
        let res = r
            .json(
                json!(
                    { "start_date" : start_date, "end_date" : end_date, "count" : count,
                    "offset" : offset }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferRepaymentReturnListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub repayment_id: String,
    pub count: Option<i64>,
    pub offset: i64,
}
impl<'a> TransferRepaymentReturnListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::TransferRepaymentReturnListResponse> {
        let mut r = self.client.post("/transfer/repayment/return/list");
        let res = r
            .json(
                json!(
                    { "repayment_id" : repayment_id, "count" : count, "offset" : offset }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxBankTransferSimulateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub bank_transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<serde_json::Value>,
}
impl<'a> SandboxBankTransferSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxBankTransferSimulateResponse> {
        let mut r = self.client.post("/sandbox/bank_transfer/simulate");
        let res = r
            .json(
                json!(
                    { "bank_transfer_id" : bank_transfer_id, "event_type" : event_type,
                    "failure_reason" : failure_reason }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxTransferSweepSimulateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
}
impl<'a> SandboxTransferSweepSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxTransferSweepSimulateResponse> {
        let mut r = self.client.post("/sandbox/transfer/sweep/simulate");
        let res = r
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxTransferSimulateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<serde_json::Value>,
}
impl<'a> SandboxTransferSimulateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxTransferSimulateResponse> {
        let mut r = self.client.post("/sandbox/transfer/simulate");
        let res = r
            .json(
                json!(
                    { "transfer_id" : transfer_id, "event_type" : event_type,
                    "failure_reason" : failure_reason }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxTransferRepaymentSimulateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
}
impl<'a> SandboxTransferRepaymentSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxTransferRepaymentSimulateResponse> {
        let mut r = self.client.post("/sandbox/transfer/repayment/simulate");
        let res = r
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct EmployersSearchRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub query: String,
    pub products: Vec<String>,
}
impl<'a> EmployersSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EmployersSearchResponse> {
        let mut r = self.client.post("/employers/search");
        let res = r
            .json(json!({ "query" : query, "products" : products }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub webhook: String,
    pub precheck_id: String,
    pub options: serde_json::Value,
}
impl<'a> IncomeVerificationCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IncomeVerificationCreateResponse> {
        let mut r = self.client.post("/income/verification/create");
        let res = r
            .json(
                json!(
                    { "webhook" : webhook, "precheck_id" : precheck_id, "options" :
                    options }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationSummaryGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationSummaryGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationSummaryGetResponse> {
        let mut r = self.client.post("/income/verification/summary/get");
        let res = r
            .json(
                json!(
                    { "income_verification_id" : income_verification_id, "access_token" :
                    access_token }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationPaystubGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationPaystubGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationPaystubGetResponse> {
        let mut r = self.client.post("/income/verification/paystub/get");
        let res = r
            .json(
                json!(
                    { "income_verification_id" : income_verification_id, "access_token" :
                    access_token }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationPaystubsGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationPaystubsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationPaystubsGetResponse> {
        let mut r = self.client.post("/income/verification/paystubs/get");
        let res = r
            .json(
                json!(
                    { "income_verification_id" : income_verification_id, "access_token" :
                    access_token }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationRefreshRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IncomeVerificationRefreshResponse> {
        let mut r = self.client.post("/income/verification/refresh");
        let res = r
            .json(
                json!(
                    { "income_verification_id" : income_verification_id, "access_token" :
                    access_token }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationTaxformsGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationTaxformsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationTaxformsGetResponse> {
        let mut r = self.client.post("/income/verification/taxforms/get");
        let res = r
            .json(
                json!(
                    { "income_verification_id" : income_verification_id, "access_token" :
                    access_token }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationPrecheckRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub user: Option<serde_json::Value>,
    pub employer: Option<serde_json::Value>,
    pub transactions_access_token: serde_json::Value,
    pub transactions_access_tokens: Vec<AccessToken>,
    pub us_military_info: Option<serde_json::Value>,
}
impl<'a> IncomeVerificationPrecheckRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationPrecheckResponse> {
        let mut r = self.client.post("/income/verification/precheck");
        let res = r
            .json(
                json!(
                    { "user" : user, "employer" : employer, "transactions_access_token" :
                    transactions_access_token, "transactions_access_tokens" :
                    transactions_access_tokens, "us_military_info" : us_military_info }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct EmploymentVerificationGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
}
impl<'a> EmploymentVerificationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EmploymentVerificationGetResponse> {
        let mut r = self.client.post("/employment/verification/get");
        let res = r
            .json(json!({ "access_token" : access_token }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchAltCreateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub target_account: serde_json::Value,
    pub target_user: serde_json::Value,
    pub options: serde_json::Value,
    pub country_code: Option<String>,
}
impl<'a> DepositSwitchAltCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchAltCreateResponse> {
        let mut r = self.client.post("/deposit_switch/alt/create");
        let res = r
            .json(
                json!(
                    { "target_account" : target_account, "target_user" : target_user,
                    "options" : options, "country_code" : country_code }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxBankTransferFireWebhookRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub webhook: String,
}
impl<'a> SandboxBankTransferFireWebhookRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxBankTransferFireWebhookResponse> {
        let mut r = self.client.post("/sandbox/bank_transfer/fire_webhook");
        let res = r
            .json(json!({ "webhook" : webhook }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxIncomeFireWebhookRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub income_verification_id: String,
    pub item_id: String,
    pub webhook: String,
    pub verification_status: String,
}
impl<'a> SandboxIncomeFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxIncomeFireWebhookResponse> {
        let mut r = self.client.post("/sandbox/income/fire_webhook");
        let res = r
            .json(
                json!(
                    { "income_verification_id" : income_verification_id, "item_id" :
                    item_id, "webhook" : webhook, "verification_status" :
                    verification_status }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxOauthSelectAccountsRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub oauth_state_id: String,
    pub accounts: Vec<String>,
}
impl<'a> SandboxOauthSelectAccountsRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxOauthSelectAccountsResponse> {
        let mut r = self.client.post("/sandbox/oauth/select_accounts");
        let res = r
            .json(json!({ "oauth_state_id" : oauth_state_id, "accounts" : accounts }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SignalEvaluateRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub access_token: String,
    pub account_id: String,
    pub client_transaction_id: String,
    pub amount: f64,
    pub user_present: Option<bool>,
    pub client_user_id: String,
    pub user: serde_json::Value,
    pub device: serde_json::Value,
}
impl<'a> SignalEvaluateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SignalEvaluateResponse> {
        let mut r = self.client.post("/signal/evaluate");
        let res = r
            .json(
                json!(
                    { "access_token" : access_token, "account_id" : account_id,
                    "client_transaction_id" : client_transaction_id, "amount" : amount,
                    "user_present" : user_present, "client_user_id" : client_user_id,
                    "user" : user, "device" : device }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SignalDecisionReportRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub client_transaction_id: String,
    pub initiated: bool,
    pub days_funds_on_hold: Option<i64>,
}
impl<'a> SignalDecisionReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SignalDecisionReportResponse> {
        let mut r = self.client.post("/signal/decision/report");
        let res = r
            .json(
                json!(
                    { "client_transaction_id" : client_transaction_id, "initiated" :
                    initiated, "days_funds_on_hold" : days_funds_on_hold }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SignalReturnReportRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub client_transaction_id: String,
    pub return_code: String,
}
impl<'a> SignalReturnReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SignalReturnReportResponse> {
        let mut r = self.client.post("/signal/return/report");
        let res = r
            .json(
                json!(
                    { "client_transaction_id" : client_transaction_id, "return_code" :
                    return_code }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletGetRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub wallet_id: String,
}
impl<'a> WalletGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletGetResponse> {
        let mut r = self.client.post("/wallet/get");
        let res = r
            .json(json!({ "wallet_id" : wallet_id }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletTransactionExecuteRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub idempotency_key: String,
    pub wallet_id: String,
    pub counterparty: serde_json::Value,
    pub amount: serde_json::Value,
    pub reference: String,
}
impl<'a> WalletTransactionExecuteRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletTransactionExecuteResponse> {
        let mut r = self.client.post("/wallet/transaction/execute");
        let res = r
            .json(
                json!(
                    { "idempotency_key" : idempotency_key, "wallet_id" : wallet_id,
                    "counterparty" : counterparty, "amount" : amount, "reference" :
                    reference }
                ),
            )
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletTransactionsListRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub wallet_id: String,
    pub cursor: String,
    pub count: i64,
}
impl<'a> WalletTransactionsListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletTransactionsListResponse> {
        let mut r = self.client.post("/wallet/transactions/list");
        let res = r
            .json(json!({ "wallet_id" : wallet_id, "cursor" : cursor, "count" : count }))
            .authenticate(&self.authentication)
            .send()
            .await
            .unwrap()
            .error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
