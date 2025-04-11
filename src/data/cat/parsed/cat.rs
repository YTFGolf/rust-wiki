//! Deals with cat data.

use super::cat_stats::CatStats;
use crate::data::{cat::raw::read_data_file, version::Version};

/// Get cat unit.
pub fn get_unit(wiki_id: usize, version: &Version) -> impl Iterator<Item = CatStats> {
    let abs_id = wiki_id + 1;
    let file_name = format!("unit{abs_id:03}.csv");
    let combined_iter = read_data_file(&file_name, version);
    combined_iter.map(|combined| CatStats::from_combined(&combined))
}
