//! Defines a trait to allow version data to be cached.

use std::fmt::Debug;
use std::path::Path;

/// Represents a cacheable version data object.
pub trait CacheableVersionData: Debug + Send + Sync {
    /// Initialises the version data.
    fn init_data(path: &Path) -> Self
    where
        Self: Sized;
}
