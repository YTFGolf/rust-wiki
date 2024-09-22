//! Module that gets information about stage names.

#![allow(dead_code, missing_docs, unused)]
use crate::{
    data::stage::stage_metadata::consts::STAGE_TYPES,
    file_handler::{get_file_location, FileLocation},
};
use serde::Deserialize;
use std::{collections::HashMap, mem::MaybeUninit, sync::LazyLock};

#[derive(Debug)]
pub struct TypeData {
    name: String,
    num: u32,
    maps: HashMap<u32, MapData>,
}

#[derive(Debug)]
struct MapData {
    name: String,
    num: u32,
    stages: HashMap<u32, StageData>,
}

#[derive(Debug)]
struct StageData {
    name: String,
    num: u32,
}

const MAX_TYPE_ID: usize = STAGE_TYPES[STAGE_TYPES.len() - 1].number as usize;
type StageNameMap = [Option<TypeData>; MAX_TYPE_ID + 1];
#[derive(Debug)]
pub struct StageNames {
    stage_name_map: LazyLock<StageNameMap>,
    continue_stages: (),
}
impl StageNames {
    pub fn stage_type(&self, id: u32) -> Option<&TypeData> {
        match self.stage_name_map.get(id as usize)? {
            None => None,
            Some(t) => Some(&t),
        }
    }
}

pub const STAGE_NAMES: StageNames = StageNames {
    stage_name_map: LazyLock::new(get_stage_name_map),
    continue_stages: (),
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

fn get_stage_name_map() -> StageNameMap {
    let mut map = [const { None }; MAX_TYPE_ID + 1];

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .comment(Some(b'#'))
        .from_path(get_file_location(FileLocation::WikiData).join("StageNames.csv"));

    for result in rdr.unwrap().deserialize() {
        let record: StageNamesLine = result.unwrap();
        match (record.s_type, record.s_map, record.s_stage) {
            (n, None, None) => {
                map[n as usize] = Some(TypeData {
                    name: record.s_link,
                    num: n,
                    maps: HashMap::new(),
                })
            }
            (t, Some(m), None) => {
                let mut type_data = map[t as usize].as_mut().unwrap();
                type_data.maps.insert(
                    m,
                    MapData {
                        name: record.s_link,
                        num: m,
                        stages: HashMap::new(),
                    },
                );
            }
            (t, Some(m), Some(s)) => {
                let mut map = &mut map[t as usize]
                    .as_mut()
                    .unwrap()
                    .maps
                    .get_mut(&m);
                let mut map_data = map
                    .as_mut()
                    .expect(&format!(
                        "Map {m} not found when attempting to insert stage {s}"
                    ));

                map_data.stages.insert(
                    s,
                    StageData {
                        name: record.s_link,
                        num: s,
                    },
                );
            }
            _ => (),
        }
    }

    map
}
