//! Get info about a map.

use crate::{config::Config, data::map::parsed::map::MapData};

/// Get full map info.
pub fn get_map_info(map: &MapData, _config: &Config) -> String {
    println!("{map:#?}");
    todo!()
}
