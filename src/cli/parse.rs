//! Parses cli arguments for use in main.
use super::{
    commands::StageInfoOptions,
    user_config::{create_config, UserConfig, UserConfigCli},
};
use crate::{
    config::{set_config_file, Config},
    data::stage::parsed::stage::Stage,
    wikitext::stage_info::get_stage_info,
};
use std::io::{self, Write};

/// Syntax sugar for a function that works like Python's `input`.
pub fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().lines().next().unwrap().unwrap()
}

/// Get stage info.
pub fn stage_info(info: StageInfoOptions, config: &Config) {
    // println!("{info:?}");
    let selector = match info.selector.len() {
        1 => {
            let mut s = info.selector;
            // So this becomes an explicit mutable move (i.e. makes it so
            // swap_remove actually returns an owned string)
            s.swap_remove(0)
        }
        0 => input("Input file selector: "),
        _ => info.selector.join(" "),
    };
    // println!("{selector:?}");
    println!(
        "{}",
        get_stage_info(
            &Stage::new(&selector, &config.current_version).unwrap(),
            config
        )
    );
}

/// Replace all fields of `config` with appropriate fields from `args` if they
/// exist.
pub fn merge_config_and_args(config: UserConfig, args: UserConfigCli) -> UserConfig {
    let mut config = config;
    if let Some(path) = args.path {
        config.version.path = path;
    }
    if let Some(name) = args.username {
        config.username = name;
    }
    if let Some(s) = args.suppress {
        config.suppress_gauntlet_magnification = s;
    }

    config
}

/// Update user config.
pub fn update_config(config: Option<UserConfig>, args: UserConfigCli) {
    let config = match config {
        None => return create_config(args),
        Some(c) => c,
    };

    let config = merge_config_and_args(config, args);
    let toml_repr = toml::to_string(&config).unwrap();
    set_config_file(&toml_repr);
}
