//! Contains functions to read data files.
use std::{path::PathBuf, sync::LazyLock};

static WIKI_DATA_LOCATION: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::current_dir().unwrap().join("data"));

/// Get root directory of wiki data.
pub fn get_wiki_data_location() -> &'static PathBuf {
    &WIKI_DATA_LOCATION
}
