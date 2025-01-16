use clap::Parser;
use rust_wiki::{cli::options::Cli, config2::config2::Config, logger::init_logger};

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
    temp();
    let cli = Cli::parse();
    // let config = get_user_config();
    // println!("{cli:?}, {config:?}");

    init_logger();
    cli.exec(Config::default());

    // match cli.command {
    //     Command::ReadWiki(c) => update_wiki_files(&get_config(config, c)),
    //     Command::Encounters(e) => {
    //         let config = &get_config(config, e.config.clone());
    //         log::warn!("This currently only works on the first enemy");
    //         let name_or_id = &e.names[0];
    //         let id = match ENEMY_DATA.get_id_from_name(name_or_id) {
    //             None => name_or_id.parse().unwrap(),
    //             Some(id) => *id,
    //         };

    //         rust_wiki::wikitext::encounters::do_thing(id, config);
    //     }
    //     Command::Config(c) => update_config(config, c),
    // }
}
