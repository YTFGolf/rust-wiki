//! Deals with getting information about a certain version of the game.

use std::{
    any::{Any, TypeId},
    path::{Path, PathBuf},
    pin::Pin,
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

type VersionDataContents = Pin<Box<dyn Any + Send + Sync>>;
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
        path.rsplit(' ').next()
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

        let mut version_data_lock = self.version_data.lock().unwrap();

        if let Some(position) = version_data_lock.iter().position(|(id, _)| *id == type_id) {
            let version_data_ptr = version_data_lock.as_ptr();
            // Pointer to underlying vec. Allows the mutex to go out of scope
            // while the pointer still points to valid memory.
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

        let new_value: VersionDataContents = Box::pin(T::init_data(&self.location));
        version_data_lock.push((type_id, new_value));

        if let Some(position) = version_data_lock.iter().position(|(id, _)| *id == type_id) {
            let version_data_ptr = version_data_lock.as_ptr();
            let file_data = unsafe { &*(version_data_ptr.add(position)) };
            return file_data
                .1
                .downcast_ref::<T>()
                .expect("Something went horribly wrong.");
        }

        /*
        This might be safe. The following is just the ramblings of someone who
        doesn't truly understand how to make `unsafe` code thread-safe. At time
        of writing the code is single-threaded anyway, so safety is pretty much
        guaranteed.

        Safety invariants:
        - Modification is atomic
          - The only point where modification occurs is through the MutexGuard.
            This will always be atomic because that's what Mutexes do.
        - Underlying data is valid
          - As long as the underlying struct doesn't get dropped/reallocated it
            should be fine, even if the underlying data was slightly modified.
            Probably.
        - All pointers are always valid
          - The MutexGuard remains active until the function returns, so at the
            moment that it returns the pointer will be valid. The function
            returns a pointer to the struct of type T. If this struct never gets
            dropped/reallocated while the pointer is in use, the pointer will
            always be valid.
          - The struct will never be dropped because the Version owns the Box
            that owns the struct. Therefore, it will never get dropped until the
            Box is dropped, which won't happen until the Vec is dropped, which
            won't happen until ... etc. until you get to the Version object.
            Basically, the struct won't be dropped until version is dropped.
          - The function signature has an elided lifetime, i.e. the reference
            can only live as long as the version object, thus the data is never
            dropped while the pointer is in use.
          - No reallocation is kind of an assumption, but I don't know why the
            struct itself would get reallocated when it's pinned.
        */

        unreachable!()
    }
}
