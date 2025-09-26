//! Gauntlet map info.

use super::legend::get_map_wiki_data;
use crate::{
    game_data::{
        map::{parsed::map::GameMap, raw::map_data::GameMapData},
        meta::stage::stage_id::StageID,
    },
    interface::{
        config::Config, error_handler::InfallibleWrite, scripts::map_info::common::stage_table,
    },
    wiki_data::stage_wiki_data::MapWikiData,
    wikitext::{page::Page, section::Section, text_utils::extract_name},
};
use std::fmt::Write;

fn get_overview_section(map: &GameMap, config: &Config, map_wiki_data: &MapWikiData) -> Section {
    let amt_stages = {
        let mut stage_id = StageID::from_map(map.id.clone(), 0);
        let version = config.version.current_version();
        while GameMapData::get_stage_data(&stage_id, version).is_some() {
            stage_id.set_num(stage_id.num() + 1);
        }

        stage_id.num()
    };

    let mut overview = format!(
        "{name} contains a total of {amt_stages} stages.",
        name = extract_name(&map_wiki_data.name)
    );

    match (map.hidden_upon_clear, map.max_clears, map.cooldown) {
        (false, Some(m), Some(c)) => {
            let m = m.get();
            let c = c.get();

            write!(
                overview,
                " After beating {m} stages, the player must wait for {cm} minutes before they may proceed to the next one ({c} minutes for one available).",
                cm = c * m
            ).infallible_write();
        }
        _ => unimplemented!("combination of hide upon clear, max clears and gauntlet cooldown"),
    }
    // Summer Break Cats contains a total of 20 stages.
    // After beating three stages, the player must wait for 90 minutes before they may proceed to the next one (30 minutes for one available).
    // Rewards for each stage completion are Legend Nets.

    Section::h2("Overview", overview)
}

/// Get gauntlet map info.
pub fn get_gauntlet_map(map: &GameMap, config: &Config) -> String {
    log::warn!("gauntlet map is incomplete.");
    let mut page = Page::blank();

    let map_wiki_data = get_map_wiki_data(&map.id);

    page.push(get_overview_section(map, config, map_wiki_data));
    page.push(Section::h2(
        "List of Stages",
        stage_table(map, map_wiki_data, config.version.current_version()),
    ));

    page.to_string()
}
