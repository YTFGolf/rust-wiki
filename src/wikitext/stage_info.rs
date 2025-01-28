//! Get information about a stage.

use super::data_files::stage_wiki_data::{MapData, StageData, STAGE_WIKI_DATA};
use super::format_parser::{parse_si_format, ParseType};
use crate::config::Config;
use crate::data::stage::parsed::stage::Stage;
use regex::Regex;
use std::fmt::Write;
use variables::{get_stage_variable, DEFAULT_FORMAT};
mod battlegrounds;
mod beginning;
mod enemies_list;
mod information;
mod misc_information;
mod restrictions;
mod test_util;
mod treasure;
mod variables;

struct StageWikiData {
    stage_map: &'static MapData,
    stage_name: &'static StageData,
}

/// Get full stage info.
pub fn get_stage_info(stage: &Stage, config: &Config) -> String {
    get_stage_info_formatted(stage, DEFAULT_FORMAT, config)
}

/// Get stage info based on specified format.
pub fn get_stage_info_formatted(stage: &Stage, format: &str, config: &Config) -> String {
    let parsed = parse_si_format(format);

    let mut buf = String::new();

    let stage_map = STAGE_WIKI_DATA
        .stage_map(stage.meta.type_num, stage.meta.map_num)
        .unwrap_or_else(|| {
            panic!(
                "Couldn't find map name: {:03}-{:03}",
                stage.meta.type_num, stage.meta.map_num
            )
        });
    let stage_name = stage_map.get(stage.meta.stage_num).unwrap_or_else(|| {
        panic!(
            "Couldn't find stage name: {:03}-{:03}-{:03}",
            stage.meta.type_num, stage.meta.map_num, stage.meta.stage_num
        )
    });

    let stage_wiki_data = StageWikiData {
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
