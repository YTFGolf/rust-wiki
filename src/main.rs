use clap::Parser;
use rust_wiki::{
    cli::{stage_info, update_config, Cli, Command}, config::get_config, wiki_files::update_wiki_files
};

fn main() {
    let cli = Cli::parse();
    // FIXME currently assumes that if config is None then the file doesn't
    // exist, but could also be an error with toml parsing if toml doesn't
    // contain every field.
    let config = get_config();

    match cli.command {
        Command::ReadWiki => update_wiki_files(&config.unwrap()),
        Command::StageInfo(si) => stage_info(si, &config.unwrap()),
        Command::Config(args) => update_config(config, args),
    }
}
