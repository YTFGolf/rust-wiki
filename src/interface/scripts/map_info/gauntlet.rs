//! Gauntlet map info.

use super::legend::get_map_wiki_data;
use crate::{
    game_data::{
        map::{
            parsed::map::{GameMap, ResetType},
            raw::map_data::GameMapData,
        },
        meta::stage::{map_id::MapID, stage_id::StageID},
    },
    interface::{
        config::Config,
        error_handler::InfallibleWrite,
        scripts::map_info::{common::stage_table, map_info::db_reference},
    },
    wiki_data::stage_wiki_data::MapWikiData,
    wikitext::{page::Page, section::Section, text_utils::extract_name},
};
use std::fmt::Write;

fn intro(_map: &GameMap, config: &Config, map_wiki_data: &MapWikiData) -> Section {
    let mut buf = String::new();
    let map_name = extract_name(&map_wiki_data.name);
    write!(
        buf,
        "'''{map_name}''' (?, ''?'', '''?''') is a [[Gauntlet]]",
    )
    .infallible_write();

    if true {
        // if config.map_info.version() {
        let mut ver = config.version.current_version().number();
        if let Some(s) = ver.strip_suffix(".0") {
            ver = s;
        }

        write!(
            buf,
            " that was added in [[Version {ver} Update|Version {ver}]]"
        )
        .infallible_write();
    }

    buf += ".";

    // match map.reset_type {
    //     ResetType::None | ResetType::ResetRewards | ResetType::ResetMaxClears => {
    //         unimplemented!("reset type is not resetting rewards and clear")
    //     }
    //     ResetType::ResetRewardsAndClear => write!(
    //         buf,
    //         " Progress made in {map_name} will reset when the event ends."
    //     )
    //     .infallible_write(),
    // }

    Section::blank(buf)
}

fn overview_section(map: &GameMap, config: &Config, map_wiki_data: &MapWikiData) -> Section {
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

fn page_end(map_id: &MapID) -> String {
    let base = String::from(
        "==First Appearance==\n===English Version===\n*September 15th, 2025 to September 29th, 2025\n===Japanese Version===\n*August 18th, 2025 to September 1st, 2025\n==Reference==\n",
    );
    base + "*"
        + &db_reference(map_id)
        + "\n\n{{SpecialStages List}}\n[[Category:Event Stages]]\n[[Category:Gauntlets]]"
}

/// Get gauntlet map info.
pub fn get_gauntlet_map(map: &GameMap, config: &Config) -> String {
    log::warn!("gauntlet map is incomplete.");
    let mut page = Page::blank();

    let map_wiki_data = get_map_wiki_data(&map.id);

    page.push(intro(map, config, map_wiki_data));
    page.push(overview_section(map, config, map_wiki_data));
    page.push(Section::h2(
        "List of Stages",
        stage_table(map, map_wiki_data, config.version.current_version()),
    ));
    page.push(Section::blank(page_end(&map.id)));

    page.to_string()
}
