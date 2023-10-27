//////// structures
#[allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MikrotikLease {
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
    pub comment: String,
}
