//! Module that deals with the `Stage_option` file.

use crate::file_handler::{get_file_location, FileLocation};
use std::{collections::HashMap, sync::LazyLock};

/// Module that contains charagroup information.
pub mod charagroups {
    use crate::file_handler::{get_file_location, FileLocation};
    use std::sync::LazyLock;

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

    #[derive(Debug)]
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

    #[derive(Debug)]
    /// Data about a CharaGroup.
    pub struct CharaGroup {
        /// Type of charagroup.
        pub group_type: CharaGroupType,
        /// Units in charagroup.
        pub units: Vec<u32>,
    }

    /// Container for static data.
    pub struct CharaGroups {
        parsed_file: LazyLock<Vec<CharaGroup>>,
    }
    impl CharaGroups {
        const fn new() -> Self {
            CharaGroups {
                parsed_file: LazyLock::new(read_charagroup_file),
            }
        }

        /// Get charagroup with id `id`.
        pub fn get_charagroup(&self, id: u32) -> Option<&CharaGroup> {
            self.parsed_file.get(usize::try_from(id - 1).unwrap())
        }
    }

    /// If you want group 1 then do `CHARAGROUP.get_charagroup(1)`.
    pub static CHARAGROUP: CharaGroups = CharaGroups::new();

    /// Reads the charagroup file and passes it into a vec of
    /// [CharaGroups][CharaGroup].
    fn read_charagroup_file() -> Vec<CharaGroup> {
        let path = get_file_location(FileLocation::GameData).join("DataLocal/Charagroup.csv");
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
                    units.push(n)
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
#[allow(dead_code)]
/// Data stored in the stage option CSV. Details the restrictions on individual
/// stages within the map.
///
/// If map has multiple restrictions it will have multiple entries in the file.
/// If any restriction field is 0 then that restriction does not apply.
pub struct StageOptionCSV {
    /// Same as [`map_option`'s][crate::map::map_option::MapOptionCSV::map_id].
    pub map_id: u32,
    /// Crown difficulties that restriction applies to. -1 = all crowns,
    /// otherwise it's just 0-based.
    pub stars: i32,
    /// If is -1 then applies to all stages in map. Otherwise only applies to
    /// the stage in the map with that id.
    pub stage_id: i32,
    /// Rarities allowed. Binary value.
    pub rarity: u32,
    /// Cat deploy limit.
    pub deploy_limit: u32,
    /// Rows that you can deploy from.
    pub rows: u32,
    /// Minimum unit cost.
    pub min_cost: u32,
    /// Maximum unit cost.
    pub max_cost: u32,
    /// [CharaGroup][charagroups::CharaGroup] id.
    pub charagroup: u32,
}

/// Container for the [STAGE_OPTION] static.
pub struct StageOption {
    map: LazyLock<HashMap<u32, Vec<StageOptionCSV>>>,
}
impl StageOption {
    const fn new() -> Self {
        Self {
            map: LazyLock::new(get_stage_option),
        }
    }

    /// Get the data for the map that `map_id` corresponds to.
    pub fn get_map(&self, map_id: u32) -> Option<&Vec<StageOptionCSV>> {
        self.map.get(&map_id)
    }

    /// Get all restrictions in the map where either the entire map has a
    /// restriction or that specific stage has a restriction.
    pub fn get_stage(
        &self,
        map_id: u32,
        stage_id: u32,
    ) -> Option<impl Iterator<Item = &StageOptionCSV>> {
        Some(
            self.map
                .get(&map_id)?
                .iter()
                .filter(move |stage| [-1, stage_id as i32].contains(&stage.stage_id)),
        )
    }
}

/// Map of valid `map_id`s to the `"DataLocal/Stage_option.csv"` file.
pub static STAGE_OPTION: StageOption = StageOption::new();

fn get_stage_option() -> HashMap<u32, Vec<StageOptionCSV>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        // technically does have headers but that's an issue for another day
        .flexible(true)
        .from_path(get_file_location(FileLocation::GameData).join("DataLocal/Stage_option.csv"))
        .unwrap();

    let mut records = rdr.byte_records();
    records.next();

    let mut map: HashMap<u32, Vec<StageOptionCSV>> = HashMap::new();
    // Since sorting by stage id will require looking at another field might as
    // well just convert everything to a [StageOptionCSV] anyway.
    for record in records {
        let result: StageOptionCSV = record.unwrap().deserialize(None).unwrap();
        let entry = map.get_mut(&result.map_id);
        match entry {
            Some(map_option) => map_option.push(result),
            None => {
                map.insert(result.map_id, vec![result]);
            }
        };
    }

    map
}
