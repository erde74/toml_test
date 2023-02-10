use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    process::exit,
};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub conf: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub features: Option<Vec<String>>,
    pub dev: Option<Vec<String>>,
    pub ip: Option<Vec<String>>,
    pub link: Option<Vec<String>>,
    pub nodev: Option<Vec<String>>,
    pub nouart: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub link: Option<HashMap<String, String>>,
    pub config: Option<Config>,
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
    if let Some(config) = &config.config {
        if let Some(features) = &config.features {
            let joined = features.join(",");
            println!("--features=\"{joined}\"");
        }

        if let Some(devices) = &config.dev {
            for dev in devices {
                println!("--cfg dev_{dev}");
            }
        }

        if let Some(ips) = &config.ip {
            for ip in ips {
                println!("--cfg ip_{ip}");
            }
        }
        if let Some(links) = &config.link {
            for link in links {
                println!("--cfg link_{link}");
            }
        }

        if let Some(nodevs) = &config.nodev {
            for nodev in nodevs {
                println!("--cfg nodev_{nodev}");
            }
        }

        if let Some(nouarts) = &config.nouart {
            for nouart in nouarts {
                println!("--cfg nouart_{nouart}");
            }
        }
    }
}

pub fn adjust_linker_script(config: &Configuration) {
    let mut filename: String = "".into();
    let mut load_address: String = "".into();

    if let Some(link) = &config.link {
        for l in link.into_iter() {
            match l.0.as_str() {
                "script" => filename = l.1.clone(),
                "load-address" => load_address = l.1.clone(),
                _ => eprintln!("ignoring unknown option '{} = {}'", l.0, l.1),
            }
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
