use config::{adjust_linker_script, generate_build_args, read_config};

mod config;

fn main() {
    let config = read_config("config/config_default.toml".into());
    let buildargs = generate_build_args(&config);
    adjust_linker_script(&config);
/*
    let config = read_config("config/config_nezha.toml".into());
    generate_build_args(&config);
    adjust_linker_script(&config);
*/
    for ba in buildargs {
        println!("{ba}");
    }
}
