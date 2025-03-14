//! Module that deals with the `Stage_option` file.

use crate::{
    data::version::version_data::CacheableVersionData,
    meta::stage::{map_id::MapID, stage_id::StageID},
};
use std::{collections::HashMap, path::Path};

/// Module that contains charagroup information.
pub mod charagroups {
    use crate::data::version::version_data::CacheableVersionData;
    use std::path::Path;

    #[derive(Debug, serde::Deserialize)]
    /// Fixed csv data in Charagroup.csv.
    struct CharaGroupFixedCSV {
        /// ID of charagroup.
        group_id: u32,
        /// Basically just `stage_restriction_charagroup_{group_id}`.
        _text_id: String,
        /// 0 = Can only use, 2 = can't use
        group_type: u32,
    }

    #[derive(Debug, PartialEq, Clone)]
    /// Type of the Charagroup.
    pub enum CharaGroupType {
        /// Can only use select cats.
        OnlyUse,
        /// Cannot use select cats.
        CannotUse,
    }

    impl From<u32> for CharaGroupType {
        fn from(value: u32) -> Self {
            match value {
                0 => Self::OnlyUse,
                2 => Self::CannotUse,
                _ => panic!("Value {value} is not recognised as a valid charagroup!"),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    /// Data about a CharaGroup.
    pub struct CharaGroup {
        /// Type of charagroup.
        pub group_type: CharaGroupType,
        /// Units in charagroup (Cat = 0).
        pub units: Vec<u32>,
    }

    #[derive(Debug)]
    /// Container for charagroups data.
    pub struct CharaGroups {
        parsed_file: Vec<CharaGroup>,
    }
    impl CharaGroups {
        /// Get charagroup with id `id`.
        pub fn get_charagroup(&self, id: u32) -> Option<&CharaGroup> {
            self.parsed_file.get(usize::try_from(id - 1).unwrap())
        }
    }
    impl CacheableVersionData for CharaGroups {
        fn init_data(path: &Path) -> Self {
            Self {
                parsed_file: read_charagroup_file(path),
            }
        }
    }

    /// Reads the charagroup file and passes it into a vec of
    /// [CharaGroups][CharaGroup].
    fn read_charagroup_file(path: &Path) -> Vec<CharaGroup> {
        let path = path.join("DataLocal/Charagroup.csv");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)
            .unwrap();

        let mut records = rdr.byte_records();
        records.next();

        let mut count = 0;
        records
            .map(|record| {
                let result = record.unwrap();
                let fixed_data: CharaGroupFixedCSV = result.deserialize(None).unwrap();

                count += 1;
                debug_assert_eq!(count, fixed_data.group_id);

                let max_ind = if result[result.len() - 1].is_empty() {
                    result.len() - 1
                } else {
                    result.len()
                };

                let mut units: Vec<u32> = vec![];
                for i in 3..max_ind {
                    let n = std::str::from_utf8(&result[i])
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    units.push(n);
                }

                CharaGroup {
                    group_type: fixed_data.group_type.into(),
                    units,
                }
            })
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
/// Data stored in the stage option CSV. Details the restrictions on individual
/// stages within the map.
///
/// If map has multiple restrictions it will have multiple entries in the file.
/// If any restriction field is 0 then that restriction does not apply.
pub struct StageOptionCSV {
    /// Stage's map's mapid.
    pub mapid: u32,
    /// Crown difficulties that restriction applies to. -1 = all crowns,
    /// otherwise it's just 0-based.
    pub stars: i8,
    /// If is -1 then applies to all stages in map. Otherwise only applies to
    /// the stage in the map with that id.
    pub stage_id: i32,
    /// Rarities allowed. Binary value.
    pub rarity: u8,
    /// Cat deploy limit.
    pub deploy_limit: u32,
    /// Rows that you can deploy from.
    pub rows: u8,
    /// Minimum unit cost.
    pub min_cost: u32,
    /// Maximum unit cost.
    pub max_cost: u32,
    /// [CharaGroup][charagroups::CharaGroup] id.
    pub charagroup: u32,
}

#[derive(Debug)]
/// Container for stage option data.
pub struct StageOption {
    map: HashMap<u32, Vec<StageOptionCSV>>,
}
impl StageOption {
    /// Get the data for the map that `map_id` corresponds to.
    pub fn get_map(&self, map_id: &MapID) -> Option<&Vec<StageOptionCSV>> {
        self.map.get(&map_id.mapid())
    }

    /// Get all restrictions in the map where either the entire map has a
    /// restriction or that specific stage has a restriction.
    #[allow(clippy::cast_possible_wrap)]
    pub fn get_stage(&self, stage_id: &StageID) -> Option<Vec<&StageOptionCSV>> {
        Some(
            self.get_map(stage_id.map())?
                .iter()
                .filter(move |stage| [-1, stage_id.num() as i32].contains(&stage.stage_id))
                .collect(),
        )
    }
}
impl CacheableVersionData for StageOption {
    fn init_data(path: &Path) -> Self {
        Self {
            map: get_stage_option(path),
        }
    }
}

fn get_stage_option(path: &Path) -> HashMap<u32, Vec<StageOptionCSV>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        // technically does have headers but that's an issue for another day
        .flexible(true)
        .from_path(path.join("DataLocal/Stage_option.csv"))
        .unwrap();

    let mut records = rdr.byte_records();
    records.next();

    let mut map: HashMap<u32, Vec<StageOptionCSV>> = HashMap::new();
    // Since sorting by stage id will require looking at another field might as
    // well just convert everything to a [StageOptionCSV] anyway.
    for record in records {
        let result: StageOptionCSV = record.unwrap().deserialize(None).unwrap();
        let entry = map.get_mut(&result.mapid);
        match entry {
            Some(map_option) => map_option.push(result),
            None => {
                map.insert(result.mapid, vec![result]);
            }
        };
    }

    map
}
