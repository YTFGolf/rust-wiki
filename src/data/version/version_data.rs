use std::fmt::Debug;
use std::path::PathBuf;

/// Represents a cacheable version data object.
pub trait CacheableVersionData: Debug + Send + Sync {
    /// Initialises the version data.
    fn init_data(path: &PathBuf) -> Self
    where
        Self: Sized;
}
