//! Defines a version's language.

use super::Version;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
#[error("invalid language: {0:?}")]
/// Represents an invalid language code.
pub struct InvalidLanguage(pub String);

/// Default language.
#[derive(Debug, Default, Clone, Copy)]
#[repr(usize)]
pub enum VersionLanguage {
    /// English.
    EN,
    /// Japanese.
    #[default]
    JP,
    // should this be JA?
}

impl Display for VersionLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lang = match self {
            Self::EN => "en",
            Self::JP => "ja",
        };
        f.write_str(lang)
    }
}

impl FromStr for VersionLanguage {
    type Err = InvalidLanguage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        match s_lower.as_str() {
            "en" => Ok(Self::EN),
            "jp" | "ja" => Ok(Self::JP),
            _ => Err(InvalidLanguage(s_lower)),
        }
    }
}

/// Struct that can hold game data for multiple languages.
pub trait MultiLangVersionContainer {
    /// Get a "default" version.
    fn lang_default(&self) -> &Version;
    /// Get the version that corresponds to the appropriate language.
    fn get_lang(&self, lang: VersionLanguage) -> &Version;
}
