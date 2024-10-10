use std::{fmt::Debug, path::PathBuf};

pub trait VersionData: Debug + Send + Sync {
    fn init_data(path: &PathBuf) -> Self
    where
        Self: Sized;
}
