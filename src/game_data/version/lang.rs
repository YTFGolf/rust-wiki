use crate::SLang;

#[derive(Debug)]
/// Represents an invalid language code.
pub struct InvalidLanguage(pub String);

#[derive(Debug)]
/// Version's language.
pub enum VersionLanguage {
    /// English.
    EN,
    /// Japanese.
    JP,
}
use VersionLanguage as V;
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
