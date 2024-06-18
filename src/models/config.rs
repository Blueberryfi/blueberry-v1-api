use alloy::transports::http::Http;
use alloy::{providers::RootProvider, transports::http::reqwest};
use gql_client::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct GlobalConfig {
    pub provider: Arc<RootProvider<Http<reqwest::Client>>>,
    pub ponder_client: Client,
}
