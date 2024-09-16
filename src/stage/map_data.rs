use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

use csv::ByteRecord;
use csv_types::{Rand, ScoreRewardsCSV, StageDataCSV, StageInfoCSVFixed, TreasureCSV};

use crate::file_handler::{get_file_location, FileLocation::GameData};

use super::stage_metadata::StageMeta;

/// Types to deserialise csv files.
pub mod csv_types {
    // #[derive(Debug, serde::Deserialize)]
    // #[allow(dead_code, missing_docs)]
    // /// No real clue.
    // // mapnum?,treasuredrop,scorerewards,?,?
    // pub struct HeaderCSV {
    //     map_file_num: i32,
    //     _unknown_1: i32,
    //     _unknown_2: i32,
    //     _unknown_3: i32,
    //     _unknown_4: i32,
    // }

    // not important probably
    // #[derive(Debug, serde::Deserialize)]
    // #[allow(dead_code, missing_docs)]
    // pub struct Line2CSV {}

    #[derive(Debug, serde::Deserialize)]
    /// All fixed data stored in the map file.
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
        // TODO determine what happens with this on Dojo stages.
        _second_track: u32,
    }

    #[derive(Debug, serde::Deserialize)]
    /// CSV data related to stage treasures.
    pub struct TreasureCSV {
        /// Chance the item will drop.
        pub item_chance: u32,
        /// Numerical value of the item.
        pub item_num: u32,
        /// Amount of the item that drops.
        pub item_amt: u32,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code, missing_docs)]
    pub struct ScoreRewardsCSV {
        pub score: u32,
        pub item_id: u32,
        pub itemquant: u32,
    }

    #[derive(Debug)]
    /// Drop reward modifier.
    ///
    /// All descriptions are purely speculative based on BCU code; if you have
    /// access to the game you may want to actually check what is said here.
    pub enum Rand {
        /// E.g. Merciless XP: first item is only available once
        FirstThenUnlimited,
        /// Default e.g. Catfruit Jubilee.
        AllUnlimited,
        /// Appears to just be a single unlimited raw value. Difference between
        /// this and [AllUnlimited][Rand::AllUnlimited] is unclear.
        UnclearMaybeRaw,
        /// Guaranteed item e.g. any stage in Infernal Tower.
        ///
        /// If has multiple items then each item's chance is `item_chance` /
        /// 100.
        Guaranteed,
        /// Same as [Guaranteed][Rand::Guaranteed] but without the possibility
        /// of treasure radar.
        GuaranteedNoTreasureRadar,
    }
    // 1 = first item is once, rest are as in 0
    // 0 = default: e.g. Catfruit Jubilee
    // -1 = unclear, seems to be unlimited and raw percentages
    // -3 = One of the following (1 time). Chances are `item_chance` / total
    // -4 = No treasure radar, additive chances same as -3.

    #[derive(Debug)]
    #[allow(dead_code, missing_docs)]
    pub struct StageDataCSV {
        pub fixed: StageInfoCSVFixed,
        pub rand: Rand,
        pub treasure: Vec<TreasureCSV>,
        pub rewards: Vec<ScoreRewardsCSV>,
    }
}

/// Currently does nothing.
pub struct GameMap {}

