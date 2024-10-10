//! Deals with getting information about a certain version of the game.

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    path::PathBuf,
};
pub mod version_data;
use version_data::VersionData;

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

    // pub fn get_file()
    pub fn get_cached_file<T: VersionData + 'static>(&self) -> &T {
        let type_id = TypeId::of::<T>();

        let mut version_data = self.version_data.borrow_mut();

        if let Some(position) = version_data.iter().position(|(id, _)| *id == type_id) {
            let boxed = &self.version_data[position].1;
            boxed
                .downcast_ref::<T>()
                .expect("Failed to downcast to the requested type");
        }

        // If not found, initialize the type and store it
        let new_value: VersionDataContents = Box::new(T::init_data(&self.location));
        version_data.push((type_id, new_value));

        // Return the newly inserted value
        self.get_cached_file()
    }
}

// get language code to use in language files.
// also have an auto language option that looks at file name from trailing space
