//! Parses cli arguments for use in main.
use super::{
    commands::StageInfo,
    user_config::{create_config, UserConfig, UserConfigCli},
};
use crate::{
    config::{set_config_file, Config},
    data::stage::parsed::stage::Stage,
    wikitext::stage_info::get_stage_info,
};
use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().lines().next().unwrap().unwrap()
}

// TODO StageInfo is a bad name
/// Get stage info.
pub fn stage_info(info: StageInfo, config: &Config) {
    println!("{info:?}");
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
    println!("{selector:?}");
    println!(
        "{}",
        get_stage_info(
            &Stage::new_versioned(&selector, &config.current_version).unwrap(),
            config
        )
    )
}

/// Update user config.
pub fn update_config(config: Option<UserConfig>, args: UserConfigCli) {
    let mut config = match config {
        None => return create_config(args),
        Some(c) => c,
    };

    // merge configs
    if let Some(path) = args.path {
        config.version.path = path;
    }
    if let Some(name) = args.username {
        config.username = name;
    }
    if let Some(s) = args.suppress {
        config.suppress_gauntlet_magnification = s;
    }

    let toml_repr = toml::to_string(&config).unwrap();
    set_config_file(&toml_repr);
}

/*
TODO

- ~~Remove static CONFIG variable and replace with borrow passed everywhere~~
  - Maybe for testing have a test_config static
  - Rename old CONFIG and Stage::new
- Implement create and update config
- Allow cmd options to override user config options
*/
