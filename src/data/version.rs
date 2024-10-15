//! Deals with getting information about a certain version of the game.

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    path::PathBuf,
};
pub mod version_data;
use version_data::CacheableVersionData;

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
    /// Root location of game files (i.e. `{data_mines}/DataLocal/stage.csv`
    /// contains the energy cost of each EoC stage).
    pub location: PathBuf,
    /// Version's language.
    pub language: VersionLanguage,
    // TODO version name param
    version_data: RefCell<Vec<(TypeId, VersionDataContents)>>,
    // TODO see if this can be stuck inside a Mutex so I can get rid of the sync
    // impl below (and make this actually thread-safe instead of lying about it)
}
unsafe impl Sync for Version {}
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

            version_data: RefCell::new(Vec::new()),
        })
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
    pub fn get_version_number(path: &str) -> Option<String> {
        path.split_whitespace()
            .find(|&part| part.chars().all(|c| c.is_digit(10) || c == '.'))
            .map(|s| s.to_string())
    }

    // pub fn get_file()

    /// Get a cached data object.
    pub fn get_cached_file<T: CacheableVersionData + 'static>(&self) -> &T {
        let type_id = TypeId::of::<T>();

        // let mut version_data = self.version_data.borrow_mut();

        if let Some(position) = self
            .version_data
            .borrow()
            .iter()
            .position(|(id, _)| *id == type_id)
        {
            let boxed = unsafe { &(*self.version_data.as_ptr()) };
            return boxed[position]
                .1
                .downcast_ref::<T>()
                .expect("Failed to downcast to the requested type");
        }

        // If not found, initialize the type and store it
        let new_value: VersionDataContents = Box::new(T::init_data(&self.location));
        self.version_data.borrow_mut().push((type_id, new_value));
        // unsafe {
        //     let s = self as *const Version as *mut Version;
        //     let version_data = &mut (*s).version_data;
        //     version_data.push();
        // }

        if let Some(position) = self
            .version_data
            .borrow()
            .iter()
            .position(|(id, _)| *id == type_id)
        {
            let boxed = unsafe { &(*self.version_data.as_ptr()) };
            return boxed[position]
                .1
                .downcast_ref::<T>()
                .expect("Failed to downcast to the requested type");
        }

        unreachable!()
    }
}

// get language code to use in language files.
// also have an auto language option that looks at file name from trailing space
