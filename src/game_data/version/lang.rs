use super::Version;
use crate::SLang;

#[derive(Debug)]
/// Represents an invalid language code.
pub struct InvalidLanguage(pub String);

#[derive(Debug, Clone, Copy)]
/// Version's language.
pub enum VersionLanguage {
    /// English.
    EN,
    /// Japanese.
    JP,
}
use VersionLanguage as V;
// TODO merge with VersionConfig's lang.

impl TryFrom<&str> for VersionLanguage {
    type Error = InvalidLanguage;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val_lower = value.to_lowercase();
        match val_lower.as_str() {
            "en" => Ok(V::EN),
            "jp" | "ja" => Ok(V::JP),
            _ => Err(InvalidLanguage(val_lower)),
        }
    }
}
impl From<SLang> for VersionLanguage {
    fn from(value: SLang) -> Self {
        match value {
            SLang::EN => Self::EN,
            SLang::JP => Self::JP,
        }
    }
}
impl From<VersionLanguage> for SLang {
    fn from(value: VersionLanguage) -> Self {
        match value {
            VersionLanguage::EN => Self::EN,
            VersionLanguage::JP => Self::JP,
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
