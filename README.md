<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/plaid-openapi/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/plaid-openapi.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/plaid-openapi/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/plaid-openapi.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/plaid-openapi/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/plaid-openapi/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/plaid-openapi">
    <img src="https://img.shields.io/crates/d/plaid-openapi?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/plaid-openapi">
    <img src="https://img.shields.io/crates/v/plaid-openapi?style=flat-square" alt="Crates.io" />
</a>

</p>

```rust
use plaid_openapi::PlaidClient;

#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token ="access-sandbox-b4957595-eae2-4130-9da7-114d14726a62".to_string();
    let item_get = client.item_get(access_token)
        .await
        .unwrap();
    println!("{:#?}", item_get);
}
```

`plaid-openapi` is a complete, easy-to-use, async client library for the Plaid API, generated from Plaid's OpenAPI spec.

# [Documentation](https://docs.rs/plaid-openapi/latest/plaid_openapi/)

# Contributing

Contributions are welcome!