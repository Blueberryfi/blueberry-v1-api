use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalance {
    pub address: String,
    pub effective_balance: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherfiPoints {
    pub result: Vec<UserBalance>,
}
