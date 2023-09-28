mod functions;
use functions::*;

use std::error::Error;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config_path: String = String::new();
    let mut i: usize = 0;
    for word in args.iter() {
        match word.as_str() {
            // Checking for --config directive
            "--config" => config_path.push_str(args[i + 1].as_str()),

            &_ => println!("directive haven't find"),
        }

        i = i + 1;
    }

    let config_content =
        fs::read_to_string(&config_path).expect("Should have been able to read the file");
    println!("{}", config_content);
}
