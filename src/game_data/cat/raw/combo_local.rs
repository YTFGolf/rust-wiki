//! Deals with the localisation of combo data (combo names, effects etc.)

use crate::game_data::version::{
    Version,
    lang::VersionLanguage,
    version_data::{CacheableVersionData, CvdCreateError, CvdResult},
};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Default)]
/// Combo names for the version.
pub struct ComboNames {
    combos: Vec<String>,
}
impl CacheableVersionData for ComboNames {
    fn create(version: &Version) -> CvdResult<Self> {
        let file_name = format!("Nyancombo_{lang}.csv", lang = version.language());
        let reader = BufReader::new(
            File::open(version.get_file_path("resLocal").join(file_name))
                .map_err(CvdCreateError::default_from_err)?,
        );

        let records: Result<Vec<_>, io::Error> = reader.lines().collect();
        let combos = records.map_err(CvdCreateError::default_from_err)?;
        Ok(Self { combos })
    }
}
impl ComboNames {
    /// Get combo name from combo id (i.e. line index in "NyancomboData.csv").
    pub fn combo_name(&self, ind: usize) -> Option<&str> {
        self.combos.get(ind).map(String::as_str)
    }
}

/// Get the names of combo effects.
pub fn get_effect_names(version: &Version) -> Result<Vec<String>, Box<dyn Error>> {
    let file_name = format!("Nyancombo1_{lang}.csv", lang = version.language());

    let reader = BufReader::new(
        File::open(version.get_file_path("resLocal").join(file_name)).map_err(Box::new)?,
    );

    reader
        .lines()
        .map(|line| {
            let line = line.map_err(Box::new)?;

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

            Ok(effect.to_string())
        })
        .collect()
}

#[derive(Debug, Default)]
/// Combo names for the version.
pub struct ComboEffects {
    effects: Vec<String>,
}
impl CacheableVersionData for ComboEffects {
    fn create(version: &Version) -> CvdResult<Self> {
        Ok(Self {
            effects: get_effect_names(version).map_err(CvdCreateError::as_default)?,
        })
    }
}
impl ComboEffects {
    /// Get effect name from effect id.
    pub fn effect_name(&self, ind: usize) -> Option<&str> {
        self.effects.get(ind).map(String::as_str)
    }
}

/// Get the names of combo intensities.
pub fn get_intensity_names(version: &Version) -> Result<Vec<String>, Box<dyn Error>> {
    let file_name = format!("Nyancombo2_{lang}.csv", lang = version.language());

    let reader = BufReader::new(
        File::open(version.get_file_path("resLocal").join(file_name)).map_err(Box::new)?,
    );

    reader
        .lines()
        .map(|line| {
            let line = line.map_err(Box::new)?;

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

            Ok(intensity.to_string())
        })
        .collect()
}

#[derive(Debug, Default)]
/// Combo names for the version.
pub struct ComboIntensities {
    intensities: Vec<String>,
}
impl CacheableVersionData for ComboIntensities {
    fn create(version: &Version) -> CvdResult<Self> {
        Ok(Self {
            intensities: get_intensity_names(version).map_err(CvdCreateError::as_default)?,
        })
    }
}
impl ComboIntensities {
    /// Get intensity name from intensity id.
    pub fn intensity_name(&self, ind: usize) -> Option<&str> {
        self.intensities.get(ind).map(String::as_str)
    }
}
