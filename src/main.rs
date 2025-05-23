use clap::Parser;
use rust_wiki::{
    interface::cli::commands::Cli,
    config::{CONFIG_FILE, Config},
    logger::{init_logger, set_log_level},
};
use std::process::exit;

fn initialise_config() -> ! {
    println!("Config not found, initialising...");
    Config::initialise();
    println!("Config initialised at {CONFIG_FILE}. Exiting program.");

    exit(0);
}

fn main() {
    let cli = Cli::parse();
    let config: Config =
        toml::from_str(&Config::read_config_file().unwrap_or_else(|| initialise_config())).unwrap();

    init_logger();
    unsafe { set_log_level(config.log_level) };
    cli.exec(config);
}
