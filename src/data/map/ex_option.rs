//! Module that deals with the `EX_option` file.

use crate::data::version::version_data::CacheableVersionData;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
/// Data stored in the EX option CSV.
pub struct ExOptionCSV {
    map_id: u32,
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
            map: get_ex_option(path),
        }
    }
}
impl ExOption {
    /// Get the ex map that the map gets invaded by.
    pub fn get_ex_map(&self, map_id: u32) -> Option<u32> {
        Some(self.map.iter().find(|o| o.map_id == map_id)?.ex_map_id)
    }
}

fn get_ex_option(path: &Path) -> Vec<ExOptionCSV> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'/'))
        .from_path(path.join("DataLocal/EX_option.csv"))
        .unwrap();

    let mut options = vec![];
    for record in rdr.byte_records() {
        let result = match record {
            Ok(r) => r,
            Err(_) => break,
        };

        let opt = result.deserialize(None).unwrap();
        options.push(opt);
    }

    options
}
