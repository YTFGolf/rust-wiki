//! Contains functions to read data files.
use std::{path::PathBuf, sync::LazyLock};

static WIKI_DATA_LOCATION: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::current_dir().unwrap().join("data"));

/// Describes a location for files.
pub enum FileLocation {
    /// Root directory of game data.
    // GameData,
    /// Root directory of downloaded wiki data.
    WikiData,
}
use FileLocation as F;

/// Get the root directory of a location.
/// ```
/// # use rust_wiki::file_handler::{FileLocation, get_file_location};
/// assert_eq!(get_file_location(&FileLocation::WikiData), &std::env::current_dir().unwrap().join("data"));
/// ```
pub fn get_file_location(location: &FileLocation) -> &'static PathBuf {
    match location {
        F::WikiData => &WIKI_DATA_LOCATION,
    }
}
