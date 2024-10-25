//! Contains global config values.
use crate::data::version::{InvalidLanguage, Version};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::PathBuf;

/// Expand home directory.
fn expand_home(dir: &str) -> PathBuf {
    if dir == "~" || dir.is_empty() {
        home_dir().unwrap()
    } else if dir.len() >= 2 && &dir[0..2] == "~/" {
        home_dir().unwrap().join(&dir[2..])
    } else {
        PathBuf::from(dir)
    }
}

// TODO replace toml with toml_edit since I don't want this to just be terrible
// and non-documented.
// See https://github.com/toml-rs/toml/issues/376
/// TOML-based representation of game version.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserVersion {
    path: String,
    lang: Option<String>,
    number: Option<String>,
}
impl From<UserVersion> for Version {
    fn from(value: UserVersion) -> Self {
        let path = expand_home(&value.path);

        let lang = match &value.lang {
            None => {
                Version::get_lang(&value.path).expect("No language name found in directory name!")
            }
            Some(lang) => lang,
        };

        let number = match &value.number {
            None => Version::get_version_number(&value.path)
                .expect("No version number found in directory name!"),
            Some(n) => n,
        };

        match Version::new(path, lang, number.to_string()) {
            Ok(v) => v,
            Err(InvalidLanguage(code)) => panic!("Version language not recognised: {code:?}."),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// TOML-based representation of user's config file.
///
/// If this gets updated then [UserConfigCli][crate::cli::UserConfigCli] also
/// needs to be updated.
// TODO something different so that this gets stored in the same place as
// UserConfigCli
pub struct UserConfig {
    version: UserVersion,
    username: String,
    suppress_gauntlet_magnification: bool,
}

/// Configuration values for the program.
// Don't update this without updating main
// version should be a full object with path, language and version number. Leave
// the latter 2 blank if you want path to infer them
#[derive(Debug)]
pub struct Config {
    /// Current game version.
    pub current_version: Version,
    /// Your name.
    pub user_name: String,
    /// Make `Magnification` put `|name|0` on gauntlet pages rather than the
    /// enemy's actual magnification.
    pub suppress_gauntlet_magnification: bool,
}
impl From<UserConfig> for Config {
    fn from(value: UserConfig) -> Self {
        Self {
            current_version: value.version.into(),
            user_name: value.username,
            suppress_gauntlet_magnification: value.suppress_gauntlet_magnification,
        }
    }
}

const CONFIG_FILE: &str = "user-config.toml";
fn read_config_file() -> Option<String> {
    let f = File::open(CONFIG_FILE);
    match f {
        Ok(mut f) => {
            let mut buf = String::from("");
            f.read_to_string(&mut buf).unwrap();
            Some(buf)
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => None,
            _ => panic!("Error when trying to open {CONFIG_FILE}: {e}"),
        },
    }
}

/// Read user config file and return parsed config object, if the file exists.
fn get_user_config() -> Option<UserConfig> {
    let config: UserConfig = toml::from_str(&read_config_file()?).unwrap();
    Some(config)
}

/// Read user config file and parse it into a [Config] object.
pub fn get_config() -> Option<Config> {
    Some(get_user_config()?.into())
}

/// Static variable representing the config, for use in tests.
#[cfg(test)]
pub static CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| get_config().unwrap());

// /// Do toml stuff.
// fn do_toml_stuff() {
//     println!("{:?}", get_config());
//     if true {
//         return;
//     }
//     let c = UserConfig {
//         version: UserVersion {
//             path: "~/Downloads/Version 13.7.0 EN".to_string(),
//             lang: None,
//             number: None,
//         },
//         username: "TheWWRNerdGuy".to_string(),
//         suppress_gauntlet_magnification: false,
//     };
//     println!("{c:?}");

//     let c2 = toml::to_string(&c).unwrap();
//     println!("{c2}");

//     let c3: UserConfig = toml::from_str(&c2).unwrap();
//     println!("{c3:?}");

//     let c4 = Version::from(c3.version);
//     let _d = c4.get_cached_file::<StageOption>();
//     println!("{:?}", c4);
//     // println!("{:?}", _d);

//     exit(0);
// }

// // TODO test with just home directory as version
