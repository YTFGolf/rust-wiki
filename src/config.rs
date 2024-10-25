//! Contains global config values.
use crate::data::stage::raw::stage_option::StageOption;
use crate::data::version::{InvalidLanguage, Version};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::{path::PathBuf, sync::LazyLock};

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

fn get_version(dir: &str) -> Version {
    match Version::new(
        expand_home(dir),
        Version::get_lang(dir).expect("No language name found in directory name!"),
        Version::get_version_number(dir)
            .expect("No version number found in directory name!")
            .to_string(),
    ) {
        Ok(v) => v,
        Err(InvalidLanguage(code)) => panic!("Version language not recognised: {code:?}."),
    }
}

// TODO replace toml with toml_edit since I don't want this to just be terrible
// and non-documented.
// See https://github.com/toml-rs/toml/issues/376
#[derive(Debug, Serialize, Deserialize)]
struct UserVersion {
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
struct UserConfig {
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

/// Static variable representing the config.
// TODO read a config file instead of writing it here.
// TODO remove dependency on static variable.
// #[cfg(test)]
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    current_version: get_version("~/Downloads/Version 13.7.0 EN"),
    user_name: "TheWWRNerdGuy".to_string(),
    // suppress_gauntlet_magnification: true,
    suppress_gauntlet_magnification: false,
});

/// Do toml stuff.
pub fn do_toml_stuff() {
    let c = UserConfig {
        version: UserVersion {
            path: "~/Downloads/Version 13.7.0 EN".to_string(),
            lang: None,
            number: None,
        },
        username: "aa".to_string(),
        suppress_gauntlet_magnification: false,
    };
    println!("{c:?}");

    let c2 = toml::to_string(&c).unwrap();
    println!("{c2}");

    let c3: UserConfig = toml::from_str(&c2).unwrap();
    println!("{c3:?}");

    let c4 = Version::from(c3.version);
    let _d = c4.get_cached_file::<StageOption>();
    println!("{:?}", c4);
    // println!("{:?}", _d);

    exit(0);
}

// TODO test with just home directory as version
