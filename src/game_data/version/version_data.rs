//! Defines a trait to allow version data to be cached.

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
}

// possible alternative is to make all things have to return a result, which at
// least gives version a chance to drop the mutex before panicking, or
// realistically would be `unwrap_or_default`ing
