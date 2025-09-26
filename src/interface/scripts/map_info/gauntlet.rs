//! Gauntlet map info.

use super::legend::get_map_wiki_data;
use crate::{
    game_data::{
        map::{parsed::map::GameMap, raw::map_data::GameMapData},
        meta::stage::stage_id::StageID,
    },
    interface::{config::Config, scripts::map_info::common::stage_table},
    wikitext::{page::Page, section::Section, text_utils::extract_name},
};

/// Get gauntlet map info.
pub fn get_gauntlet_map(map: &GameMap, config: &Config) -> String {
    log::warn!("gauntlet map is incomplete.");
    let mut page = Page::blank();

    let map_wiki_data = get_map_wiki_data(&map.id);
    page.push(Section::h2(
        "List of Stages",
        stage_table(map, map_wiki_data, config.version.current_version()),
    ));

    page.to_string()
}
