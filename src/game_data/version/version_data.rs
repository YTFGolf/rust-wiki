//! Defines a trait to allow version data to be cached.

use crate::game_data::version::Version;
use std::{error::Error, fmt::Debug};

#[derive(Debug)]
/// How a CVD error should be handled.
pub enum CvdCreateHandler<T: CacheableVersionData> {
    /// Log an error and return contained data as default.
    Default(T),
    /// Throw the error and panic.
    Throw,
}

#[derive(Debug)]
/// Error that occurred when creating CVD object.
pub struct CvdCreateError<T: CacheableVersionData> {
    /// How this error should be handled.
    pub handler: CvdCreateHandler<T>,
    /// What the error is.
    pub err: Box<dyn Error>,
}
impl<T: CacheableVersionData> CvdCreateError<T> {
    /// Create throw handler from given error.
    pub fn throw(err: Box<dyn Error>) -> Self {
        Self {
            handler: CvdCreateHandler::Throw,
            err,
        }
    }

    /// Create throw handler from static error.
    pub fn throw_from_err<E: Error + 'static>(e: E) -> Self {
        // needs static because dyn types need the vtables in memory throughout
        // the program
        Self::throw(Box::new(e))
    }
}
impl<T: CacheableVersionData + Default> CvdCreateError<T> {
    /// Create default handler from given error.
    pub fn as_default(err: Box<dyn Error>) -> Self {
        Self {
            handler: CvdCreateHandler::Default(T::default()),
            err,
        }
    }

    /// Create default handler from static error.
    pub fn default_from_err<E: Error + 'static>(e: E) -> Self {
        Self::as_default(Box::new(e))
    }
}

/// Shorthand for CVD result with duplicate types
pub type CvdResult<T> = Result<T, CvdCreateError<T>>;

/// Represents a cacheable version data object.
///
/// Use this trait for large files that get repeatedly used, such as
/// `Map_option.csv`.
pub trait CacheableVersionData: Debug + Send + Sync {
    /// Create the cacheable version data object.
    fn create(version: &Version) -> CvdResult<Self>
    where
        Self: Sized;
}
