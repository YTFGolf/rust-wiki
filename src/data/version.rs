//! Deals with getting information about a certain version of the game.

use std::path::PathBuf;

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

#[derive(Debug)]
/// Represents a version of the game.
pub struct Version {
    /// Root location of game files (i.e. `{data_mines}/DataLocal/stage.csv`
    /// contains the energy cost of each EoC stage).
    pub location: PathBuf,
    /// Version's language.
    pub language: VersionLanguage,
    // TODO static data store
}
impl Version {
    pub fn new<P>(location: P, language: &str) -> Result<Self, InvalidLanguage>
    where
        PathBuf: From<P>,
    {
        Ok(Self {
            location: PathBuf::from(location),
            language: language.try_into()?,
        })
    }

    /// Automatically extract the language code from the directory name.
    ///
    /// Literally just checks the last word of the directory and returns that.
    pub fn get_lang(path: &str) -> &str {
        path.rsplit(" ").next().unwrap()
    }
}

// get language code to use in language files.
// also have an auto language option that looks at file name from trailing space
