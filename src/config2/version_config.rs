//! Deals with the config for version.

use std::path::PathBuf;

use crate::data::version::Version;
use home::home_dir;
use serde::{Deserialize, Serialize};

/// Default language.
#[allow(missing_docs)]
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    EN,
    #[default]
    JP,
}

const TOTAL_VERSIONS: usize = 2;
#[derive(Debug, Default, Deserialize, Serialize)]
/// Configuration for Versions.
///
/// Make sure to initialise the config before it is read.
/// ```
/// # use rust_wiki::config2::version_config::VersionConfig;
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
    #[serde(skip)]
    cur_index: usize,
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
            let ind = lang.clone() as usize;
            self.versions[ind] = Some(Version::new2(location, lang, None));
        }

        self.cur_index = self.lang.clone() as usize;
    }

    /// Try to get current version.
    pub fn try_current_version(&self) -> Option<&Version> {
        self.versions[self.cur_index].as_ref()
    }

    /// Get current game version.
    pub fn current_version(&self) -> &Version {
        self.try_current_version().unwrap()
    }
}

impl VersionConfig {
    /// Set the version's `lang`.
    pub fn set_lang(&mut self, lang: Lang) {
        self.lang = lang;
    }
    /// Set the version's `enpath`.
    pub fn set_enpath(&mut self, enpath: String) {
        self.enpath = enpath;
    }
    /// Set the version's `jppath`.
    pub fn set_jppath(&mut self, jppath: String) {
        self.jppath = jppath;
    }
}
