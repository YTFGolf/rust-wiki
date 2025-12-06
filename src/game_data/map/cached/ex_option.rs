//! Module that deals with the `EX_option` file.

use crate::game_data::{
    meta::stage::map_id::MapID,
    version::{
        Version,
        version_data::{CacheableVersionData, CvdCreateError, CvdResult},
    },
};
use std::{error::Error, path::Path};

#[derive(Debug, serde::Deserialize)]
/// Data stored in the EX option CSV.
pub struct ExOptionCSV {
    /// Map that EX option applies to.
    map_id: u32,
    /// EX map that all stages in map are invaded by.
    ex_map_id: u32,
}

fn get_ex_option(path: &Path) -> Result<Vec<ExOptionCSV>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'/'))
        .from_path(path.join("DataLocal/EX_option.csv"))
        .map_err(Box::new)?;

    rdr.byte_records()
        .map(|record| {
            let result = record.map_err(Box::new)?;
            let opt = result.deserialize(None).map_err(Box::new)?;
            Ok(opt)
        })
        .collect()
}

#[derive(Debug, Default)]
/// Container for EX option data.
pub struct ExOption {
    map: Vec<ExOptionCSV>,
}
impl CacheableVersionData for ExOption {
    fn create(version: &Version) -> CvdResult<Self> {
        Ok(Self {
            map: get_ex_option(version.location()).map_err(CvdCreateError::as_default)?,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;
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
