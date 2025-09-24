use clap::Parser;
use rust_wiki::{
    CONFIG_FILE, Cli, Config,
    logger::{init_logger, set_log_level},
};
use std::process::exit;

/// Initialise the user config.
fn initialise_config() -> ! {
    println!("Config not found, initialising...");
    Config::initialise();
    println!("Config initialised at {CONFIG_FILE}. Exiting program.");

    exit(0)
}

fn get_config() -> Config {
    let file_content = &Config::read_config_file().unwrap_or_else(|| initialise_config());
    match toml::from_str(file_content) {
        Ok(config) => config,
        Err(e) => panic!("Error when parsing config: {e}"),
    }
}

fn main() {
    let cli = Cli::parse();
    let config = get_config();

    init_logger();
    unsafe { set_log_level(config.log_level) };
    cli.exec(config);
}
