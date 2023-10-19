//////// structures
#[allow(dead_code)]
use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub id: String,
    pub active_address: String,
    pub active_client_id: String,
    pub active_mac_address: String,
    pub active_server: String,
    pub address: String,
    pub address_lists: String,
    pub age: String,
    pub blocked: String,
    pub client_id: String,
    pub dhcp_option: String,
    pub disabled: String,
    pub dynamic: String,
    pub expires_after: String,
    pub host_name: String,
    pub last_seen: String,
    pub mac_address: String,
    pub radius: String,
    pub server: String,
    pub status: String,
}

//pub struct Host {
//    pub id: Option<&str>,
//    pub active_address: Option<&str>,
//    pub active_client_id: Option<&str>,
//    pub active_mac_address: Option<&str>,
//    pub active_server: Option<&str>,
//    pub address: Option<&str>,
//    pub address_lists: Option<&str>,
//    pub age: Option<&str>,
//    pub blocked: Option<&str>,
//    pub client_id: Option<&str>,
//    pub dhcp_option: Option<&str>,
//    pub disabled: Option<&str>,
//    pub dynamic: Option<&str>,
//    pub expires_after: Option<&str>,
//    pub host_name: Option<&str>,
//    pub last_seen: Option<&str>,
//    pub mac_address: Option<&str>,
//    pub radius: Option<&str>,
//    pub server: Option<&str>,
//    pub status: Option<&str>,
//}

//pub struct Host {
//    pub id: Option<String>,
//    pub active_address: Option<String>,
//    pub active_client_id: Option<String>,
//    pub active_mac_address: Option<String>,
//    pub active_server: Option<String>,
//    pub address: Option<String>,
//    pub address_lists: Option<String>,
//    pub age: Option<String>,
//    pub blocked: Option<String>,
//    pub client_id: Option<String>,
//    pub dhcp_option: Option<String>,
//    pub disabled: Option<String>,
//    pub dynamic: Option<String>,
//    pub expires_after: Option<String>,
//    pub host_name: Option<String>,
//    pub last_seen: Option<String>,
//    pub mac_address: Option<String>,
//    pub radius: Option<String>,
//    pub server: Option<String>,
//    pub status: Option<String>,
//}
