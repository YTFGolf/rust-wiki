//! Contains global config values.
use crate::data::version::{InvalidLanguage, Version};
use home::home_dir;
use std::{path::PathBuf, sync::LazyLock};

/// Expand home directory if `dir` begins with `~/`.
fn expand_home(dir: &str) -> PathBuf {
    if &dir[0..2] == "~/" {
        home_dir().unwrap().join(&dir[2..])
    } else {
        PathBuf::from(dir)
    }
}

fn get_version(dir: &str) -> Version {
    match Version::new(
        expand_home(dir),
        Version::get_lang(dir).expect("No language name found in directory name!"),
        Version::get_version_number(dir).expect("No version number found in directory name!"),
    ) {
        Ok(v) => v,
        Err(InvalidLanguage(code)) => panic!("Version language not recognised: {code:?}."),
    }
}

/// Configuration values for the program.
#[derive(Debug)]
pub struct Config<'a> {
    /// Current game version.
    pub current_version: Version,
    /// Your name.
    pub user_name: &'a str,
    /// Make `Magnification` put `|name|0` on gauntlet pages rather than the
    /// enemy's actual magnification.
    pub suppress_gauntlet_magnification: bool,
}

/// Static variable representing the config.
// TODO read a config file instead of writing it here.
// TODO remove dependency on static variable.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    current_version: get_version("~/Downloads/Version 13.7.0 JP"),
    user_name: "TheWWRNerdGuy",
    // suppress_gauntlet_magnification: true,
    suppress_gauntlet_magnification: false,
});
