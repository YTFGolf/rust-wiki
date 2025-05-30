//! Deals with the config for version.

use crate::game_data::version::{lang::{self, MultiLangVersionContainer}, Version};
use serde::{Deserialize, Serialize};
use std::{env::home_dir, fmt::Display, path::PathBuf};

/// Default language.
#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[repr(usize)]
pub enum Lang {
    /// English.
    EN,
    #[default]
    /// Japanese.
    JP,
    // should this be JA?
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lang = match self {
            Self::EN => "en",
            Self::JP => "ja",
        };
        f.write_str(lang)?;

        Ok(())
    }
}

const TOTAL_VERSIONS: usize = 2;
#[derive(Debug, Default, Deserialize, Serialize)]
/// Configuration for Versions.
///
/// Make sure to initialise the config before it is read.
/// ```
/// # use rust_wiki::VersionConfig;
/// let mut new_vc = VersionConfig::default();
/// assert!(matches!(new_vc.try_current_version(), None));
/// new_vc.init_all();
/// assert!(matches!(new_vc.try_current_version(), Some(_)));
/// ```
pub struct VersionConfig {
    lang: Lang,
    enpath: String,
    jppath: String,

    #[serde(skip)]
    versions: [Option<Version>; TOTAL_VERSIONS],
}

impl Clone for VersionConfig {
    fn clone(&self) -> Self {
        Self {
            lang: self.lang,
            enpath: self.enpath.clone(),
            jppath: self.jppath.clone(),

            versions: Default::default(),
        }
    }
}

impl VersionConfig {
    fn expand_home(dir: &str) -> PathBuf {
        if dir == "~" || dir.is_empty() {
            home_dir().unwrap()
        } else if dir.len() >= 2 && &dir[0..2] == "~/" {
            home_dir().unwrap().join(&dir[2..])
        } else {
            PathBuf::from(dir)
        }
    }

    /// Initialise all versions.
    pub fn init_all(&mut self) {
        const LANGS: [Lang; TOTAL_VERSIONS] = [Lang::EN, Lang::JP];
        for lang in LANGS {
            let location = match lang {
                Lang::EN => &self.enpath,
                Lang::JP => &self.jppath,
            };
            let location = Self::expand_home(location);
            self.versions[lang as usize] = Some(Version::new(location, lang, None));
        }
    }

    /// Get configured language.
    pub fn lang(&self) -> Lang {
        self.lang
    }
}

impl VersionConfig {
    /// Try to get version.
    pub fn try_version(&self, lang: Lang) -> Option<&Version> {
        self.versions[lang as usize].as_ref()
    }

    /// Try to get current game version.
    pub fn try_current_version(&self) -> Option<&Version> {
        self.try_version(self.lang)
    }

    /// Get game version.
    pub fn version(&self, lang: Lang) -> &Version {
        self.try_version(lang)
            .expect("config has not been properly initialised")
    }

    /// Get current game version.
    pub fn current_version(&self) -> &Version {
        self.version(self.lang)
    }

    /// Get English version.
    pub fn en(&self) -> &Version {
        self.version(Lang::EN)
    }

    /// Get Japanese version.
    pub fn jp(&self) -> &Version {
        self.version(Lang::JP)
    }
}
impl MultiLangVersionContainer for VersionConfig {
    fn lang_default(&self) -> &Version {
        self.current_version()
    }

    fn get_lang(&self, lang: lang::VersionLanguage) -> &Version {
        self.version(lang.into())
    }
}

impl VersionConfig {
    /// Set the path of the current version to `path`. Must be called before
    /// [`init_all`][VersionConfig::init_all] or it will do nothing.
    pub fn set_current_path(&mut self, path: String) {
        match self.lang {
            Lang::EN => self.enpath = path,
            Lang::JP => self.jppath = path,
        }
    }

    /// Set the version's `lang`.
    pub fn set_lang(&mut self, lang: Lang) {
        self.lang = lang;
    }
}
