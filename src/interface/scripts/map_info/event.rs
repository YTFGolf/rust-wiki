//! Event map info.

use super::legend::get_map_wiki_data;
use crate::{
    game_data::map::parsed::map::GameMap,
    interface::{config::Config, scripts::map_info::common::stage_table},
};

/// Get event map info.
pub fn get_event_map(map: &GameMap, config: &Config) -> String {
    log::warn!("Event map is incomplete.");
    log::debug!("{map:?}");
    let map_wiki_data = get_map_wiki_data(&map.id);
    stage_table(map, map_wiki_data, config.version.current_version())
}

/// Only get the table.
pub fn only_table(map: &GameMap, config: &Config) -> String {
    let map_wiki_data = get_map_wiki_data(&map.id);
    stage_table(map, map_wiki_data, config.version.current_version())
}
