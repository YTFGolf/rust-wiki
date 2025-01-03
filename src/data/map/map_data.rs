//! Module that deals with getting information about stage maps.

use crate::data::{stage::raw::stage_metadata::StageMeta, version::Version};
use csv::ByteRecord;
use csv_types::{ScoreRewardsCSV, StageDataCSV, StageInfoCSVFixed, TreasureCSV};
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

/// Types to deserialise csv files.
pub mod csv_types {
    // #[derive(Debug, serde::Deserialize)]
    // #[allow(dead_code, missing_docs)]
    // /// No real clue.
    // // mapnum?,treasuredrop,scorerewards,?,?
    // pub struct HeaderCSV {
    //     map_file_num: i32,
    // // "itemsetting" according to clamchowder
    //     _unknown_1: i32,
    // // Something to do with score rewards
    //     _unknown_2: i32,
    // // next 2 are something to do with unlock conditions
    //     _unknown_3: i32,
    //     _unknown_4: i32,

    // 9,-1,-1,137000,137000
    // Means map 9, and must complete map 37000 to unlock
    // mapcondition and stagecondition are the 137000s according to clamchowder

    // アイテム報酬型ステージ設定(-1:OFF),　スコア報酬型ステージ設定(-1:OFF)

    // }

    // not important probably
    // #[derive(Debug, serde::Deserialize)]
    // #[allow(dead_code, missing_docs)]
    // pub struct Line2CSV {}

    /*
     * Other things:
     * - stage_conditions.csv for Labyrinth
     * - DataLocal/SpecialRulesMap.json for Colosseum
     * - stage_hint_popup.csv for tutorial
     */

    #[derive(Debug, serde::Deserialize)]
    /// All fixed data stored in the map file. Can reliably be deserialised
    /// using serde.
    pub struct StageInfoCSVFixed {
        /// Energy to challenge stage.
        pub energy: u32,
        /// Base XP rewarded.
        pub xp: u32,
        /// Music track played at beginning of stage.
        _init_track: u32,
        /// Base percentage where music changes to
        /// [_second_track][StageInfoCSVFixed::_second_track].
        _base_drop: u32,
        /// Music track played when base hp goes below
        /// [_base_drop][StageInfoCSVFixed::_base_drop].
        _second_track: u32,
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    /// CSV data related to stage treasures.
    pub struct TreasureCSV {
        /// Chance the item will drop.
        pub item_chance: u32,
        /// ID of item.
        pub item_id: u32,
        /// Amount of the item that drops.
        pub item_amt: u32,
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    /// CSV data related to timed score rewards.
    pub struct ScoreRewardsCSV {
        /// Score required to get item.
        pub score: u32,
        /// ID of item.
        pub item_id: u32,
        /// Amount of the item that drops.
        pub item_amt: u32,
    }

    #[derive(Debug, PartialEq)]
    /// Treasure drop reward modifier.
    ///
    /// All descriptions are purely speculative based on BCU code; if you have
    /// access to the game you may want to actually check what is said here.
    pub enum TreasureType {
        /// E.g. Merciless XP: first item is only available once. After that
        /// works exactly the same as
        /// [AllUnlimited][TreasureType::AllUnlimited].
        OnceThenUnlimited = 1,
        /// Default e.g. Catfruit Jubilee.
        ///
        /// E.g. if you have (50, 50, 50) as the chances then the effective
        /// chances are (50, 25, 12.5).
        AllUnlimited = 0,
        /// Appears to just be a single unlimited raw value. Difference between
        /// this and [AllUnlimited][TreasureType::AllUnlimited] is unclear.
        ///
        /// There are no occurrences of this value being used on stages with
        /// multiple treasure rewards as of 13.6.0.
        UnclearMaybeRaw = -1,
        /// Guaranteed item once e.g. any stage in Infernal Tower. Can't use a
        /// Treasure Radar to get any items.
        ///
        /// If has multiple items then each item's chance is (`item_chance` /
        /// total sum). The exact mechanism is unclear but this seems to be the
        /// case.
        GuaranteedOnce = -3,
        /// Same as [GuaranteedOnce][TreasureType::GuaranteedOnce] but with
        /// unlimited rewards.
        GuaranteedUnlimited = -4,
    }

    impl From<i32> for TreasureType {
        fn from(treasure_type: i32) -> Self {
            match treasure_type {
                1 => TreasureType::OnceThenUnlimited,
                0 => TreasureType::AllUnlimited,
                -1 => TreasureType::UnclearMaybeRaw,
                -3 => TreasureType::GuaranteedOnce,
                -4 => TreasureType::GuaranteedUnlimited,
                _ => panic!("{treasure_type} is not recognised!"),
            }
        }
    }
    // 1 = first item is once, rest are as in 0
    // 0 = default: e.g. Catfruit Jubilee
    // -1 = unclear, seems to be unlimited and raw percentages
    // -3 = One of the following (1 time). Chances are `item_chance` / total
    // -4 = No treasure radar, additive chances same as -3.

    #[derive(Debug)]
    /// Container struct for all of the data for an individual stage.
    pub struct StageDataCSV {
        /// Data that is always fixed into the csv.
        pub fixed_data: StageInfoCSVFixed,
        /// Modifier for the treasure drop.
        pub treasure_type: TreasureType,
        /// Raw treasure drop data.
        pub treasure_drop: Vec<TreasureCSV>,
        /// Raw score rewards data.
        pub score_rewards: Vec<ScoreRewardsCSV>,
    }
}

/// Currently does nothing.
pub struct GameMap {}

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
        if split_line.ends_with(",") {
            split_line = &split_line[0..split_line.len() - 1]
            // remove final bit since parse function relies on it
        }

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // .flexible(true)
            .from_reader(Cursor::new(split_line));
        let stage_line = rdr.byte_records().next().unwrap().unwrap();

        Some(Self::parse_stage_line(stage_line))
    }

    fn parse_stage_line(record: ByteRecord) -> StageDataCSV {
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
