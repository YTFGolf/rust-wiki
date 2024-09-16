//! Contains functions to read data files.
use crate::config::CONFIG;
use std::{
    fs::{self, File},
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
use csv::ByteRecord;
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
    let file_name = "DataLocal/stage.csv";
    let content = fs::read_to_string(get_file_location(GameData).join(file_name))
        .expect("File name no existo!");
    println!("{content:?}");

    read_csv_file("DataLocal/stageRN000_00.csv");
    read_csv_file("DataLocal/stageRS250_00.csv");
    read_csv_file("DataLocal/stage00.csv");
    // read_csv_file("DataLocal/stage.csv");
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct HeaderCSV {
    base_id: u32,
    no_cont: u32,
    cont_chance: u32,
    contmap_id: u32,
    cont_stage_idmin: u32,
    cont_stage_idmax: u32,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Line2CSV {
    width: u32,
    base_hp: u32,
    unknown_1: u32,
    unknown_2: u32,
    background_id: u32,
    max_enemies: u32,
    animbase_id: u32,
    time_limit: u32,
    indestructible: u32,
    unknown_3: Option<u32>,
}

fn read_csv_file(file_name: &str) {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        // .from_reader(File::open(gd.join("DataLocal/stage00.csv")).unwrap())
        // .from_reader(File::open(gd.join("DataLocal/stage.csv")).unwrap())
        .from_reader(get_decommented_file_reader(file_name).unwrap());

    let mut records = rdr.byte_records();

    let mut head = records.next().unwrap().unwrap();
    let csv_head: HeaderCSV = if head.len() <= 7 || head[6].is_empty() {
        let tmp = head;
        head = records.next().unwrap().unwrap();
        tmp.deserialize(None).unwrap()
    } else {
        ByteRecord::from(vec!["0", "0", "0", "0", "0", "0", ""])
            .deserialize(None)
            .unwrap()
    };
    let line_2 = head;
    let csv_line_2: Line2CSV = line_2.deserialize(None).unwrap();

    println!("{csv_head:?}");
    println!("{csv_line_2:?}");

    for result in rdr.byte_records() {
        println!("{:?}", result);
    }

    // check all stage files ig
}
