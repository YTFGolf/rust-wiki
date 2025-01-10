//! Util functions for testing stage info.
#![cfg(test)]
use crate::{
    data::stage::parsed::stage::Stage,
    wikitext::{data_files::stage_wiki_data::STAGE_WIKI_DATA, stage_info::StageWikiData},
};
/// Get the stage's [StageWikiData] for a test function.
pub fn get_stage_wiki_data(stage: &Stage) -> StageWikiData {
    let stage_map = STAGE_WIKI_DATA
        .stage_map(stage.meta.type_num, stage.meta.map_num)
        .unwrap();
    let stage_name = stage_map.get(stage.meta.stage_num).unwrap();
    StageWikiData {
        stage_map,
        stage_name,
    }
}
