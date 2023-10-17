pub mod conf;
//pub use conf::API_URL;
pub mod structures;
pub use structures::Host;

use std::error::Error;
//use std::fs;
//use config::DefaultConfigurationBuilder;
//use std::path::PathBuf;
//use config::builder::ConfigBuilder;
use config::{Config, File, FileFormat};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

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
