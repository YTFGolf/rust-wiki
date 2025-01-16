use clap::Parser;
use rust_wiki::{cli::commands::Cli, config2::config2::Config, logger::init_logger};

fn temp() {
    // if true {
    //     return;
    // }
    // use serde::Serialize;
    use rust_wiki::config2::config2::Config;
    let def_config = Config::default();
    println!("{:?}", def_config);
    println!("{}", toml::to_string(&def_config).unwrap());
    println!("{}", serde_json::to_string(&def_config).unwrap());

    let toml_repr = toml::to_string(&def_config).unwrap();
    println!("{:?}", toml::from_str::<Config>(&toml_repr).unwrap());
}

fn main() {
    let cli = Cli::parse();
    let config: Config = toml::from_str(&Config::read_config_file().unwrap()).unwrap();

    init_logger();
    cli.exec(config);
}
