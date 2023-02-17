use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReqCoinTask {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct CoinBalance {
    pub address: String,
    pub balance: u32,
}
