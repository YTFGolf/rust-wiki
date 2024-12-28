//! Deals with getting information about a certain version of the game.

use std::{
    any::{Any, TypeId},
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

    version_data: Mutex<Vec<(TypeId, VersionDataContents)>>,
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

            version_data: Mutex::from(Vec::new()),
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

        let mut version_data = self.version_data.lock().unwrap();

        if let Some(position) = version_data.iter().position(|(id, _)| *id == type_id) {
            let version_data_ptr = version_data.as_ptr();
            // Pointer to underlying vec. Allows the mutex to go out of scope
            // while the pointer still points to valid memory.
            drop(version_data);
            // Note that it still compiles even with the drop. All the drop does
            // is release the mutex lock.
            let file_data = unsafe { &*(version_data_ptr.add(position)) };
            // Immutable reference to version_data[position].
            // All this might seem a bit pointless but it means that the
            // function can just return a &T rather than a MutexGuard or
            // something, so calling code can be much simpler.
            return file_data
                .1
                .downcast_ref::<T>()
                .expect("Something went horribly wrong.");
        }

        let new_value: VersionDataContents = Box::new(T::init_data(&self.location));
        version_data.push((type_id, new_value));

        if let Some(position) = version_data.iter().position(|(id, _)| *id == type_id) {
            let version_data_ptr = version_data.as_ptr();
            drop(version_data);
            let file_data = unsafe { &*(version_data_ptr.add(position)) };
            return file_data
                .1
                .downcast_ref::<T>()
                .expect("Something went horribly wrong.");
        }

        // Note I don't really know much about safety, this is a guess.
        // Safety: it's just pointers. If the unsafe pointers were mutable then
        // there would probably be problems, but they aren't so there aren't.

        // This is inside a mutex, so no data races or anything.

        // If needs to update the vec, then it can do that easily since nothing
        // else will be reading from or writing to the vec due to Mutex.

        // When reading, it just gets a pointer to the vec and then does normal
        // pointer arithmetic to get to the appropriate position. All the unsafe
        // bit does is allow me to drop the mutex while still having a pointer
        // to valid data. The reference is immutable, so assuming that nothing
        // weird happens when accessing immutable data, then even if multiple
        // things were to access it at the same time nothing bad happens.

        // Assuming `file_data` is a valid reference for the reasons above, then
        // all that needs to be done is getting the boxed data and converting it
        // from Any to T. That bit appears to work fine and I can't be bothered
        // to audit it.

        unreachable!()
    }
}
