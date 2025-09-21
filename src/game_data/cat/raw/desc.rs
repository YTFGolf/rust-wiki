//! Cat descriptions.

use crate::game_data::version::{Version, lang::VersionLanguage};
use csv::ByteRecord;
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
/// Description of the cat.
pub struct CatDescription {
    name: String,
    line1: String,
    line2: String,
    line3: String,
    jp_furigana: String,
}
impl CatDescription {
    /// Cat's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Lines of the description, joined by `<br>`s.
    pub fn lines(&self) -> String {
        [&self.line1, &self.line2, &self.line3]
            .iter()
            .filter_map(|line| {
                let l = line.trim();
                if l.is_empty() { None } else { Some(l) }
            })
            .collect::<Vec<_>>()
            .join("<br>")
    }
}

/// Get descriptions for the unit.
pub fn get_cat_descriptions(
    wiki_id: u32,
    version: &Version,
) -> impl Iterator<Item = CatDescription> {
    let file_name = format!(
        "Unit_Explanation{inc}_{lang}.csv",
        inc = wiki_id + 1,
        lang = version.language()
    );

    let reader =
        BufReader::new(File::open(version.get_file_path("resLocal").join(file_name)).unwrap());

    reader.lines().map(|line| {
        let line = line.unwrap();
        println!("{line:?}");

        let delimiter = match version.language() {
            VersionLanguage::EN => '|',
            VersionLanguage::JP => ',',
            VersionLanguage::KR => '|',
            VersionLanguage::TW => '|',
            VersionLanguage::Fallback => unreachable!(),
        };

        line.split(delimiter)
            .collect::<ByteRecord>()
            .deserialize(None)
            .unwrap()
    })
}
