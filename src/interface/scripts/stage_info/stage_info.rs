use super::{
    super::format_parser::{ParseType, parse_info_format},
    variables::{DEFAULT_FORMAT, get_stage_variable},
};
use crate::{
    config::Config,
    game_data::{meta::stage::stage_id::StageID, stage::parsed::stage::Stage},
    interface::error_handler::InfallibleWrite,
    regex_handler::static_regex,
    wiki_data::stage_wiki_data::{MapWikiData, STAGE_WIKI_DATA, StageWikiData},
};
use std::fmt::{Display, Write};

/// Container for wiki data about a stage.
pub struct StageWikiDataContainer {
    // TODO rename
    /// Stage's map.
    pub stage_map: &'static MapWikiData,
    /// Stage itself.
    pub stage_name: &'static StageWikiData,
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
            buf.write_str(node.content).infallible_write();
            continue;
        }

        let new_buf = get_stage_variable(node.content, stage, &stage_wiki_data, config);
        buf.write_str(&new_buf).infallible_write();
    }

    let buf = static_regex(r"\n==.*==\n\n").replace_all(&buf, "");
    // Remove empty sections.

    buf.into_owned()
}

/// Get the stage's corresponding wiki data.
pub fn get_stage_wiki_data(stage: &StageID) -> StageWikiDataContainer {
    let stage_map = STAGE_WIKI_DATA
        .stage_map(stage.map())
        .unwrap_or_else(|| panic!("Couldn't find map name: {}", stage.map()));
    let stage_name = stage_map
        .get(stage.num())
        .unwrap_or_else(|| panic!("Couldn't find stage name: {stage}"));

    StageWikiDataContainer {
        stage_map,
        stage_name,
    }
}
