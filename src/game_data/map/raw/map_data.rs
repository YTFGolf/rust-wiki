//! Module that deals with getting information about stage maps.

use super::super::{
    cacheable::{
        drop_item::{DropItem, DropItemRaw},
        ex_option::ExOption,
        map_option::{MapOption, MapOptionCSV},
        special_rules::{SpecialRule, SpecialRules},
    },
    raw::csv_types::{HeaderCSV, ScoreRewardsCSV, StageDataCSV, StageInfoCSVFixed, TreasureCSV},
};
use crate::game_data::{
    meta::stage::{
        map_id::MapID, stage_id::StageID, stage_types::transform::transform_map::map_data_file,
        variant::StageVariantID,
    },
    stage::raw::stage_option::{StageOption, StageOptionCSV},
    version::Version,
};
use csv::ByteRecord;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Stage map.
pub struct GameMapData {
    /// Background image of the map.
    pub map_file_num: i32,
}

impl GameMapData {
    /// Create new [`GameMapData`] object.
    pub fn new(map: &MapID, v: &Version) -> Self {
        let map_file = v.get_file_path("DataLocal").join(map_data_file(map));
        let lines = BufReader::new(File::open(map_file).unwrap());

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // .flexible(true)
            .from_reader(lines);

        let header = rdr.byte_records().next().unwrap().unwrap();

        let header: HeaderCSV = header.deserialize(None).unwrap();
        Self {
            map_file_num: header.map_file_num,
        }
    }
}

// Stage-related.
impl GameMapData {
    /// Just get the stage data, don't care for anything else the map can offer.
    ///
    /// If you get [None] then the stage doesn't have proper rewards, e.g.
    /// Labyrinth stages above 100.
    fn stage_data(stage: &StageID, v: &Version) -> Option<StageDataCSV> {
        let map_file = v
            .get_file_path("DataLocal")
            .join(map_data_file(stage.map()));
        let line = BufReader::new(File::open(map_file).unwrap())
            .lines()
            .skip(2)
            .nth(stage.num().try_into().unwrap())?
            .unwrap();

        let split_line = line
            .split("//")
            .next()
            .expect("Shouldn't panic on first next.")
            .trim_matches(|c: char| c.is_whitespace() || c == ',');

        if split_line.is_empty() {
            return None;
        }

        let stage_line = split_line.split(',').collect::<ByteRecord>();
        // assuming that there is no quoting

        Some(Self::parse_stage_line(&stage_line))
    }

    fn parse_stage_line(record: &ByteRecord) -> StageDataCSV {
        // https://github.com/battlecatsultimate/BCU_java_util_common/blob/slow_kotlin/util/stage/info/DefStageInfo.java#L36

        let fixed_data: StageInfoCSVFixed = record
            .deserialize(None)
            .expect("Couldn't deserialise stage line.");

        let _once = &record[record.len() - 1];
        // what does this actually do

        let mut is_time = record.len() > 15;
        if is_time {
            for i in 8..15 {
                if &record[i] != b"-2" {
                    is_time = false;
                    break;
                }
            }
        }
        let is_time = is_time;

        let parse_i32 = |slice| std::str::from_utf8(slice).unwrap().parse::<i32>().unwrap();
        let parse_u32 = |slice| std::str::from_utf8(slice).unwrap().parse::<u32>().unwrap();

        let score_rewards: Vec<ScoreRewardsCSV> = if is_time {
            let time_len = (record.len() - 17) / 3;
            let mut time = vec![];
            for i in 0..time_len {
                time.push(ScoreRewardsCSV {
                    score: parse_u32(&record[16 + i * 3]),
                    item_id: parse_u32(&record[16 + i * 3 + 1]),
                    item_amt: parse_u32(&record[16 + i * 3 + 2]),
                });
            }

            time
        } else {
            vec![]
        };

        let is_multi = !is_time && record.len() > 9;

        let treasure_type: i32;
        let treasure_drop: Vec<TreasureCSV> = if record.len() == 6 {
            treasure_type = 0;
            vec![]
        } else if !is_multi {
            treasure_type = 0;
            vec![TreasureCSV {
                item_chance: parse_u32(&record[5]),
                item_id: parse_u32(&record[6]),
                item_amt: parse_u32(&record[7]),
            }]
        } else {
            let drop_len = (record.len() - 7) / 3;
            let mut drop = Vec::with_capacity(drop_len);
            drop.push(TreasureCSV {
                item_chance: parse_u32(&record[5]),
                item_id: parse_u32(&record[6]),
                item_amt: parse_u32(&record[7]),
            });
            treasure_type = parse_i32(&record[8]);
            for i in 1..drop_len {
                drop.push(TreasureCSV {
                    item_chance: parse_u32(&record[6 + i * 3]),
                    item_id: parse_u32(&record[6 + i * 3 + 1]),
                    item_amt: parse_u32(&record[6 + i * 3 + 2]),
                });
            }

            drop
        };

        StageDataCSV {
            fixed_data,
            treasure_drop,
            score_rewards,
            treasure_type: treasure_type.into(),
        }
    }
}

impl GameMapData {
    /// Get MapStageData data for the stage if it exists.
    pub fn get_stage_data(stage: &StageID, version: &Version) -> Option<StageDataCSV> {
        if stage.variant() == StageVariantID::Labyrinth {
            return None;
        }
        GameMapData::stage_data(stage, version)
    }

    /// Get Map_option data if it exists.
    pub fn get_map_option_data(map: &MapID, version: &Version) -> Option<MapOptionCSV> {
        let map_option = version.get_cached_file::<MapOption>();
        map_option.get_map(map)
    }

    /// Get Stage_option data for the whole map if it exists.
    pub fn map_stage_option_data<'a>(
        map: &MapID,
        version: &'a Version,
    ) -> Option<&'a Vec<StageOptionCSV>> {
        let stage_option = version.get_cached_file::<StageOption>();
        stage_option.get_map(map)
    }

    /// Get Stage_option data for the stage if it exists.
    pub fn stage_stage_option_data<'a>(
        stage: &StageID,
        version: &'a Version,
    ) -> Option<Vec<&'a StageOptionCSV>> {
        let stage_option = version.get_cached_file::<StageOption>();
        stage_option.get_stage(stage)
    }

    /// Get Map_option data if it exists.
    pub fn get_ex_option_data(map: &MapID, version: &Version) -> Option<u32> {
        let ex_option = version.get_cached_file::<ExOption>();
        ex_option.get_ex_map(map)
    }

    /// Get SpecialRulesMap data if it exists.
    pub fn get_special_rules_data<'a>(
        map: &MapID,
        version: &'a Version,
    ) -> Option<&'a SpecialRule> {
        let special_rules = version.get_cached_file::<SpecialRules>();
        special_rules.get_map(map)
    }

    /// Get DropItem data if it exists.
    pub fn get_drop_item<'a>(map: &MapID, version: &'a Version) -> Option<&'a DropItemRaw> {
        let drop_item = version.get_cached_file::<DropItem>();
        drop_item.get_drop_item(map)
    }
}
