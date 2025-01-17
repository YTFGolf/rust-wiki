use clap::Parser;
use rust_wiki::{
    cli::commands::Cli,
    config::{Config, CONFIG_FILE},
    logger::init_logger,
};
use std::process::exit;

fn temp() {
    if true {
        return;
    }
    // use serde::Serialize;
    use rust_wiki::config::Config;
    let def_config = Config::default();
    println!("{def_config:?}");
    println!("{}", toml::to_string(&def_config).unwrap());
    println!("{}", serde_json::to_string(&def_config).unwrap());

    let toml_repr = toml::to_string(&def_config).unwrap();
    println!("{:?}", toml::from_str::<Config>(&toml_repr).unwrap());
}

fn initialise_config() -> ! {
    println!("Config not found, initialising...");
    Config::initialise();
    println!("Config initialised at {CONFIG_FILE}. Exiting program.");

    exit(0);
}

fn main() {
    temp();
    let cli = Cli::parse();
    let config: Config =
        toml::from_str(&Config::read_config_file().unwrap_or_else(|| initialise_config())).unwrap();

    init_logger();
    cli.exec(config);
}
