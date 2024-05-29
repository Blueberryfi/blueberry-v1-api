use gql_client::Client;

#[derive(Clone)]
pub struct GlobalConfig {
    pub rpc_url: String,
    pub ponder_client: Client,
}
