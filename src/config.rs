//! Contains global config values.
use crate::data::version::{InvalidLanguage, Version};
use home::home_dir;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
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

// impl From<UserVersion> for Version {
//     fn from(value: UserVersion) -> Self {
//         let path = expand_home(&value.path);

//         let lang = match &value.lang {
//             None => {
//                 Version::get_lang(&value.path).expect("No language name found in directory name!")
//             }
//             Some(lang) => lang,
//         };

//         let number = match &value.number {
//             None => Version::get_version_number(&value.path)
//                 .expect("No version number found in directory name!"),
//             Some(n) => n,
//         };

//         match Version::new(path, lang, number.to_string()) {
//             Ok(v) => v,
//             Err(InvalidLanguage(code)) => panic!("Version language not recognised: {code:?}."),
//         }
//     }
// }

/// Configuration values for the program.
// Don't update this without updating main
// version should be a full object with path, language and version number. Leave
// the latter 2 blank if you want path to infer them
#[derive(Debug)]
pub struct Config {
    // TODO store all versions in config
    /// Current game version.
    current_version: Version,
    /// Your name.
    user_name: String,
    /// Make `Magnification` put `|name|0` on gauntlet pages rather than the
    /// enemy's actual magnification.
    suppress_gauntlet_magnification: bool,
}
// impl From<UserConfig> for Config {
//     fn from(value: UserConfig) -> Self {
//         Self {
//             current_version: value.version.into(),
//             user_name: value.username,
//             suppress_gauntlet_magnification: value.suppress_gauntlet_magnification,
//         }
//     }
// }
impl Config {
    /// Get current game version.
    pub fn current_version(&self) -> &Version {
        &self.current_version
    }
    /// Get username.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    /// Do you suppress gauntlet magnifications.
    pub fn suppress_gauntlet_magnification(&self) -> bool {
        self.suppress_gauntlet_magnification
    }
}

const CONFIG_FILE: &str = "user-config.toml";
/// Set config file to `new_value`.
pub fn set_config_file(new_value: &str) {
    let f = File::create(CONFIG_FILE);
    f.unwrap().write_all(new_value.as_bytes()).unwrap();
    println!("Successfully set config at {CONFIG_FILE}.");
}

fn read_config_file() -> Option<String> {
    let f = File::open(CONFIG_FILE);
    match f {
        Ok(mut f) => {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            Some(buf)
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => None,
            _ => panic!("Error when trying to open {CONFIG_FILE}: {e}"),
        },
    }
}

// /// Read user config file and return parsed config object, if the file exists.
// pub fn get_user_config() -> Option<UserConfig> {
//     // FIXME currently assumes that if config is None then the file doesn't
//     // exist, but could also be an error with toml parsing if toml doesn't
//     // contain every field.
//     let config: UserConfig = toml::from_str(&read_config_file()?).unwrap();
//     Some(config)
// }

// /// Read user config file and parse it into a [Config] object.
#[cfg(test)]
pub fn get_config() -> Option<Config> {
    todo!()
    // Some(get_user_config()?.into())
}

/// Static variable representing the config, for use in tests.
#[cfg(test)]
pub static DEFAULT_CONFIG: std::sync::LazyLock<Config> =
    std::sync::LazyLock::new(|| get_config().unwrap());

// TODO test with just home directory as version, make sure doesn't panic when
// initialising.
