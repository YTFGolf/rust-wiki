use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

use csv::ByteRecord;
use csv_types::{ScoreRewardsCSV, StageInfoCSVFixed, TreasureCSV};

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
    #[allow(dead_code, missing_docs)]
    /// All fixed data stored in the map file.
    pub struct StageInfoCSVFixed {
        energy: u32,
        xp: u32,
        inittrack: u32,
        basedrop: u32,
        secondtrack: u32,
    }
    // energy,xp,inittrack,basedrop,secondtrack,(itemchance?,itemnum?,itemlimit?),?,?,(score,itemID,itemquant)

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code, missing_docs)]
    pub struct TreasureCSV {
        pub itemchance: u32,
        pub itemnum: u32,
        pub itemlimit: u32,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code, missing_docs)]
    pub struct ScoreRewardsCSV {
        pub score: u32,
        pub item_id: u32,
        pub itemquant: u32,
    }
}

pub struct GameMap {}

impl GameMap {
    pub fn get_stage_data(md: StageMeta) {
        println!("Meta object: {:?}", md);
        // println!("Map file: {}", md.map_file_name);
        let map_file = get_file_location(GameData)
            .join("DataLocal")
            .join(&md.map_file_name);
        let line = BufReader::new(File::open(map_file).unwrap())
            .lines()
            .skip(2)
            .nth(md.stage_num.try_into().unwrap())
            .unwrap_or_else(|| panic!("Stage with index {} does not exist!", md.stage_num))
            .unwrap();

        let mut split_line = line.split("//").next().unwrap().trim();
        println!("{split_line:?}");
        if split_line.ends_with(",") {
            split_line = &split_line[0..split_line.len() - 1]
            // remove final bit since parse function relies on it
        }

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // .flexible(true)
            .from_reader(Cursor::new(split_line));
        let stage_line = rdr.byte_records().next().unwrap().unwrap();
        let stage_data = Self::parse_stage_line(stage_line);

        println!("{stage_data:?}");

        ()
    }

    fn parse_stage_line(record: ByteRecord) {
        let fixed_data: StageInfoCSVFixed = record.deserialize(None).unwrap();

        // let once = match record[record.len() - 1] {
        //     [] => &record[record.len() - 2],
        //     _ => &record[record.len() - 1],
        // };

        let once = &record[record.len() - 1];
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

        let time: Vec<ScoreRewardsCSV> = if is_time {
            let time_len = (record.len() - 17) / 3;
            let mut time = vec![];
            for i in 0..time_len {
                time.push(ScoreRewardsCSV {
                    score: parse_i32(&record[16 + i * 3 + 0]) as u32,
                    item_id: parse_i32(&record[16 + i * 3 + 1]) as u32,
                    itemquant: parse_i32(&record[16 + i * 3 + 2]) as u32,
                });
            }

            time
        } else {
            vec![]
        };

        let is_multi = !is_time && record.len() > 9;

        let mut drop: Vec<TreasureCSV>;
        let mut drop: Vec<Vec<i32>>;
        let mut rand: i32;

        if record.len() == 6 {
            drop = Vec::new();
            rand = 0;
        } else if !is_multi {
            drop = vec![vec![]];
            rand = 0;
        } else {
            let drop_len = (record.len() - 7) / 3;
            drop = vec![vec![0; 3]; drop_len];
            rand = parse_i32(&record[8]);
            for i in 1..drop_len {
                for j in 0..3 {
                    drop[i][j] = parse_i32(&record[6 + i * 3 + j]);
                }
            }
        }

        if !drop.is_empty() {
            drop[0] = vec![
                parse_i32(&record[5]),
                parse_i32(&record[6]),
                parse_i32(&record[7]),
            ];
        }

        println!("{fixed_data:?}, {time:?}, {drop:?}");
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
