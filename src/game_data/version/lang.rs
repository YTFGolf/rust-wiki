//! Defines a version's language.

use super::Version;
use std::{fmt::Display, str::FromStr};
use strum::EnumIter;

#[derive(Debug, thiserror::Error)]
#[error("invalid language: {0:?}")]
/// Represents an invalid language code.
pub struct InvalidLanguage(pub String);

/// Version's language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
#[repr(usize)]
pub enum VersionLanguage {
    /// English.
    EN,
    /// Japanese.
    JP,
    // should this be JA?
    /// Korean.
    KR,
    /// Taiwanese.
    TW,
    /// Fallback data.
    Fallback,
}

impl Display for VersionLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lang = match self {
            Self::EN => "en",
            Self::JP => "ja",
            Self::KR => "ko",
            Self::TW => "tw",
            Self::Fallback => "fallback",
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
            "kr" => Ok(Self::KR),
            "tw" => Ok(Self::TW),
            "fallback" => Ok(Self::Fallback),
            _ => Err(InvalidLanguage(s_lower)),
        }
    }
}

/// Total amount of languages available.
pub const TOTAL_LANGS: usize = 5;

/// Type that can hold data for all available languages.
pub type MultiLangContainer<T> = [T; TOTAL_LANGS];

/// Struct that can hold game data for multiple languages.
pub trait MultiLangVersionContainer {
    /// Get a "default" version.
    fn lang_default(&self) -> &Version;
    /// Get the version that corresponds to the appropriate language.
    fn get_lang(&self, lang: VersionLanguage) -> &Version;
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_total_langs() {
        let count = VersionLanguage::iter().map(|l| l as usize).max().unwrap();
        assert_eq!(count + 1, TOTAL_LANGS);
    }
}
