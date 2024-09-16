use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
};

use csv_types::StageInfoCSVFixed;

use crate::file_handler::{get_decommented_file_reader, get_file_location, FileLocation::GameData};

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

        let split_line = line.split("//").next().unwrap().trim();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // .flexible(true)
            .from_reader(Cursor::new(split_line));
        let stage_data: StageInfoCSVFixed = rdr.deserialize().next().unwrap().unwrap();

        println!("{stage_data:?}");

        ()
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
