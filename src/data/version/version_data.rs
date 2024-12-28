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

/*
I don't think this is necessary but I want it in git history.
///
/// Safety: This trait is unsafe due to implementation details of
/// [Version][super::Version] requiring that any struct implementing this trait
/// is immutable.
// pub unsafe trait CacheableVersionData: Debug + Send + Sync {
*/
