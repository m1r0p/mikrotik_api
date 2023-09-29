pub mod conf;
pub use conf::API_URL;
pub mod structures;
pub use structures::Mikrotik;

//use std::fs;
use config::DefaultConfigurationBuilder;
use std::path::PathBuf;
//use config::builder::ConfigBuilder;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn config_parse(string_path: String) {
    //let config_content =
    //    fs::read_to_string(&config_path).expect("Should have been able to read the file");

    let config_path = PathBuf::from(string_path.as_str());
    let config = DefaultConfigurationBuilder::new()
        .add_json_file(&config_path)
        .build();

    //let mut builder = Config::builder();
    //builder = builder.set_default("default", "1")?;
    //builder = builder.add_source(File::new(&string_path, FileFormat::Json));
    //builder = builder.set_override("override", "1")?;

    //match builder.build() {
    //    Ok(config) => {
    //        // use your config
    //        println!("{}", config)
    //    }
    //    Err(e) => {
    //        // something went wrong
    //    }
    //}

    let ip = config.get("ip").unwrap();
    let user = config.get("user").unwrap();
    let password = config.get("password").unwrap();

    println!("ip = {}, user = {}, pass = {}", &ip, &user, &password);
    //print_type_of(&config_content);
}
