use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResFormat<T> {
    pub code: u8,
    pub data: T,
    pub msg: String,
}
// impl ResFormat<T> {
//     fn error(_msg: String) -> ResFormat<String> {
//         ResFormat {
//             code: 400,
//             data: String::from(""),
//             msg: _msg,
//         }
//     }
// }
