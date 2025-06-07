//! Deals with the config for version.

use crate::game_data::version::{
    Version,
    lang::{self, MultiLangContainer, MultiLangVersionContainer, VersionLanguage},
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
    krpath: String,
    twpath: String,

    #[serde(skip)]
    versions: MultiLangContainer<Option<Version>>,
}

impl Clone for VersionConfig {
    fn clone(&self) -> Self {
        Self {
            lang: self.lang,
            enpath: self.enpath.clone(),
            jppath: self.jppath.clone(),
            krpath: self.krpath.clone(),
            twpath: self.twpath.clone(),

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
            krpath: Default::default(),
            twpath: Default::default(),

            versions: Default::default(),
        }
    }
}

impl VersionConfig {
    /// Get the filepath of the decrypted contents for this language.
    fn get_lang_path_mut(&mut self, lang: VersionLanguage) -> &mut String {
        match lang {
            VersionLanguage::EN => &mut self.enpath,
            VersionLanguage::JP => &mut self.jppath,
            VersionLanguage::KR => &mut self.krpath,
            VersionLanguage::TW => &mut self.twpath,
        }
    }

    /// [`Self::get_lang_path_mut`] but immutable.
    fn get_lang_path(&self, lang: VersionLanguage) -> &String {
        match lang {
            VersionLanguage::EN => &self.enpath,
            VersionLanguage::JP => &self.jppath,
            VersionLanguage::KR => &self.krpath,
            VersionLanguage::TW => &self.twpath,
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
        const LANGS: MultiLangContainer<VersionLanguage> = [
            VersionLanguage::EN,
            VersionLanguage::JP,
            VersionLanguage::KR,
            VersionLanguage::TW,
        ];
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

    /// Get Korean version.
    pub fn kr(&self) -> &Version {
        self.version(VersionLanguage::KR)
    }

    /// Get Taiwanese version.
    pub fn tw(&self) -> &Version {
        self.version(VersionLanguage::TW)
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
