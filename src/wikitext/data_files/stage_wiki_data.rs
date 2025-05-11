//! Module that gets information about stage names and continue stages.

use crate::{
    file_handler::{FileLocation, get_file_location},
    meta::stage::{
        map_id::MapID, stage_id::StageID, stage_types::MAX_VARIANT_INDEX, variant::StageVariantID,
    },
};
use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug)]
/// Data about all possible stage types.
pub struct StageVariantWikiData {
    /// Name of the type.
    pub name: String,
    _num: u32,
    maps: HashMap<u32, MapWikiData>,
}
impl StageVariantWikiData {
    /// Get map with map id `map_id` in this type.
    pub fn get(&self, map_id: u32) -> Option<&MapWikiData> {
        self.maps.get(&map_id)
    }
}

#[derive(Debug)]
/// Data about stage maps.
pub struct MapWikiData {
    /// Name of the map.
    pub name: String,
    _num: u32,
    stages: Vec<StageWikiData>,
}
impl MapWikiData {
    /// Get stage with stage id `stage_id` in this map.
    pub fn get(&self, stage_id: u32) -> Option<&StageWikiData> {
        self.stages.get(stage_id as usize)
    }
    /// Does the map have any stages.
    pub fn is_empty(&self) -> bool {
        self.stages.is_empty()
    }
}

#[derive(Debug)]
/// Data about individual stages.
pub struct StageWikiData {
    /// Name of the stage.
    pub name: String,
    _num: u32,
}

type StageNameMap = [Option<StageVariantWikiData>; MAX_VARIANT_INDEX];
type ContinueStagesMap = Vec<Option<(u32, u32)>>;
type StageDifficultyMap = HashMap<String, u8>;
#[derive(Debug)]
/// Container for [STAGE_WIKI_DATA] static.
pub struct StageWikiDataContainer {
    stage_name_map: LazyLock<StageNameMap>,
    continue_stages: LazyLock<ContinueStagesMap>,
    stage_difficulty_map: LazyLock<StageDifficultyMap>,
}

impl StageWikiDataContainer {
    /// Get stage type.
    pub fn stage_type(&self, id: StageVariantID) -> Option<&StageVariantWikiData> {
        self.stage_name_map.get(id.num() as usize)?.into()
    }

    /// Get stage map.
    pub fn stage_map(&self, id: &MapID) -> Option<&MapWikiData> {
        self.stage_type(id.variant())?.get(id.num())
    }

    /// Get stage.
    pub fn stage(&self, id: &StageID) -> Option<&StageWikiData> {
        self.stage_map(id.map())?.get(id.num())
    }

    /// Get the type and map numbers from the ex map id.
    pub fn continue_id(&self, ex_map_id: u32) -> Option<(u32, u32)> {
        self.continue_stages[ex_map_id as usize]
    }

    /// Get stage difficulty.
    pub fn difficulty(&self, id: &StageID) -> Option<&u8> {
        self.difficulty_str(&id.to_string())
    }

    /// Get stage difficulty from the string key.
    fn difficulty_str(&self, id: &str) -> Option<&u8> {
        self.stage_difficulty_map.get(id)
    }
}

/// Contains parsed StageNames.csv file.
pub static STAGE_WIKI_DATA: StageWikiDataContainer = StageWikiDataContainer {
    stage_name_map: LazyLock::new(get_stage_name_map),
    continue_stages: LazyLock::new(get_continue_stages_map),
    stage_difficulty_map: LazyLock::new(get_stage_difficulty_map),
};

#[derive(Debug, Deserialize)]
struct StageNamesLine {
    #[serde(rename = "Type")]
    type_num: u32,
    #[serde(rename = "Map")]
    map_num: Option<u32>,
    #[serde(rename = "Stage")]
    stage_num: Option<u32>,
    #[serde(rename = "Link (EN)")]
    link: String,
}
#[derive(Debug, Deserialize)]
struct ContinueStagesLine {
    #[serde(rename = "EX Map Name", skip)]
    _ex_map_name: String,
    #[serde(rename = "Type")]
    type_num: u32,
    #[serde(rename = "Map")]
    map_num: u32,
}
#[derive(Debug, Deserialize)]
struct StageDifficultyLine {
    stage_id: String,
    difficulty: u8,
}

