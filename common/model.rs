use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EtherScanResult<T>{
    pub status: String,
    pub message: String,
    pub result: T,
}