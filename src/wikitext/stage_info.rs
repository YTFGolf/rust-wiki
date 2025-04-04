//! Get information about a stage.

use super::data_files::stage_wiki_data::{MapWikiData, STAGE_WIKI_DATA, StageWikiData};
use super::format_parser::{ParseType, parse_info_format};
use crate::config::Config;
use crate::data::stage::parsed::stage::Stage;
use crate::meta::stage::stage_id::StageID;
use regex::Regex;
use std::fmt::{Display, Write};
use variables::{DEFAULT_FORMAT, get_stage_variable};
mod battlegrounds;
mod beginning;
mod enemies_list;
mod tests;
mod information;
mod misc_information;
mod restrictions;
mod treasure;
mod variables;

struct StageWikiDataContainer {
    stage_map: &'static MapWikiData,
    stage_name: &'static StageWikiData,
}

/// Get full stage info.
pub fn get_stage_info(stage: &Stage, config: &Config) -> impl Display {
    get_stage_info_formatted(stage, DEFAULT_FORMAT, config)
}

/// Get stage info based on specified format.
pub fn get_stage_info_formatted(stage: &Stage, format: &str, config: &Config) -> String {
    let parsed = parse_info_format(format);
    let mut buf = String::new();
    let stage_wiki_data = get_stage_wiki_data(&stage.id);

    for node in parsed {
        if node.ptype == ParseType::Text {
            buf.write_str(node.content).unwrap();
            continue;
        }

        let new_buf = get_stage_variable(node.content, stage, &stage_wiki_data, config);
        buf.write_str(&new_buf).unwrap();
    }

    let buf = Regex::new(r"\n==.*==\n\n").unwrap().replace_all(&buf, "");
    // Remove empty sections.

    buf.into_owned()
}

fn get_stage_wiki_data(stage: &StageID) -> StageWikiDataContainer {
    let stage_map = STAGE_WIKI_DATA
        .stage_map(stage.map())
        .unwrap_or_else(|| panic!("Couldn't find map name: {}", stage.map()));
    let stage_name = stage_map
        .get(stage.num())
        .unwrap_or_else(|| panic!("Couldn't find stage name: {}", stage));

    StageWikiDataContainer {
        stage_map,
        stage_name,
    }
}
