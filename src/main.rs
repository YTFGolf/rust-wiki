use clap::Parser;
use rust_wiki::{
    cli::{
        commands::{Cli, Command},
        parse::{merge_config_and_args, stage_info, update_config},
        user_config::{UserConfig, UserConfigCli},
    },
    config::{get_user_config, Config},
    logger::init_logger,
    wiki_files::update_wiki_files,
    wikitext::data_files::enemy_data::ENEMY_DATA,
};

fn get_config(config: Option<UserConfig>, args: UserConfigCli) -> Config {
    let config = config.expect("Config not found or is invalid!");
    merge_config_and_args(config, args).into()
}

fn main() {
    let cli = Cli::parse();
    let config = get_user_config();

    init_logger();

    match cli.command {
        Command::ReadWiki(c) => update_wiki_files(&get_config(config, c)),
        Command::StageInfo(si) => {
            let config = &get_config(config, si.config.clone());
            stage_info(si, config);
        }
        Command::Encounters(e) => {
            let config = &get_config(config, e.config.clone());
            log::warn!("This currently only works on the first enemy");
            let name_or_id = &e.names[0];
            let id = match ENEMY_DATA.get_id_from_name(name_or_id) {
                None => name_or_id.parse().unwrap(),
                Some(id) => *id,
            };

            rust_wiki::wikitext::encounters::do_thing(id, config);
        }
        Command::Config(c) => update_config(config, c),
    }
}
