pub mod conf;
//pub use conf::API_URL;
pub mod structures;
pub use structures::Mikrotik;

use std::error::Error;
//use std::fs;
//use config::DefaultConfigurationBuilder;
//use std::path::PathBuf;
//use config::builder::ConfigBuilder;
use config::{Config, File, FileFormat};

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

pub fn config_parse(string_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut credentials: Vec<String> = Vec::new();

    let mut builder = Config::builder();
    builder = builder.set_default("default", "1")?;
    builder = builder.add_source(File::new(&string_path, FileFormat::Json));
    builder = builder.set_override("override", "1")?;
    let raw_conf = builder.build().unwrap();
    credentials.push(raw_conf.get("ip").unwrap());
    credentials.push(raw_conf.get("user").unwrap());
    credentials.push(raw_conf.get("password").unwrap());
    return Ok(credentials);
}
