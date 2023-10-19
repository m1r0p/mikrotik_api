mod functions;
use functions::*;

//use std::error::Error;
use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config_path: String = String::new();
    let mut i: usize = 0;
    for word in args.iter() {
        //match word.as_str() {
        //    // Checking for --config directive
        //    "--config" => config_path.push_str(args[i + 1].as_str()),

        //    &_ => continue,
        //}

        if word.as_str().eq("--config") {
            config_path.push_str(args[i + 1].as_str());
        }

        i = i + 1;
    }

    let vec_config: Vec<String> = get_mikrotik_params(config_path).unwrap();
    //for i in vec_config.iter() {
    //    println!("{}", i);
    //}
    let mikrotik_leases: Vec<MikrotikLease> = get_dhcp_leases(vec_config).unwrap();
    for i in mikrotik_leases.iter() {
        //let mut host_name: String = String::new();
        //match &i.host_name {
        //    Some(x) => host_name.push_str(x),
        //    None => host_name.push_str("None"),
        //}
        //let mut active_address: String = String::new();
        //match &i.active_address {
        //    Some(x) => active_address.push_str(x),
        //    None => active_address.push_str("None"),
        //}
        //let mut active_mac_address: String = String::new();
        //match &i.active_mac_address {
        //    Some(x) => active_mac_address.push_str(x),
        //    None => active_mac_address.push_str("None"),
        //}

        println!("{}\t\t\t{}\t\t{}", i.host_name, i.active_address, i.active_mac_address);
    }
}
