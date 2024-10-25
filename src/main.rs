use clap::Parser;
use rust_wiki::{
    cli::{stage_info, update_config, Cli, Command},
    config::get_user_config,
    wiki_files::update_wiki_files,
};

fn main() {
    let cli = Cli::parse();
    let config = get_user_config();

    match cli.command {
        Command::ReadWiki => {
            update_wiki_files(&config.expect("Config not found or is invalid!").into())
        }
        Command::StageInfo(si) => {
            stage_info(si, &config.expect("Config not found or is invalid!").into())
        }
        Command::Config(args) => update_config(config, args),
    }
}
