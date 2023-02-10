use config::{generate_build_args, read_config};

mod config;

fn main() {
    let config = read_config("config/config_default.toml".into());
    generate_build_args(&config);

    let config = read_config("config/config_nezha.toml".into());
    generate_build_args(&config);
}
