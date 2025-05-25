use super::legend::{get_map_wiki_data, stage_table};
use crate::{interface::config::Config, game_data::map::parsed::map::GameMap};

pub fn get_event_map(map: &GameMap, config: &Config) -> String {
    log::warn!("Event map is incomplete.");
    let map_wiki_data = get_map_wiki_data(&map.id);
    stage_table(map, map_wiki_data, config.version.current_version())
}
