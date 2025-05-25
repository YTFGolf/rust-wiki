use std::process::exit;

use clap::Parser;
use rust_wiki::{
    CONFIG_FILE, Cli, Config,
    logger::{init_logger, set_log_level},
};

/// Initialise the user config.
fn initialise_config() -> ! {
    println!("Config not found, initialising...");
    Config::initialise();
    println!("Config initialised at {CONFIG_FILE}. Exiting program.");

    exit(0)
}

fn main() {
    let cli = Cli::parse();
    let config: Config =
        toml::from_str(&Config::read_config_file().unwrap_or_else(|| initialise_config())).unwrap();

    init_logger();
    unsafe { set_log_level(config.log_level) };
    cli.exec(config);
}