impl GameMap {
    /// Just get the stage data, don't care for anything else the map can offer.
    ///
    /// If you get [None] then the stage doesn't have proper rewards, e.g.
    /// Labyrinth stages above 100.
    pub fn get_stage_data(md: StageMeta) -> Option<StageDataCSV> {
        let map_file = get_file_location(GameData)
            .join("DataLocal")
            .join(&md.map_file_name);
        let line = BufReader::new(File::open(map_file).unwrap())
            .lines()
            .skip(2)
            .nth(md.stage_num.try_into().unwrap())?
            .unwrap();

        let mut split_line = line.split("//").next().unwrap().trim();
        if split_line == "" {
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

        Self::parse_stage_line(stage_line)
    }

    fn parse_stage_line(record: ByteRecord) -> Option<StageDataCSV> {
        let fixed_data: StageInfoCSVFixed = record.deserialize(None).unwrap();

        // let once = match record[record.len() - 1] {
        //     [] => &record[record.len() - 2],
        //     _ => &record[record.len() - 1],
        // };

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

        let parse_i32 = |slice| {
            std::str::from_utf8(slice)
                .expect("Invalid UTF-8")
                .parse::<i32>()
                .expect("Unable to parse to i32")
        };
        let parse_u32 = |slice| {
            std::str::from_utf8(slice)
                .expect("Invalid UTF-8")
                .parse::<u32>()
                .expect("Unable to parse to u32")
        };

        let time: Vec<ScoreRewardsCSV> = if is_time {
            let time_len = (record.len() - 17) / 3;
            let mut time = vec![];
            for i in 0..time_len {
                time.push(ScoreRewardsCSV {
                    score: parse_u32(&record[16 + i * 3 + 0]),
                    item_id: parse_u32(&record[16 + i * 3 + 1]),
                    itemquant: parse_u32(&record[16 + i * 3 + 2]),
                });
            }

            time
        } else {
            vec![]
        };

        let is_multi = !is_time && record.len() > 9;

        let rand: i32;
        let drop: Vec<TreasureCSV> = if record.len() == 6 {
            rand = 0;
            vec![]
        } else if !is_multi {
            rand = 0;
            vec![TreasureCSV {
                item_chance: parse_u32(&record[5]),
                item_num: parse_u32(&record[6]),
                item_amt: parse_u32(&record[7]),
            }]
        } else {
            let drop_len = (record.len() - 7) / 3;
            let mut drop = vec![];
            drop.push(TreasureCSV {
                item_chance: parse_u32(&record[5]),
                item_num: parse_u32(&record[6]),
                item_amt: parse_u32(&record[7]),
            });
            rand = parse_i32(&record[8]);
            for i in 1..drop_len {
                drop.push(TreasureCSV {
                    item_chance: parse_u32(&record[6 + i * 3 + 0]),
                    item_num: parse_u32(&record[6 + i * 3 + 1]),
                    item_amt: parse_u32(&record[6 + i * 3 + 2]),
                });
            }

            drop
        };

        let rand_enum = match rand {
            1 => Rand::FirstThenUnlimited,
            0 => Rand::AllUnlimited,
            -1 => Rand::UnclearMaybeRaw,
            -3 => Rand::Guaranteed,
            -4 => Rand::GuaranteedNoTreasureRadar,
            _ => panic!("{rand} is not recognised!"),
        };

        Some(StageDataCSV {
            fixed: fixed_data,
            treasure: drop,
            rewards: time,
            rand: rand_enum,
        })
        // Rand values:
        // 1 = first item is once, rest are as in 0
        // 0 = default: e.g. Catfruit Jubilee
        // -1 = unclear, seems to be unlimited and raw percentages
        // -3 = One of the following (1 time). Chances are `item_chance` / total
        // -4 = No treasure radar, additive chances same as -3.
        // maybe other
        // else {
        //     for(int[] d : drop) {
        //         res.add(String.valueOf(d[0]));
        //     }
        // }
        // getDropData
        // getDropChances

        // if(i == 0 && (info.rand == 1 || (info.drop[i][1] >= 1000 && info.drop[i][1] < 30000)))
        //     builder.append(LangID.getStringByID("data.stage.reward.once", lang));

        // if(i == 0 && info.drop[i][0] != 100 && info.rand != -4 && !chances.isEmpty())
        //     builder.append(EmojiStore.TREASURE_RADAR.getFormatted());

        // if(i == 0 && (info.rand == 1 || (info.drop[i][1] >= 1000 && info.drop[i][1] < 30000)))
        //     reward += " " + LangID.getStringByID("data.stage.reward.once", lang);
    }
    // https://github.com/battlecatsultimate/BCU_java_util_common/commits/slow_kotlin/util/stage/info/DefStageInfo.java

    pub fn read_stage_csv<R: std::io::Read>(reader: R) {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            // .from_reader(File::open(gd.join("DataLocal/stage00.csv")).unwrap())
            // .from_reader(File::open(gd.join("DataLocal/stage.csv")).unwrap())
            .from_reader(reader);

        let records = rdr.byte_records();
        for result in records {
            println!("{result:?}");
        }

        // let mut head = records.next().unwrap().unwrap();
        // let csv_head: HeaderCSV = if head.len() <= 7 || head[6].is_empty() {
        //     let tmp = head;
        //     head = records.next().unwrap().unwrap();
        //     tmp.deserialize(None).unwrap()
        // } else {
        //     // In EoC
        //     HeaderCSV {
        //         base_id: 0,
        //         no_cont: 0,
        //         cont_chance: 0,
        //         contmap_id: 0,
        //         cont_stage_idmin: 0,
        //         cont_stage_idmax: 0,
        //     }
        //     // ByteRecord::from(vec!["0", "0", "0", "0", "0", "0", ""])
        //     //     .deserialize(None)
        //     //     .unwrap()
        // };
        // let line_2 = head;
        // let csv_line_2: Line2CSV = line_2.deserialize(None).unwrap();

        // println!("{csv_head:?}");
        // println!("{csv_line_2:?}");

        // for result in rdr.byte_records() {
        //     let record: StageEnemyCSV = result.unwrap().deserialize(None).unwrap();
        //     if record.num == 0 {
        //         break;
        //     }
        //     println!("{:?}", record);
        // }
    }
}
