use async_graphql::SimpleObject;
use serde::Serialize;
use torrent_search_client::Provider;

#[derive(SimpleObject, Serialize)]
pub struct ProviderError {
    provider: Provider,
    error: String,
}

impl ProviderError {
    pub fn new(provider: Provider, error: String) -> Self {
        Self { provider, error }
    }
}
