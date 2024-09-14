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
    pub data_mines: PathBuf,
}

/// Static variable representing the config.
// TODO read a file instead of writing it here.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    data_mines: expand_home("~/Downloads/Version 13.6.0 EN"),
});
