//! Colosseum map info.

use crate::{
    Config,
    game_data::map::parsed::map::GameMap,
    interface::scripts::map_info::{common::stage_table, legend::get_map_wiki_data},
    regex_handler::static_regex,
};

/// Get colosseum map info.
pub fn get_colosseum_map(map: &GameMap, config: &Config) -> String {
    let map_wiki_data = get_map_wiki_data(&map.id);

    let m = stage_table(map, map_wiki_data, config.version.current_version());

    let maps = static_regex(r"(Mapname|Mapsn)\d{3}");
    let m = maps.replace_all(&m, "${1}000");

    let romaji = static_regex(r"(link=Round )(\d)(.*\n.*?)\?");
    let m = romaji.replace_all(&m, "$1$2${3}Round $2");

    let translation = static_regex(r"(\|Round )(\d)(.*\n.*?)\?");
    let m = translation.replace_all(&m, "$1$2${3}Round $2");

    m.into_owned()
}
