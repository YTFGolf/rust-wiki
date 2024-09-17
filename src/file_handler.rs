//! Contains functions to read data files.
use crate::{config::CONFIG, stage::stage_data::Stage};
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
    GameData,
    /// Root directory of downloaded wiki data.
    WikiData,
}
use FileLocation::{GameData, WikiData};

/// Get the root directory of a location.
/// ```
/// # use rust_wiki::file_handler::{FileLocation, get_file_location};
/// # use rust_wiki::config::CONFIG;
/// assert_eq!(get_file_location(FileLocation::GameData), &CONFIG.data_mines);
/// assert_eq!(get_file_location(FileLocation::WikiData), &std::env::current_dir().unwrap().join("data"));
/// ```
#[inline]
pub fn get_file_location(location: FileLocation) -> &'static PathBuf {
    match location {
        GameData => &CONFIG.data_mines,
        WikiData => &WIKI_DATA_LOCATION,
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
    let gd = get_file_location(GameData);
    let f = BufReader::new(File::open(gd.join(path))?)
        .lines()
        .map(|line| line.unwrap().split("//").next().unwrap().trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(Cursor::new(f))
}

/// temp function
pub fn do_stuff() {
    Stage::new("stageRN000_00.csv");
    Stage::new("stageRND000_00.csv");
    Stage::new("stageRV006_19.csv");
    Stage::new("stageRQ000_09.csv");
    Stage::new("stageRC128_00.csv");
    Stage::new("stageRS250_00.csv");
    Stage::new("stageRS155_00.csv");
    Stage::new("stageEX000_00.csv");
    Stage::new("stageL000_18.csv");
    Stage::new("stageW04_08.csv");
    // Germany (Into the Future)
    Stage::new("stage00.csv");
    // read_csv_file("DataLocal/stage.csv");
}
