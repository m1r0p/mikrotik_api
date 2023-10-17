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
    for i in vec_config.iter() {
        println!("{}", i);
    }
}
