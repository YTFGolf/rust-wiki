//! Get information about a stage.

use super::data_files::stage_wiki_data::{MapWikiData, STAGE_WIKI_DATA, StageWikiData};
use super::format_parser::{ParseType, parse_info_format};
use crate::config::Config;
use crate::data::stage::parsed::stage::Stage;
use regex::Regex;
use std::fmt::Write;
use variables::{DEFAULT_FORMAT, get_stage_variable};
mod battlegrounds;
mod beginning;
mod enemies_list;
mod information;
mod misc_information;
mod restrictions;
mod test_util;
mod treasure;
mod variables;

struct StageWikiDataContainer {
    stage_map: &'static MapWikiData,
    stage_name: &'static StageWikiData,
}

/// Get full stage info.
pub fn get_stage_info(stage: &Stage, config: &Config) -> String {
    get_stage_info_formatted(stage, DEFAULT_FORMAT, config)
}

/// Get stage info based on specified format.
pub fn get_stage_info_formatted(stage: &Stage, format: &str, config: &Config) -> String {
    let parsed = parse_info_format(format);

    let mut buf = String::new();

    let stage_map = STAGE_WIKI_DATA
        .stage_map(stage.id.map())
        .unwrap_or_else(|| {
            panic!(
                "Couldn't find map name: {:03}-{:03}",
                stage.id.variant().num(),
                stage.id.map().num()
            )
        });
    let stage_name = stage_map.get(stage.id.num()).unwrap_or_else(|| {
        panic!(
            "Couldn't find stage name: {:03}-{:03}-{:03}",
            stage.id.variant().num(),
            stage.id.map().num(),
            stage.id.num()
        )
    });

    let stage_wiki_data = StageWikiDataContainer {
        stage_map,
        stage_name,
    };

    for node in parsed {
        if node.ptype == ParseType::Text {
            buf.write_str(node.content).unwrap();
            continue;
        }

        let new_buf = get_stage_variable(node.content, stage, &stage_wiki_data, config);
        buf.write_str(&new_buf).unwrap();
    }

    let buf = Regex::new(r"\n+(\||\}\})")
        .unwrap()
        .replace_all(&buf, "\n$1");
    let buf = Regex::new(r"\n==.*==\n\n").unwrap().replace_all(&buf, "");

    buf.to_string()
}
