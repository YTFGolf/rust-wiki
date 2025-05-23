use crate::{config::Config, data::map::parsed::map::GameMap};

use super::legend::{get_map_wiki_data, stage_table};

pub fn get_event_map(map: &GameMap, config: &Config) -> String {
    log::warn!("Event map is incomplete.");
    let map_wiki_data = get_map_wiki_data(&map.id);
    stage_table(map, map_wiki_data, config.version.current_version())
}
