//! Defines a trait to allow version data to be cached.

use crate::game_data::version::Version;
use std::{fmt::Debug, path::Path};

/// Represents a cacheable version data object.
///
/// Use this trait for large files that get repeatedly used, such as
/// `Map_option.csv`.
pub trait CacheableVersionData: Debug + Send + Sync {
    /// Initialises the version data.
    fn init_data(path: &Path) -> Self
    where
        Self: Sized;

    /// Initialise the version data, using the version object itself.
    ///
    /// ___Only___ implement this if `init_data` is impossible to use, for
    /// example if you are using a file from `resLocal` which has the language
    /// at the end of the file name. Default implementation just calls
    /// `init_data` with the version's directory location.
    fn init_data_with_version(version: &Version) -> Self
    where
        Self: Sized,
    {
        Self::init_data(&version.location())
    }
}

// possible alternative is to make all things have to return a result, which at
// least gives version a chance to drop the mutex before panicking, or
// realistically would be `unwrap_or_default`ing
