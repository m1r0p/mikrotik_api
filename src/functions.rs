#[allow(dead_code)]
pub mod conf;
pub use conf::{MIKROTIK_DHCP_LEASES, MIKROTIK_PROTO};
pub mod structures;
pub use structures::Host;

use config::{Config, File, FileFormat};
//use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use std::error::Error;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

pub fn get_mikrotik_params(string_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut mikrotik_params: Vec<String> = Vec::new();

    let mut builder = Config::builder();
    builder = builder.set_default("default", "1")?;
    builder = builder.add_source(File::new(&string_path, FileFormat::Json));
    builder = builder.set_override("override", "1")?;
    let raw_conf = builder.build().unwrap();
    mikrotik_params.push(raw_conf.get("mikrotik_ip").unwrap());
    mikrotik_params.push(raw_conf.get("mikrotik_user").unwrap());
    mikrotik_params.push(raw_conf.get("mikrotik_password").unwrap());
    return Ok(mikrotik_params);
}

//#[tokio::main]
//pub async fn get_dhcp_hosts(mikrotik_params: Vec<String> ) -> Result<Vec<Host>, Box<dyn Error>> {
//    let mut headers = HeaderMap::new();
//    headers.insert(AUTHORIZATION, format!("Bearer {}", TOKEN).parse().unwrap());
//    headers.insert(
//        CONTENT_TYPE,
//        format!("application/json-rpc").parse().unwrap(),
//    );
//    let host_request = format!(
//        r#"{{"jsonrpc":"2.0","method":"host.get","params":{{"groupids":{}, "output":["host","name"]}},"id":1}}"#,
//        GROUP_ID
//    );
//    let client = reqwest::Client::new();
//    let hosts_raw_data = client
//        .post(ZABBIX_URL)
//        .headers(headers.clone())
//        .body(host_request)
//        .send()
//        .await?
//        .text()
//        .await?;
//    let hosts_json: Value = serde_json::from_str(hosts_raw_data.as_str()).unwrap();
//    let hosts_vec: &Vec<Value> = hosts_json["result"].as_array().unwrap();
//    let mut tmp_hosts: Vec<Host> = Vec::new();
//    for i in hosts_vec.iter() {
//        let id: u32 = i["hostid"].as_str().unwrap().parse()?;
//        let host: Host = Host {
//            id: id,
//            host: i["host"].as_str().unwrap().to_string(),
//            name: i["name"].as_str().unwrap().to_string(),
//            ipv4: "None".to_string(),
//        };
//        tmp_hosts.push(host);
//    }
//
//    let mut zabbix_hosts: Vec<Host> = Vec::new();
//    for i in tmp_hosts.iter() {
//        let ip_request = format!(
//            r#"{{"jsonrpc":"2.0","method":"hostinterface.get","params":{{"hostids":{}, "output":["interfaceid","ip"]}},"id":1}}"#,
//            i.id
//        );
//        let client = reqwest::Client::new();
//        let ip_raw_data = client
//            .post(ZABBIX_URL)
//            .headers(headers.clone())
//            .body(ip_request)
//            .send()
//            .await?
//            .text()
//            .await?;
//        let ip_json: Value = serde_json::from_str(ip_raw_data.as_str()).unwrap();
//        let host: Host = Host {
//            id: i.id,
//            host: i.host.clone(),
//            name: i.name.clone(),
//            ipv4: ip_json["result"][0]["ip"].as_str().unwrap().to_string()
//        };
//        zabbix_hosts.push(host);
//
//
//    }
//
//    return Ok(dhcp_hosts);
//}

