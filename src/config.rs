use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    process::exit, path::Path,
};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub conf: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub features: Vec<String>,
    pub dev: Vec<String>,
    pub ip: Vec<String>,
    pub link: Vec<String>,
    pub nodev: Vec<String>,
    pub nouart: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub link: HashMap<String, String>,
    pub config: Config,
}

pub fn read_config(filename: String) -> Configuration {
    let contents = match fs::read_to_string(filename.clone()) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{filename}`");
            exit(1);
        }
    };
    let config: Configuration = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ConfigFile: Unable to load data from `{}`", filename);
            eprintln!("{e}");
            exit(1);
        }
    };
    config
}

pub fn generate_build_args(config: &Configuration) {
    let joined = config.config.features.join(",");
    println!("--features=\"{joined}\"");

    for dev in &config.config.dev {
        println!("--cfg dev_{dev}");
    }

    for ip in &config.config.ip {
        println!("--cfg ip_{ip}");
    }

    for link in &config.config.link {
        println!("--cfg link_{link}");
    }

    for nodev in &config.config.nodev {
        println!("--cfg nodev_{nodev}");
    }

    for nouart in &config.config.nouart {
        println!("--cfg nouart_{nouart}");
    }
}

pub fn adjust_linker_script(config: &Configuration) {
    let mut filename: String = "".into();
    let mut load_address: String = "".into();

    for l in config.link.clone().into_iter() {
        match l.0.as_str() {
            "script" => filename = l.1,
            "load-address" => load_address = l.1,
            _ => eprintln!("ignoring unknown option '{} = {}'", l.0, l.1),
        }
    }

    if !filename.is_empty() && !load_address.is_empty() {
        let mut contents = match fs::read_to_string(filename.clone()) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{filename}`");
                exit(1);
            }
        };
        contents = contents.replace("${LOAD-ADDRESS}", &load_address);
        let mut file = File::create("target/debug/kernel.ld").unwrap();
        file.write_all(contents.as_bytes());
    }
}
