//////// structures
#[allow(dead_code)]
use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub id: u32,
    pub host: String,
    pub name: String,
    pub ipv4: String,
}
