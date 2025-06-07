//! Deals with the config for version.

use crate::game_data::version::{
    Version,
    lang::{self, MultiLangVersionContainer, VersionLanguage},
};
use serde::{
    Deserialize, Serialize,
    de::{self},
};
use std::{env::home_dir, path::PathBuf, str::FromStr};

pub fn deserialize_lang<'de, D>(deserializer: D) -> Result<VersionLanguage, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    FromStr::from_str(&s).map_err(de::Error::custom)
}
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_lang<S>(lang: &VersionLanguage, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.collect_str(lang)
}

const TOTAL_VERSIONS: usize = 2;
#[derive(Debug, Deserialize, Serialize)]
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
    #[serde(
        serialize_with = "serialize_lang",
        deserialize_with = "deserialize_lang"
    )]
    lang: VersionLanguage,
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

impl Default for VersionConfig {
    fn default() -> Self {
        Self {
            lang: VersionLanguage::JP,
            enpath: Default::default(),
            jppath: Default::default(),
            versions: Default::default(),
        }
    }
}

impl VersionConfig {
    /// So the other functions don't need to duplicate this switch statement.
    fn get_lang_path_ptr(&self, lang: VersionLanguage) -> *const String {
        let path = match lang {
            VersionLanguage::EN => &self.enpath,
            VersionLanguage::JP => &self.jppath,
        };
        path as *const String
    }

    /// Get the filepath of the decrypted contents for this language.
    fn get_lang_path_mut(&mut self, lang: VersionLanguage) -> &mut String {
        let path = self.get_lang_path_ptr(lang) as *mut String;
        unsafe { &mut *path }
        // safety: idk if this is undefined behaviour or not, rust doesn't do a
        // great job of defining undefined behaviour and unsafe really is not
        // ergonomic

        // tbf at least rust doesn't give me a warning about this one unlike if
        // I tried to directly convert the pointer from an `&T`

        // maybe I should just use a macro instead
    }

    /// [`Self::get_lang_path_mut`] but immutable.
    fn get_lang_path(&self, lang: VersionLanguage) -> &String {
        unsafe { &*self.get_lang_path_ptr(lang) }
        // safety: this has gotta be safe right
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
        const LANGS: [VersionLanguage; TOTAL_VERSIONS] = [VersionLanguage::EN, VersionLanguage::JP];
        for lang in LANGS {
            let location = Self::expand_home(self.get_lang_path(lang));
            self.versions[lang as usize] = Some(Version::new(location, lang, None));
        }
    }

    /// Get configured language.
    pub fn lang(&self) -> VersionLanguage {
        self.lang
    }
}

impl VersionConfig {
    /// Try to get version.
    pub fn try_version(&self, lang: VersionLanguage) -> Option<&Version> {
        self.versions[lang as usize].as_ref()
    }

    /// Try to get current game version.
    pub fn try_current_version(&self) -> Option<&Version> {
        self.try_version(self.lang)
    }

    /// Get game version.
    pub fn version(&self, lang: VersionLanguage) -> &Version {
        self.try_version(lang)
            .expect("config has not been properly initialised")
    }

    /// Get current game version.
    pub fn current_version(&self) -> &Version {
        self.version(self.lang)
    }

    /// Get English version.
    pub fn en(&self) -> &Version {
        self.version(VersionLanguage::EN)
    }

    /// Get Japanese version.
    pub fn jp(&self) -> &Version {
        self.version(VersionLanguage::JP)
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
        let p = self.get_lang_path_mut(self.lang);
        *p = path;
    }

    /// Set the version's `lang`.
    pub fn set_lang(&mut self, lang: VersionLanguage) {
        self.lang = lang;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_path() {
        let mut version = VersionConfig::default();
        assert_eq!(version.lang, VersionLanguage::JP);
        assert_eq!(version.get_lang_path(version.lang), "");

        version.set_current_path("###".to_string());
        assert_eq!(version.get_lang_path(version.lang), "###");
        // just to ensure that this works, since it relies on something that may
        // or may not be undefined behaviour idk
    }
}
