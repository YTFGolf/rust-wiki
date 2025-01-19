//! Module that deals with getting information about stage maps.

use super::{
    ex_option::ExOption,
    map_option::{MapOption, MapOptionCSV},
    raw::csv_types::{ScoreRewardsCSV, StageDataCSV, StageInfoCSVFixed, TreasureCSV},
    special_rules::{SpecialRule, SpecialRules},
};
use crate::data::{
    stage::raw::{
        stage_metadata::{consts::StageTypeEnum, StageMeta},
        stage_option::{StageOption, StageOptionCSV},
    },
    version::Version,
};
use csv::ByteRecord;
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

/// Currently stores nothing.
pub struct GameMap {}

// Stage-related.
impl GameMap {
    /// Just get the stage data, don't care for anything else the map can offer.
    ///
    /// If you get [None] then the stage doesn't have proper rewards, e.g.
    /// Labyrinth stages above 100.
    pub fn get_stage_data(md: &StageMeta, v: &Version) -> Option<StageDataCSV> {
        let map_file = v.get_file_path("DataLocal").join(&md.map_file_name);
        let line = BufReader::new(File::open(map_file).unwrap())
            .lines()
            .skip(2)
            .nth(md.stage_num.try_into().unwrap())?
            .unwrap();

        let mut split_line = line.split("//").next().unwrap().trim();
        if split_line.is_empty() {
            return None;
        }
        if split_line.ends_with(',') {
            split_line = &split_line[0..split_line.len() - 1];
            // remove final bit since parse function relies on it
        }

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // .flexible(true)
            .from_reader(Cursor::new(split_line));
        let stage_line = rdr.byte_records().next().unwrap().unwrap();

        Some(Self::parse_stage_line(&stage_line))
    }

    fn parse_stage_line(record: &ByteRecord) -> StageDataCSV {
        // https://github.com/battlecatsultimate/BCU_java_util_common/commits/slow_kotlin/util/stage/info/DefStageInfo.java

        let fixed_data: StageInfoCSVFixed = record.deserialize(None).unwrap();

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
            let mut drop = vec![];
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

impl GameMap {
    /// Get `map_id` to use in map_option and stage_option.
    fn get_map_id(meta: &StageMeta) -> u32 {
        let m = meta;
        m.type_num * 1000 + m.map_num
    }

    /// Get MapStageData data for the stage if it exists.
    pub fn get_map_stage_data(meta: &StageMeta, version: &Version) -> Option<StageDataCSV> {
        if meta.type_enum == StageTypeEnum::Labyrinth {
            return None;
        }
        GameMap::get_stage_data(meta, version)
    }

    /// Get Map_option data if it exists.
    pub fn get_map_option_data(meta: &StageMeta, version: &Version) -> Option<MapOptionCSV> {
        let map_id = Self::get_map_id(meta);
        let map_option = version.get_cached_file::<MapOption>();
        map_option.get_map(map_id)
    }

    /// Get Stage_option data for the stage (and whole map) if it exists.
    pub fn get_stage_option_data<'a>(
        meta: &StageMeta,
        version: &'a Version,
    ) -> Option<Vec<&'a StageOptionCSV>> {
        let map_id = Self::get_map_id(meta);
        let stage_option = version.get_cached_file::<StageOption>();
        stage_option.get_stage(map_id, meta.stage_num)
    }

    /// Get Map_option data if it exists.
    pub fn get_ex_option_data(meta: &StageMeta, version: &Version) -> Option<u32> {
        let map_id = Self::get_map_id(meta);
        let ex_option = version.get_cached_file::<ExOption>();
        ex_option.get_ex_map(map_id)
    }

    /// Get SpecialRulesMap data if it exists.
    pub fn get_special_rules_data<'a>(
        meta: &StageMeta,
        version: &'a Version,
    ) -> Option<&'a SpecialRule> {
        let map_id = Self::get_map_id(meta);
        let special_rules = version.get_cached_file::<SpecialRules>();
        special_rules.get_map(map_id)
    }
}
