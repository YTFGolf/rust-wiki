//! Deals with getting information about a certain version of the game.

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    path::{Path, PathBuf},
    sync::Mutex,
};
pub mod version_data;
use version_data::CacheableVersionData;

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

type VersionDataContents = Box<dyn Any + Send + Sync>;
#[derive(Debug)]
/// Represents a version of the game.
pub struct Version {
    /// Root location of game files (i.e. `{location()}/DataLocal/stage.csv`
    /// contains the energy cost of each EoC stage).
    location: PathBuf,
    /// Version's language.
    pub language: VersionLanguage,
    /// Represents the version's number.
    pub number: String,

    version_data: Mutex<RefCell<Vec<(TypeId, VersionDataContents)>>>,
}
impl Version {
    /// Create new Version object.
    pub fn new<P>(location: P, language: &str, number: String) -> Result<Self, InvalidLanguage>
    where
        PathBuf: From<P>,
    {
        Ok(Self {
            location: PathBuf::from(location),
            language: language.try_into()?,
            number,

            version_data: Mutex::from(RefCell::new(Vec::new())),
        })
    }

    /// Get full absolute file path of the version's game directory.
    pub fn get_file_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.location.join(path)
    }

    /// Automatically extract the language code from the directory name.
    ///
    /// Literally just checks the last word of the directory and returns that.
    pub fn get_lang(path: &str) -> Option<&str> {
        path.rsplit(" ").next()
    }

    /// Automatically extract version number from directory name.
    ///
    /// Literally just checks the first instance of a full word that only
    /// contains numbers and full stops.
    pub fn get_version_number(path: &str) -> Option<&str> {
        path.split_whitespace()
            .find(|&part| part.chars().all(|c| c.is_ascii_digit() || c == '.'))
    }

    /// Get a cached data object.
    ///
    /// ## Usage
    /// ```rust,no_run
    /// use rust_wiki::data::map::map_option::MapOption;
    /// # use rust_wiki::data::version::Version;
    ///
    /// let version = Version::new("~", "en", "1.0".to_string()).unwrap();
    /// let map_option = version.get_cached_file::<MapOption>();
    /// let earthshaker_option = map_option.get_map(0);
    /// ```
    /// This can be run with any type that implements [CacheableVersionData].
    pub fn get_cached_file<T: CacheableVersionData + 'static>(&self) -> &T {
        let type_id = TypeId::of::<T>();

        let version_data = self.version_data.lock().unwrap();

        if let Some(position) = version_data
            .borrow()
            .iter()
            .position(|(id, _)| *id == type_id)
        {
            let data_vec = unsafe { &(*version_data.as_ptr()) };
            return data_vec[position]
                .1
                .downcast_ref::<T>()
                .expect("Something went horribly wrong.");
        }

        let new_value: VersionDataContents = Box::new(T::init_data(&self.location));
        version_data.borrow_mut().push((type_id, new_value));

        if let Some(position) = version_data
            .borrow()
            .iter()
            .position(|(id, _)| *id == type_id)
        {
            let data_vec = unsafe { &(*version_data.as_ptr()) };
            return data_vec[position]
                .1
                .downcast_ref::<T>()
                .expect("Something went horribly wrong.");
        }

        unreachable!()
    }
}
