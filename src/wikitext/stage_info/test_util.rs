//! Util functions for testing stage info.
#![cfg(test)]
use crate::{
    data::stage::parsed::stage::Stage,
    wikitext::{data_files::stage_wiki_data::STAGE_WIKI_DATA, stage_info::StageWikiDataContainer},
};
/// Get the stage's [StageWikiData] for a test function.
#[deprecated]
pub fn get_stage_wiki_data(stage: &Stage) -> StageWikiDataContainer {
    let stage_map = STAGE_WIKI_DATA.stage_map(stage.id.map()).unwrap();
    let stage_name = stage_map.get(stage.id.num()).unwrap();
    StageWikiDataContainer {
        stage_map,
        stage_name,
    }
}
