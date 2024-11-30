use clap::Parser;
use rust_wiki::{
    cli::{
        commands::{Cli, Command},
        parse::{merge_config_and_args, stage_info, update_config},
        user_config::{UserConfig, UserConfigCli},
    },
    config::{get_user_config, Config},
    wiki_files::update_wiki_files,
};

fn get_config(config: Option<UserConfig>, args: UserConfigCli) -> Config {
    let config = config.expect("Config not found or is invalid!");
    merge_config_and_args(config, args).into()
}

fn test() {
    if true {
        return;
    }

    let wiki_enemy_id = 703;
    let config: Config = get_user_config().unwrap().into();

    rust_wiki::wikitext::encounters::do_thing(wiki_enemy_id, &config);

    if true {
        std::process::exit(0)
    }
}

fn main() {
    test();

    let cli = Cli::parse();
    let config = get_user_config();

    match cli.command {
        Command::ReadWiki(c) => update_wiki_files(&get_config(config, c)),
        Command::StageInfo(si) => {
            let config = &get_config(config, si.config.clone());
            stage_info(si, config)
        }
        Command::Config(c) => update_config(config, c),
    }
}
