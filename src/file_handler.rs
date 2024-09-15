//! Contains functions to read data files.
use crate::config::CONFIG;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Cursor},
    path::PathBuf,
    sync::LazyLock,
};

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

/// Get game data file, stripped of comments.
/// ```
/// # use rust_wiki::file_handler::get_decommented_file_reader;
/// let reader = get_decommented_file_reader("DataLocal/stage.csv").unwrap();
/// let mut rdr = csv::Reader::from_reader(reader);
/// ```
pub fn get_decommented_file_reader(file_name: &str) -> Result<Cursor<String>, io::Error> {
    use FileLocation::GameData;
    let gd = get_file_location(GameData);
    let f = BufReader::new(File::open(gd.join(file_name))?)
        .lines()
        .map(|line| line.unwrap().split("//").next().unwrap().trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(Cursor::new(f))
}

/// temp function
pub fn do_stuff() {
    let file_name = "DataLocal/stage.csv";
    let content = fs::read_to_string(get_file_location(FileLocation::GameData).join(file_name))
        .expect("File name no existo!");
    println!("{content:?}");

    read_csv_file("DataLocal/stageRN000_00.csv");
    read_csv_file("DataLocal/stage.csv");
}

fn read_csv_file(file_name: &str) {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        // .from_reader(File::open(gd.join("DataLocal/stage00.csv")).unwrap())
        // .from_reader(File::open(gd.join("DataLocal/stage.csv")).unwrap())
        .from_reader(get_decommented_file_reader(file_name).unwrap());

    for result in rdr.byte_records() {
        println!("{:?}", result);
    }

    // check all stage files ig
}
// could probably do some trait thing
