//! Deals with the localisation of combo data (combo names, effects etc.)

use crate::game_data::version::{
    Version, lang::VersionLanguage, version_data::CacheableVersionData,
};
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

/// Get the names of combo effects.
pub fn get_effect_names(version: &Version) -> Vec<String> {
    let file_name = format!("Nyancombo1_{lang}.csv", lang = version.language());

    let reader =
        BufReader::new(File::open(version.get_file_path("resLocal").join(file_name)).unwrap());

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let delimiter = match version.language() {
                VersionLanguage::EN | VersionLanguage::KR | VersionLanguage::TW => '|',
                VersionLanguage::JP => ',',
                VersionLanguage::Fallback => unreachable!(),
            };

            let mut iter = line.split(delimiter);
            let effect = iter.next().expect("first item should always exist");

            match iter.next() {
                None | Some("") => (),
                _ => panic!("found text after the delimiter"),
            }

            effect.to_string()
        })
        .collect()
}

#[derive(Debug)]
/// Combo names for the version.
pub struct ComboEffects {
    effects: Vec<String>,
}
impl CacheableVersionData for ComboEffects {
    fn init_data(_: &Path) -> Self {
        unimplemented!();
    }

    fn init_data_with_version(version: &Version) -> Self {
        Self {
            effects: get_effect_names(version),
        }
    }
}
impl ComboEffects {
    /// Get effect name from effect id.
    pub fn effect_name(&self, ind: usize) -> Option<&str> {
        self.effects.get(ind).map(String::as_str)
    }
}

/// Get the names of combo intensities.
pub fn get_intensity_names(version: &Version) -> Vec<String> {
    let file_name = format!("Nyancombo2_{lang}.csv", lang = version.language());

    let reader =
        BufReader::new(File::open(version.get_file_path("resLocal").join(file_name)).unwrap());

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let delimiter = match version.language() {
                VersionLanguage::EN | VersionLanguage::KR | VersionLanguage::TW => '|',
                VersionLanguage::JP => ',',
                VersionLanguage::Fallback => unreachable!(),
            };

            let mut iter = line.split(delimiter);
            let intensity = iter.next().expect("first item should always exist");

            match iter.next() {
                None | Some("") => (),
                _ => panic!("found text after the delimiter"),
            }

            intensity.to_string()
        })
        .collect()
}

#[derive(Debug)]
/// Combo names for the version.
pub struct ComboIntensities {
    intensities: Vec<String>,
}
impl CacheableVersionData for ComboIntensities {
    fn init_data(_: &Path) -> Self {
        unimplemented!();
    }

    fn init_data_with_version(version: &Version) -> Self {
        Self {
            intensities: get_intensity_names(version),
        }
    }
}
impl ComboIntensities {
    /// Get intensity name from intensity id.
    pub fn intensity_name(&self, ind: usize) -> Option<&str> {
        self.intensities.get(ind).map(String::as_str)
    }
}
