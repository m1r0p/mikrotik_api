mod functions;
use functions::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config_path: String = String::new();
    let mut i: usize = 0;
    for word in args.iter() {
        if word.as_str().eq("--config") {
            config_path.push_str(args[i + 1].as_str());
        }

        i = i + 1;
    }

    let vec_config: Vec<String> = get_config_params(config_path).unwrap();
    let mikrotik_leases: Vec<MikrotikLease> = get_mikrotik_leases(
        &vec_config[0],
        &vec_config[1],
        &vec_config[2],
        &vec_config[3],
    )
    .unwrap();
    let _ = del_phpipam_existing_hosts(&vec_config[4], &vec_config[5], &vec_config[6]);

    for i in mikrotik_leases.iter() {
        let _ = create_phpipam_host(
            &vec_config[4],
            &vec_config[5],
            &vec_config[6],
            &i.address,
            &i.host_name,
            &i.mac_address,
            &i.status,
            &i.dynamic,
            &i.comment,
        );
        println!("{:?} - done", &i.address);
    }
}
