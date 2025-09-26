//! Gauntlet map info.

use super::legend::get_map_wiki_data;
use crate::{
    game_data::map::parsed::map::GameMap,
    interface::{config::Config, scripts::map_info::common::stage_table},
};

/// Get gauntlet map info.
pub fn get_gauntlet_map(map: &GameMap, config: &Config) -> String {
    log::warn!("gauntlet map is incomplete.");
    let map_wiki_data = get_map_wiki_data(&map.id);
    stage_table(map, map_wiki_data, config.version.current_version())
}
