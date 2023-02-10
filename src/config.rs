use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, process::exit};
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
    for l in config.link.clone().into_iter() {
        match l.0.as_str() {
            "script" => {}
            "load-address" => {}
            _ => eprintln!("ignoring unknown option '{} = {}'", l.0, l.1),
        }
    }

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
