//! Evolution evolution descriptions.

use crate::game_data::version::{
    Version, lang::VersionLanguage, version_data::CacheableVersionData,
};
use csv::ByteRecord;
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const AT: &str = "＠";

// True Form evolution increases all traits|and grants Wave Attack abilities!|＠|＠|Ultra Form evolution gains improved|movement speed and Omnistrike!|Plus, upgraded Wave attacks!|
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
/// Description of the evolution.
pub struct EvolutionDescription {
    tf_line_1: String,
    tf_line_2: String,
    tf_line_3: String,
    _maybe_blank: String,
    uf_line_1: String,
    uf_line_2: String,
    uf_line_3: String,
    #[serde(default)]
    name_comment: String,
}
impl EvolutionDescription {
    /// Get True Form evolution description.
    pub fn tf(&self) -> String {
        [&self.tf_line_1, &self.tf_line_2, &self.tf_line_3]
            .iter()
            .filter_map(|line| {
                let l = line.trim();
                if l.is_empty() || l == AT {
                    None
                } else {
                    Some(l)
                }
            })
            .collect::<Vec<_>>()
            .join("<br>")
    }

    /// Get Ultra Form evolution description.
    pub fn uf(&self) -> String {
        [&self.uf_line_1, &self.uf_line_2, &self.uf_line_3]
            .iter()
            .filter_map(|line| {
                let l = line.trim();
                if l.is_empty() || l == AT {
                    None
                } else {
                    Some(l)
                }
            })
            .collect::<Vec<_>>()
            .join("<br>")
    }
}

/// Get descriptions for the unit.
pub fn get_evolution_descriptions(version: &Version) -> Vec<EvolutionDescription> {
    let file_name = format!("unitevolve_{lang}.csv", lang = version.language());

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

            line.split(delimiter)
                .collect::<ByteRecord>()
                .deserialize(None)
                .unwrap()
        })
        .collect()
}

#[derive(Debug)]
/// Combo names for the version.
pub struct EvolutionDescriptions {
    descs: Vec<EvolutionDescription>,
}

impl CacheableVersionData for EvolutionDescriptions {
    fn init_data(_: &Path) -> Self {
        unimplemented!();
    }

    fn init_data_with_version(version: &Version) -> Self {
        Self {
            descs: get_evolution_descriptions(version),
        }
    }
}

impl EvolutionDescriptions {
    /// Get the cat's evolution description.
    pub fn get_cat_evolution_desc(&self, id: usize) -> Option<&EvolutionDescription> {
        self.descs.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn assert_blanks() {
        fn do_version(version: &Version) {
            for (i, desc) in get_evolution_descriptions(version).into_iter().enumerate() {
                assert!(
                    desc._maybe_blank == AT
                        || desc._maybe_blank.is_empty()
                        || desc.name_comment.is_empty(),
                    // if name comment is empty then has been shifted
                    // erroneously and name comment has been serde defaulted, in
                    // which case it will be empty
                    "Description with id {i} doesn't have {AT}. {desc:?}"
                );
                let comment = &desc.name_comment;
                assert!(
                    comment.trim().starts_with("//") || comment.is_empty() || comment == AT,
                    "Description with id {i} has a weird order. {desc:?}"
                );
            }
        }
        do_version(TEST_CONFIG.version.en());
        do_version(TEST_CONFIG.version.jp());
        do_version(TEST_CONFIG.version.kr());
        do_version(TEST_CONFIG.version.tw());
    }
}
