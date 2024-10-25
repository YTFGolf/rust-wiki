//! Contains functions to read data files.
use std::{
    io::{BufRead, Cursor},
    path::PathBuf,
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

/// Strip comments from file reader.
/// ```
/// # use std::{fs::File, io::BufReader, path::PathBuf};
/// # use rust_wiki::config::get_config;
/// # use rust_wiki::file_handler::decomment_file_reader;
/// # let version = &get_config().unwrap().current_version;
/// let reader = File::open(version.get_file_path("DataLocal/stage.csv")).unwrap();
/// let reader = decomment_file_reader(BufReader::new(reader));
/// let mut rdr = csv::Reader::from_reader(reader);
/// ```
pub fn decomment_file_reader<R: BufRead>(reader: R) -> Cursor<String> {
    let f = reader
        .lines()
        .map(|line| line.unwrap().split("//").next().unwrap().trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    Cursor::new(f)
}
