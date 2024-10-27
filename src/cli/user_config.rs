use clap::Args;
use serde::{Deserialize, Serialize};

// TODO replace toml with toml_edit since I don't want this to just be terrible
// and non-documented.
// See https://github.com/toml-rs/toml/issues/376
/// TOML-based representation of game version.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserVersion {
    pub path: String,
    pub lang: Option<String>,
    pub number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// TOML-based representation of user's config file.
///
/// If this gets updated then [UserConfigCli][crate::cli::UserConfigCli] also
/// needs to be updated.
// TODO something different so that this gets stored in the same place as
pub struct UserConfig {
    pub version: UserVersion,
    pub username: String,
    pub suppress_gauntlet_magnification: bool,
}

#[derive(Debug, Args, PartialEq)]
/// User config options.
pub struct UserConfigCli {
    #[arg(short, long)]
    pub path: Option<String>,

    #[arg(short = 'n', long)]
    pub username: Option<String>,

    #[arg(long)]
    pub suppress: Option<bool>,
}