#[tokio::main]
pub async fn get_dhcp_leases(mikrotik_params: Vec<String>) -> Result<Vec<Host>, Box<dyn Error>> {
    let mut dhcp_leases: Vec<Host> = Vec::new();
    let client = reqwest::Client::new();
    let user_name: String = mikrotik_params[1].to_string();
    let password: Option<String> = Some(mikrotik_params[2].to_string());

    let resp = client
        .get(format!(
            "{}{}{}",
            MIKROTIK_PROTO, mikrotik_params[0], MIKROTIK_DHCP_LEASES
        ))
        .basic_auth(user_name, password)
        .send()
        .await?
        .text()
        .await?;
    let hosts_json: Value = serde_json::from_str(resp.as_str()).unwrap();
    let hosts_vec: &Vec<Value> = hosts_json.as_array().unwrap();
    for i in hosts_vec.iter() {
        //let mut address_lists: String = String::new();
        //if i["address-lists"].as_str().is_empty()  {
        //    address_lists.push_str("None");
        //} else {
        //    address_lists.push_str(i["address-lists"].as_str().unwrap());
        //}
        //let mut dhcp_option: String = String::new();
        //if i["dhcp-option"].as_str().is_empty()  {
        //    dhcp_option.push_str("None");
        //} else {
        //    dhcp_option.push_str(i["dhcp-option"].as_str().unwrap());
        //}
        

        let mut id: String = String::new();
        match i[".id"].as_str() {
            None => id.push_str("None"),
            Some(x) => id.push_str(x),
        }
        let mut active_address: String = String::new();
        match i["active-address"].as_str() {
            None => active_address.push_str("None"),
            Some(x) => active_address.push_str(x),
        }
        let mut active_client_id: String = String::new();
        match i["active-client-id"].as_str() {
            None => active_client_id.push_str("None"),
            Some(x) => active_client_id.push_str(x),
        }
        let mut active_mac_address: String = String::new();
        match i["active-mac-address"].as_str() {
            None => active_mac_address.push_str("None"),
            Some(x) => active_mac_address.push_str(x),
        }
        let mut active_server: String = String::new();
        match i["active-server"].as_str() {
            None => active_server.push_str("None"),
            Some(x) => active_server.push_str(x),
        }
        let mut address: String = String::new();
        match i["address"].as_str() {
            None => address.push_str("None"),
            Some(x) => address.push_str(x),
        }
        let mut address_lists: String = String::new();
        match i["address-lists"].as_str() {
            None => address_lists.push_str("None"),
            Some(x) => address_lists.push_str(x),
        }
        let mut age: String = String::new();
        match i["age"].as_str() {
            None => age.push_str("None"),
            Some(x) => age.push_str(x),
        }
        let mut blocked: String = String::new();
        match i["blocked"].as_str() {
            None => blocked.push_str("None"),
            Some(x) => blocked.push_str(x),
        }
        let mut client_id: String = String::new();
        match i["client-id"].as_str() {
            None => client_id.push_str("None"),
            Some(x) => client_id.push_str(x),
        }
        let mut dhcp_option: String = String::new();
        match i["dhcp-option"].as_str() {
            None => dhcp_option.push_str("None"),
            Some(x) => dhcp_option.push_str(x),
        }
        let mut disabled: String = String::new();
        match i["disabled"].as_str() {
            None => disabled.push_str("None"),
            Some(x) => disabled.push_str(x),
        }
        let mut dynamic: String = String::new();
        match i["dynamic"].as_str() {
            None => dynamic.push_str("None"),
            Some(x) => dynamic.push_str(x),
        }
        let mut expires_after: String = String::new();
        match i["expires-after"].as_str() {
            None => expires_after.push_str("None"),
            Some(x) => expires_after.push_str(x),
        }
        let mut host_name: String = String::new();
        match i["host-name"].as_str() {
            None => host_name.push_str("None"),
            Some(x) => host_name.push_str(x),
        }
        let mut last_seen: String = String::new();
        match i["last-seen"].as_str() {
            None => last_seen.push_str("None"),
            Some(x) => last_seen.push_str(x),
        }
        let mut mac_address: String = String::new();
        match i["mac-address"].as_str() {
            None => mac_address.push_str("None"),
            Some(x) => mac_address.push_str(x),
        }
        let mut radius: String = String::new();
        match i["radius"].as_str() {
            None => radius.push_str("None"),
            Some(x) => radius.push_str(x),
        }
        let mut server: String = String::new();
        match i["server"].as_str() {
            None => server.push_str("None"),
            Some(x) => server.push_str(x),
        }
        let mut status: String = String::new();
        match i["status"].as_str() {
            None => status.push_str("None"),
            Some(x) => status.push_str(x),
        }

        let host: Host = Host {
            id: id, 
            active_address: active_address,
            active_client_id: active_client_id,
            active_mac_address: active_mac_address,
            active_server: active_server,
            address: address,
            address_lists: address_lists,
            age: age,
            blocked: blocked,
            client_id: client_id,
            dhcp_option: dhcp_option,
            disabled: disabled,
            dynamic: dynamic,
            expires_after: expires_after,
            host_name: host_name,
            last_seen: last_seen,
            mac_address: mac_address,
            radius: radius,
            server: server,
            status: status,
        };

        //let host: Host = Host {
        //    id: i[".id"].as_str().map(Into::into),
        //    active_address: i["active-address"].as_str().map(Into::into),
        //    active_client_id: i["active-client-id"].as_str().map(Into::into),
        //    active_mac_address: i["active-mac-address"].as_str().map(Into::into),
        //    active_server: i["active-server"].as_str().map(Into::into),
        //    address: i["address"].as_str().map(Into::into),
        //    address_lists: i["address-lists"].as_str().map(Into::into),
        //    age: i["age"].as_str().map(Into::into),
        //    blocked: i["blocked"].as_str().map(Into::into),
        //    client_id: i["client-id"].as_str().map(Into::into),
        //    dhcp_option: i["dhcp-option"].as_str().map(Into::into),
        //    disabled: i["disabled"].as_str().map(Into::into),
        //    dynamic: i["dynamic"].as_str().map(Into::into),
        //    expires_after: i["expires-after"].as_str().map(Into::into),
        //    host_name: i["host-name"].as_str().map(Into::into),
        //    last_seen: i["last-seen"].as_str().map(Into::into),
        //    mac_address: i["mac-address"].as_str().map(Into::into),
        //    radius: i["radius"].as_str().map(Into::into),
        //    server: i["server"].as_str().map(Into::into),
        //    status: i["status"].as_str().map(Into::into),
        //};

        dhcp_leases.push(host);
        //println!("{:?}\n\n\n\n", i);
    }

    //println!("{:?}", &hosts_json);
    //print_type_of(&hosts_json);
    return Ok(dhcp_leases);
}
