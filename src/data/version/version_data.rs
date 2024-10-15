//! Defines a trait to allow version data to be cached.

use std::fmt::Debug;
use std::path::Path;

/// Represents a cacheable version data object.
///
/// Use this trait for large files that get repeatedly used, such as
/// `Map_option.csv`.
pub trait CacheableVersionData: Debug + Send + Sync {
    /// Initialises the version data.
    fn init_data(path: &Path) -> Self;
}
