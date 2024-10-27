//! Deals with the user config.

use super::parse::input;
use crate::{config::set_config_file, data::version::Version};
use clap::Args;
use serde::{Deserialize, Serialize};

// TODO replace toml with toml_edit since I don't want this to just be terrible
// and non-documented.
// See https://github.com/toml-rs/toml/issues/376
/// TOML-based representation of game version.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserVersion {
    /// Root directory of decrypted files.
    pub path: String,
    /// Version's language.
    pub lang: Option<String>,
    /// Version number.
    pub number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// TOML-based representation of user's config file.
///
/// If this gets updated then [UserConfigCli] also needs to be updated.
pub struct UserConfig {
    /// Current version.
    pub version: UserVersion,
    /// Your wiki username.
    pub username: String,
    /// Do you put `|0` in the Magnification template instead of the actual
    /// magnification for gauntlets?
    pub suppress_gauntlet_magnification: bool,
}

#[derive(Debug, Args, PartialEq)]
/// User config options.
pub struct UserConfigCli {
    #[arg(short, long)]
    /// Root directory of encrypted files.
    pub path: Option<String>,

    #[arg(short = 'n', long)]
    /// Your wiki username.
    pub username: Option<String>,

    #[arg(long)]
    /// Do you put `|0` in the Magnification template instead of the actual
    /// magnification for gauntlets?
    pub suppress: Option<bool>,
}

/// Create the [UserVersion] object from a given path.
fn create_config_version(path: String) -> UserVersion {
    let lang = match Version::get_lang(&path) {
        None => Some(input("Enter language: ")),
        Some(language) => {
            let prompt = format!("Enter language (default: {language}): ");
            let l = input(&prompt);
            if l.is_empty() {
                None
            } else {
                Some(l)
            }
        }
    };

    let version_number = match Version::get_version_number(&path) {
        None => Some(input("Enter version number: ")),
        Some(number) => {
            let prompt = format!("Enter version number (default: {number}): ");
            let n = input(&prompt);
            if n.is_empty() {
                None
            } else {
                Some(n)
            }
        }
    };

    UserVersion {
        path,
        lang,
        number: version_number,
    }
}

/// Create user config file. If a config arg is not provided then it is provided
/// by input.
pub fn create_config(args: UserConfigCli) {
    let path = match args.path {
        Some(p) => p,
        None => input("Enter root directory of decrypted files: "),
    };
    let version = create_config_version(path);

    let name = match args.username {
        Some(name) => name,
        None => input("Enter wiki username: "),
    };

    let suppress = args.suppress.unwrap_or(false);

    let user_config = UserConfig {
        version,
        username: name,
        suppress_gauntlet_magnification: suppress,
    };

    let toml_repr = toml::to_string(&user_config).unwrap();
    set_config_file(&toml_repr);
}
