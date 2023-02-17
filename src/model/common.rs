use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResFormat<T> {
    pub code: u8,
    pub data: T,
    pub msg: String,
}
