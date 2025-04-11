//! Deals with cat data.

use super::cat_stats::CatStats;
use crate::data::{cat::raw::stats::read_data_file, version::Version};

#[derive(Debug)]
/// Individual form of a cat.
pub struct CatForm {
    /// Form's stats.
    pub stats: CatStats,
    // anim
    // desc
}

type CatForms = Vec<CatForm>;
// might need to create an actual type for this; also needs unitbuy first.

#[derive(Debug)]
/// Parsed cat object.
pub struct Cat {
    /// CRO id.
    pub id: u32,
    /// Cat's forms.
    pub forms: CatForms,
    // xp curve (unitbuy and unitexp)
    // growth curve
    // talents
    // evolutions
    // combos
}

impl Cat {
    /// Get cat from wiki id.
    pub fn from_wiki_id(wiki_id: u32, version: &Version) -> Self {
        let forms = Self::get_stats(wiki_id, version)
            .map(|stats| CatForm { stats })
            .collect();
        Self { id: wiki_id, forms }
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
