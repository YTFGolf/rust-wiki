//! Contains functions to read data files.
use crate::config::CONFIG;
use std::{fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf, sync::LazyLock};

static WIKI_DATA_LOCATION: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::current_dir().unwrap().join("/data"));

/// Describes a location for files.
pub enum FileLocation {
    /// Root directory of game data.
    GameData,
    /// Root directory of downloaded wiki data.
    WikiData,
}

/// Get the root directory of a location.
/// ```
/// # use rust_wiki::file_handler::{FileLocation, get_file_location};
/// # use rust_wiki::config::CONFIG;
/// assert_eq!(get_file_location(FileLocation::GameData), &CONFIG.data_mines);
/// assert_eq!(get_file_location(FileLocation::WikiData), &std::env::current_dir().unwrap().join("/data"));
/// ```
#[inline]
pub fn get_file_location(location: FileLocation) -> &'static PathBuf {
    match location {
        FileLocation::GameData => &CONFIG.data_mines,
        FileLocation::WikiData => &WIKI_DATA_LOCATION,
    }
}

/// temp function
pub fn do_stuff() {
    let file_name = "DataLocal/stage.csv";
    let content = fs::read_to_string(get_file_location(FileLocation::GameData).join(file_name))
        .expect("File name no existo!");
    println!("{content:?}");

    read_file_lines(get_file_location(FileLocation::GameData).join(file_name))
}

/// Also temp function kinda
pub fn read_file_lines(p: PathBuf) {
    let process = |line: &str| println!("{:?}", line.split(",").collect::<Vec<_>>());
    let mut lock = BufReader::new(File::open(p).unwrap());
    let mut line = String::new();
    while lock.read_line(&mut line).unwrap() != 0 {
        process(&line);
        line.clear();
    };
}
