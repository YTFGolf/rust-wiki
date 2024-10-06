//! Contains global config values.
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

/// Configuration values for the program.
#[derive(Debug)]
pub struct Config<'a> {
    /// Root location of game files (i.e. `{data_mines}/DataLocal/stage.csv`
    /// contains the energy cost of each EoC stage).
    pub data_mines: PathBuf,
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
    data_mines: expand_home("~/Downloads/Version 13.7.0 JP"),
    user_name: "TheWWRNerdGuy",
    suppress_gauntlet_magnification: true,
});
