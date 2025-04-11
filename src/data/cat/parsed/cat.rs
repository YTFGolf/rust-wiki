//! Deals with cat data.

use super::cat_stats::CatStats;
use crate::data::{cat::raw::read_data_file, version::Version};

#[derive(Debug)]
pub struct CatForm {
    pub stats: CatStats,
    // anim
    // desc
}

#[derive(Debug)]
pub struct Cat {
    pub forms: Vec<CatForm>,
    // is ancient egg if last column of "unitbuy.csv" says so
    // xp curve
    // growth curve
    // talents
    // evolutions
    // combos
}

impl Cat {
    pub fn from_wiki_id(wiki_id: u32, version: &Version) -> Self {
        let forms = Self::get_stats(wiki_id, version)
            .map(|stats| CatForm { stats })
            .collect();
        Self { forms }
    }

    /// Get stats for each form.
    pub fn get_stats(wiki_id: u32, version: &Version) -> impl Iterator<Item = CatStats> {
        // get_stats(wiki_id + 1, version)
        let abs_id = wiki_id + 1;
        let file_name = format!("unit{abs_id:03}.csv");
        let combined_iter = read_data_file(&file_name, version);
        combined_iter.map(|combined| CatStats::from_combined(&combined))
    }
}