fn get_stage_name_map() -> StageNameMap {
    let mut map = [const { None }; MAX_VARIANT_INDEX];

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .comment(Some(b'#'))
        .from_path(get_file_location(&FileLocation::WikiData).join("StageNames.csv"));

    for result in rdr.expect("couldn't open stage names map").deserialize() {
        let record: StageNamesLine = result.expect("invalid stage names line");
        match (record.type_num, record.map_num, record.stage_num) {
            (n, None, None) => {
                // stage type
                map[n as usize] = Some(StageVariantWikiData {
                    name: record.link,
                    _num: n,
                    maps: HashMap::new(),
                });
            }
            (t, Some(m), None) => {
                // stage map
                let type_data = map[t as usize]
                    .as_mut()
                    .unwrap_or_else(|| panic!("Stage type {t:03} not found"));
                type_data.maps.insert(
                    m,
                    MapWikiData {
                        name: record.link,
                        _num: m,
                        stages: Vec::new(),
                    },
                );
            }
            (t, Some(m), Some(s)) => {
                // stage
                let map = &mut map[t as usize]
                    .as_mut()
                    .unwrap_or_else(|| panic!("Stage type {t:03} not found"))
                    .maps
                    .get_mut(&m);
                let map_data = map.as_mut().unwrap_or_else(|| {
                    panic!("Map {m} not found when attempting to insert stage {s}")
                });
                let stages = &mut map_data.stages;

                assert_eq!(
                    s,
                    u32::try_from(stages.len()).expect("u32 should be big enough"),
                    "Error parsing stage names record {record:?}: data is out of order."
                );

                stages.push(StageWikiData {
                    name: record.link,
                    _num: s,
                });
            }
            r => panic!("Unexpected line found when getting stage names: {r:?}"),
        }
    }

    map
}

// one day I may have to reformat this entire thing so that instead of just
// `expect`ing these have proper errors, but low priority as if panic is hit at
// runtime the user probably did something wrong

fn get_continue_stages_map() -> ContinueStagesMap {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(get_file_location(&FileLocation::WikiData).join("ContinueStages.csv"));

    // expect is fine since this is static
    rdr.expect("couldn't open continue stages map")
        .deserialize::<ContinueStagesLine>()
        .map(|c| {
            let c = c.ok()?;
            Some((c.type_num, c.map_num))
        })
        .collect()
}

fn get_stage_difficulty_map() -> StageDifficultyMap {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_path(get_file_location(&FileLocation::WikiData).join("Difficulty.txt"));

    // expect is fine since this is static
    rdr.expect("couldn't open stage difficulty map")
        .deserialize::<StageDifficultyLine>()
        .map(|d| {
            let d = d.expect("invalid stage difficulty line");
            (d.stage_id, d.difficulty)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::stage::parsed::stage::Stage;
    use StageVariantID as T;

    #[test]
    fn test_parse_succeeds() {
        let _ = get_stage_name_map();
        let _ = get_continue_stages_map();
        let _ = get_stage_difficulty_map();
    }

    #[test]
    fn assert_continue_stages_name_is_correct() {
        let mut max_index = 0;
        for i in 0.. {
            let d = Stage::from_id_current(StageID::from_components(T::Extra, i, 0));
            match d {
                None => break,
                Some(_) => max_index = i,
            }
        }
        let rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(get_file_location(&FileLocation::WikiData).join("ContinueStages.csv"));

        let mut i = 0;
        for record in rdr.unwrap().records() {
            let result = record.as_ref().unwrap();
            let continue_map_name = &result[0];

            let map_data = STAGE_WIKI_DATA
                .stage_map(&MapID::from_numbers(4, i))
                .unwrap_or_else(|| panic!("Map name data does not exist for ex map {i}."));
            let real_map_name = &map_data.name;

            assert_eq!(real_map_name, continue_map_name, "Error on map {i}:");
            i += 1;
        }

        assert_eq!(max_index, i - 1, "Not all ex stages are in ContinueStages!");
    }
}
