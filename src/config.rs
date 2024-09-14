//! Contains global config values.
use std::{path::PathBuf, sync::LazyLock};

use home::home_dir;

fn expand_home(dir: &str) -> PathBuf {
    if &dir[0..2] == "~/" {
        home_dir().unwrap().join(&dir[2..])
    } else {
        PathBuf::from(dir)
    }
}

/// Configuration values for the program.
#[derive(Debug)]
pub struct Config {
    /// Root location of game files (i.e. `{data_mines}/DataLocal/stage.csv`
    /// contains the energy cost of each EoC stage).
    pub data_mines: PathBuf,
}

/// Static variable representing the config.
// TODO read a config file instead of writing it here.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    data_mines: expand_home("~/Downloads/Version 13.6.0 EN"),
});
