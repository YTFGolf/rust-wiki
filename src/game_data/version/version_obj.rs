//! Deals with getting information about a certain version of the game.

use super::{lang::VersionLanguage, version_data::CacheableVersionData};
use std::{
    any::{Any, TypeId},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Mutex,
};

/// Holds any [`CacheableVersionData`] object.
type VersionDataContents = Pin<Box<dyn Any + Send + Sync>>;
#[derive(Debug)]
/// Represents a version of the game.
pub struct Version {
    /// Root location of game files (i.e. `location.join("DataLocal/stage.csv")`
    /// contains the energy cost of each EoC stage).
    location: PathBuf,

    language: VersionLanguage,
    /// E.g. `"14.0"`.
    _number: String,

    /// Contains cached data so large files don't have to be parsed repeatedly.
    version_data: Mutex<Vec<(TypeId, VersionDataContents)>>,
}
impl Version {
    /// Create new Version object.
    pub fn new<P>(location: P, language: VersionLanguage, number: Option<String>) -> Self
    where
        PathBuf: From<P>,
    {
        Self {
            location: PathBuf::from(location),
            language,
            _number: number.unwrap_or_default(),

            version_data: Mutex::from(Vec::new()),
            // TODO combine with Lang from VersionConfig, and add number to the
            // cli
        }
    }
}

impl Version {
    /// Get version's directory location.
    pub fn location(&self) -> &Path {
        &self.location
    }

    /// Get version's language.
    pub fn language(&self) -> &VersionLanguage {
        &self.language
    }

    /// Get version's number.
    pub fn number(&self) -> &str {
        let loc = self.location.to_str().unwrap();
        loc.split_whitespace()
            .find(|&part| part.chars().all(|c| c.is_ascii_digit() || c == '.'))
            .unwrap()
    }

    /// Get version's number, in the same format as unitbuy.
    /// ```
    /// # use rust_wiki::game_data::version::Version;
    /// # use rust_wiki::game_data::version::lang::VersionLanguage;
    /// let version = Version::new("~/Version 12.3.4", VersionLanguage::EN, None);
    /// assert_eq!(version.number_u32(), Some(120304));
    /// let version = Version::new("~/Version 15.0.0", VersionLanguage::EN, None);
    /// assert_eq!(version.number_u32(), Some(150000));
    /// let version = Version::new("~/Version 15.0", VersionLanguage::EN, None);
    /// assert_eq!(version.number_u32(), Some(150000));
    /// let version = Version::new("~/Version 15", VersionLanguage::EN, None);
    /// assert_eq!(version.number_u32(), Some(150000));
    /// ```
    pub fn number_u32(&self) -> Option<u32> {
        const REASON: &str = "`Self::number` assures this is only ASCII digits";
        let mut num = self.number().split('.');
        let major: u32 = num.next()?.parse().expect(REASON);
        let minor: u32 = num.next().unwrap_or("0").parse().expect(REASON);
        let patch: u32 = num.next().unwrap_or("0").parse().expect(REASON);

        Some(major * 10_000 + minor * 100 + patch)
    }

    /// Get full absolute file path of the version's game directory.
    pub fn get_file_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.location.join(path)
    }

    /// Get a cached data object.
    ///
    /// ## Usage
    /// ```rust,no_run
    /// use rust_wiki::game_data::map::cached::map_option::MapOption;
    /// # use rust_wiki::game_data::version::Version;
    /// # use rust_wiki::game_data::version::lang::VersionLanguage;
    /// # use rust_wiki::game_data::meta::stage::map_id::MapID;
    ///
    /// let version = Version::new("~", VersionLanguage::EN, Some("1.0".into()));
    /// let map_option = version.get_cached_file::<MapOption>();
    /// let earthshaker_option = map_option.get_map(&MapID::from_numbers(0, 0));
    /// ```
    /// This can be run with any type that implements [`CacheableVersionData`].
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
                .expect("Error when casting in `Version::get_cached_file`.");
        }

        let new_value: VersionDataContents = Box::pin(T::init_data_with_version(self));
        version_data_lock.push((type_id, new_value));

        if let Some(position) = version_data_lock.iter().position(|(id, _)| *id == type_id) {
            let version_data_ptr = version_data_lock.as_ptr();
            let file_data = unsafe { &*(version_data_ptr.add(position)) };
            return file_data
                .1
                .downcast_ref::<T>()
                .expect("Error when casting in `Version::get_cached_file`.");
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
            struct itself would get reallocated. The struct is also pinned, so
            any possible reallocation would result in a compiler error probably.
        */

        unreachable!()
    }
}
