use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReqBalance {
    pub contract: String,
    pub account: String,
}
pub struct ReqAllowance {
    pub contract: String,
    pub owner: String,
    pub spender: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResBalance {
    pub balance: String,
}
