//! Module that deals with the `EX_option` file.

use crate::{game_data::version::version_data::CacheableVersionData, game_data::meta::stage::map_id::MapID};
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
/// Data stored in the EX option CSV.
pub struct ExOptionCSV {
    /// Map that EX option applies to.
    map_id: u32,
    /// EX map that all stages in map are invaded by.
    ex_map_id: u32,
}

#[derive(Debug)]
/// Container for EX option data.
pub struct ExOption {
    map: Vec<ExOptionCSV>,
}
impl CacheableVersionData for ExOption {
    fn init_data(path: &std::path::Path) -> Self {
        Self {
            map: get_ex_option(path).unwrap_or_default(),
        }
    }
}
impl ExOption {
    /// Get the ex map that the map gets invaded by.
    pub fn get_ex_map(&self, map_id: &MapID) -> Option<u32> {
        Some(
            self.map
                .iter()
                .find(|o| o.map_id == map_id.mapid())?
                .ex_map_id,
        )
    }
}

fn get_ex_option(path: &Path) -> Option<Vec<ExOptionCSV>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'/'))
        .from_path(path.join("DataLocal/EX_option.csv"))
        .ok()?;

    let mut options = vec![];
    for record in rdr.byte_records() {
        let Ok(result) = record else { break };

        let opt = result.deserialize(None).unwrap();
        options.push(opt);
    }

    Some(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TEST_CONFIG;
    use std::collections::HashSet;

    #[test]
    fn assert_no_dupes() {
        let mut seen = HashSet::new();
        let data: &ExOption = TEST_CONFIG.version.current_version().get_cached_file();
        for option in &data.map {
            assert!(seen.insert(option.map_id));
        }
    }
}
