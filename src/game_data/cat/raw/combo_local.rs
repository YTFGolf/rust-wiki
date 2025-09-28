//! Deals with the localisation of combo data (combo names, effects etc.)

use crate::game_data::version::{Version, version_data::CacheableVersionData};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
/// Combo names for the version.
pub struct ComboNames {
    combos: Vec<String>,
}
impl CacheableVersionData for ComboNames {
    fn init_data(_: &Path) -> Self {
        unimplemented!();
    }

    fn init_data_with_version(version: &Version) -> Self {
        let file_name = format!("Nyancombo_{lang}.csv", lang = version.language());
        let reader =
            BufReader::new(File::open(version.get_file_path("resLocal").join(file_name)).unwrap());

        let combos = reader.lines().map(Result::unwrap).collect();
        Self { combos }
    }
}
impl ComboNames {
    /// Get combo name from combo id (i.e. line index in "NyancomboData.csv").
    pub fn combo_name(&self, ind: usize) -> Option<&str> {
        self.combos.get(ind).map(String::as_str)
    }
}

/*
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
) -> Option<impl Iterator<Item = CatDescription>> {
    let file_name = format!(
        "Unit_Explanation{inc}_{lang}.csv",
        inc = wiki_id + 1,
        lang = version.language()
    );

    let reader =
        BufReader::new(File::open(version.get_file_path("resLocal").join(file_name)).ok()?);

    Some(reader.lines().map(|line| {
        let line = line.unwrap();

        let delimiter = match version.language() {
            VersionLanguage::EN | VersionLanguage::KR | VersionLanguage::TW => '|',
            VersionLanguage::JP => ',',
            VersionLanguage::Fallback => unreachable!(),
        };

        line.split(delimiter)
            .collect::<ByteRecord>()
            .deserialize(None)
            .unwrap()
    })) */
