use clap::Parser;
use rust_wiki::{
    cli::{
        commands::{Cli, Command},
        parse::{stage_info, update_config},
        user_config::UserConfig,
    },
    config::{get_user_config, Config},
    wiki_files::update_wiki_files,
};

fn get_config(config: Option<UserConfig>) -> Config {
    config.expect("Config not found or is invalid!").into()
}

fn main() {
    let cli = Cli::parse();
    let config = get_user_config();

    match cli.command {
        Command::ReadWiki => update_wiki_files(&get_config(config)),
        Command::StageInfo(si) => stage_info(si, &get_config(config)),
        Command::Config(args) => update_config(config, args),
    }
}
