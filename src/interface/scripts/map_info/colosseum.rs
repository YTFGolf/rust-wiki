use crate::{
    Config, game_data::map::parsed::map::GameMap,
    interface::scripts::map_info::event::get_event_map, regex_handler::static_regex,
};

pub fn get_colosseum_map(map: &GameMap, config: &Config) -> String {
    let m = get_event_map(map, config);

    let maps = static_regex(r"(Mapname|Mapsn)\d{3}");
    let m = maps.replace_all(&m, "${1}000");

    let romaji = static_regex(r"(link=Round )(\d)(.*\n.*?)\?");
    let m = romaji.replace_all(&m, "$1$2${3}Round $2");

    let translation = static_regex(r"(\|Round )(\d)(.*\n.*?)\?");
    let m = translation.replace_all(&m, "$1$2${3}Round $2");

    m.into_owned()
}
