//! Defines a trait to allow version data to be cached.

use crate::game_data::version::Version;
use std::{fmt::Debug, path::Path};

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
    pub err: Box<dyn Debug>,
}
impl<T: CacheableVersionData> CvdCreateError<T> {
    /// Create throw handler from given error.
    pub fn throw_from_err<E: Debug + 'static>(e: E) -> Self {
        // needs static because dyn types need the vtables in memory throughout
        // the program
        Self {
            handler: CvdCreateHandler::Throw,
            err: Box::new(e),
        }
    }
}
impl<T: CacheableVersionData + Default> CvdCreateError<T> {
    /// Create default handler from given error.
    pub fn default_from_err<E: Debug + 'static>(e: E) -> Self {
        Self {
            handler: CvdCreateHandler::Default(T::default()),
            err: Box::new(e),
        }
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
        Self: Sized,
    {
        log::warn!(
            "using deprecated default `CacheableVersionData::create` impl for {s}",
            s = std::any::type_name::<Self>()
        );
        Ok(Self::init_data_with_version(version))
    }

    /// Initialises the version data.
    #[deprecated]
    fn init_data(_path: &Path) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

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
        Self::init_data(version.location())
    }
}

// possible alternative is to make all things have to return a result, which at
// least gives version a chance to drop the mutex before panicking, or
// realistically would be `unwrap_or_default`ing
