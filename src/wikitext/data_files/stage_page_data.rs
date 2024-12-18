//! Module that gets information about stage names and continue stages.

use crate::{
    data::stage::raw::stage_metadata::consts::STAGE_TYPES,
    file_handler::{get_file_location, FileLocation},
};
use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug)]
/// Data about all possible stage types.
pub struct TypeData {
    /// Name of the type.
    pub name: String,
    _num: u32,
    maps: HashMap<u32, MapData>,
}
impl TypeData {
    /// Get map with map id `map_id` in this type.
    pub fn get(&self, map_id: u32) -> Option<&MapData> {
        self.maps.get(&map_id)
    }
}

#[derive(Debug)]
/// Data about stage maps.
pub struct MapData {
    /// Name of the map.
    pub name: String,
    _num: u32,
    stages: Vec<StageData>,
}
impl MapData {
    /// Get stage with stage id `stage_id` in this map.
    pub fn get(&self, stage_id: u32) -> Option<&StageData> {
        self.stages.get(stage_id as usize)
    }
}

#[derive(Debug)]
/// Data about individual stages.
pub struct StageData {
    /// Name of the stage.
    pub name: String,
    _num: u32,
}

const MAX_TYPE_ID: usize = STAGE_TYPES[STAGE_TYPES.len() - 1].number as usize;
type StageNameMap = [Option<TypeData>; MAX_TYPE_ID + 1];
type StageDifficultyMap = HashMap<String, u8>;
#[derive(Debug)]
/// Container for [STAGE_NAMES] static.
pub struct StagePageData {
    stage_name_map: LazyLock<StageNameMap>,
    _continue_stages: (),
    stage_difficulty_map: LazyLock<StageDifficultyMap>,
}
impl StagePageData {
    /// Get stage type from stage type id.
    pub fn stage_type(&self, id: u32) -> Option<&TypeData> {
        self.stage_name_map.get(id as usize)?.into()
    }
    /// Get stage map from type and map id.
    pub fn stage_map(&self, type_id: u32, map_id: u32) -> Option<&MapData> {
        self.stage_type(type_id)?.get(map_id)
    }
    /// Get stage from type, map and stage id.
    pub fn stage(&self, type_id: u32, map_id: u32, stage_id: u32) -> Option<&StageData> {
        self.stage_map(type_id, map_id)?.get(stage_id)
    }

    /// Get stage difficulty.
    pub fn difficulty(&self, type_id: u32, map_id: u32, stage_id: u32) -> Option<&u8> {
        self.difficulty_str(&format!("{type_id:03}-{map_id:03}-{stage_id:03}"))
    }
    /// Get stage difficulty.
    fn difficulty_str(&self, id: &str) -> Option<&u8> {
        self.stage_difficulty_map.get(id)
    }
}

/// Contains parsed StageNames.csv file.
// TODO rename
pub static STAGE_NAMES: StagePageData = StagePageData {
    stage_name_map: LazyLock::new(get_stage_name_map),
    _continue_stages: (),
    stage_difficulty_map: LazyLock::new(get_stage_difficulty_map),
};

#[derive(Debug, Deserialize)]
struct StageNamesLine {
    #[serde(rename = "Type")]
    s_type: u32,
    #[serde(rename = "Map")]
    s_map: Option<u32>,
    #[serde(rename = "Stage")]
    s_stage: Option<u32>,
    #[serde(rename = "Link (EN)")]
    s_link: String,
}
#[derive(Debug, Deserialize)]
struct StageDifficultyLine {
    stage_id: String,
    difficulty: u8,
}

fn get_stage_name_map() -> StageNameMap {
    let mut map = [const { None }; MAX_TYPE_ID + 1];

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .comment(Some(b'#'))
        .from_path(get_file_location(FileLocation::WikiData).join("StageNames.csv"));

    for result in rdr.unwrap().deserialize() {
        let record: StageNamesLine = result.unwrap();
        match (record.s_type, record.s_map, record.s_stage) {
            (n, None, None) => {
                map[n as usize] = Some(TypeData {
                    name: record.s_link,
                    _num: n,
                    maps: HashMap::new(),
                })
            }
            (t, Some(m), None) => {
                let type_data = map[t as usize].as_mut().unwrap();
                type_data.maps.insert(
                    m,
                    MapData {
                        name: record.s_link,
                        _num: m,
                        stages: Vec::new(),
                    },
                );
            }
            (t, Some(m), Some(s)) => {
                let map = &mut map[t as usize].as_mut().unwrap().maps.get_mut(&m);
                let map_data = map.as_mut().unwrap_or_else(|| {
                    panic!("Map {m} not found when attempting to insert stage {s}")
                });
                let stages = &mut map_data.stages;

                assert_eq!(
                    s,
                    stages.len() as u32,
                    "Error parsing stage names record {record:?}: data is out of order."
                );

                stages.push(StageData {
                    name: record.s_link,
                    _num: s,
                });
            }
            _ => (),
        }
    }

    map
}

fn get_stage_difficulty_map() -> StageDifficultyMap {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_path(get_file_location(FileLocation::WikiData).join("Difficulty.txt"));

    rdr.unwrap()
        .deserialize::<StageDifficultyLine>()
        .map(|d| {
            let d = d.unwrap();
            (d.stage_id, d.difficulty)
        })
        .collect()
}
