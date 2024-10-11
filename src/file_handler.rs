//! Contains functions to read data files.
use crate::config::CONFIG;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor},
    path::{Path, PathBuf},
    sync::LazyLock,
};

static WIKI_DATA_LOCATION: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::current_dir().unwrap().join("data"));

/// Describes a location for files.
pub enum FileLocation {
    /// Root directory of game data.
    // GameData,
    /// Root directory of downloaded wiki data.
    WikiData,
}
use FileLocation as F;

/// Get the root directory of a location.
/// ```
/// # use rust_wiki::file_handler::{FileLocation, get_file_location};
/// assert_eq!(get_file_location(FileLocation::WikiData), &std::env::current_dir().unwrap().join("data"));
/// ```
#[inline]
pub fn get_file_location(location: FileLocation) -> &'static PathBuf {
    match location {
        F::WikiData => &WIKI_DATA_LOCATION,
    }
}

/// Get game data file, stripped of comments.
/// ```
/// # use rust_wiki::file_handler::get_decommented_file_reader;
/// # use std::path::PathBuf;
/// let reader = get_decommented_file_reader("DataLocal/stage.csv").unwrap();
/// let mut rdr = csv::Reader::from_reader(reader);
/// let reader = get_decommented_file_reader(PathBuf::from("DataLocal").join("stage.csv")).unwrap();
/// let mut rdr = csv::Reader::from_reader(reader);
/// ```
pub fn get_decommented_file_reader<P: AsRef<Path>>(path: P) -> Result<Cursor<String>, io::Error> {
    let gd = &CONFIG.current_version.location;
    // TODO just accept path and let user version take care of getting full
    // path.
    let f = BufReader::new(File::open(gd.join(path))?)
        .lines()
        .map(|line| line.unwrap().split("//").next().unwrap().trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(Cursor::new(f))
}
