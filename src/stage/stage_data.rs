//! Module that deals with getting information about stages.
use super::stage_metadata::StageMeta;
use crate::file_handler::get_decommented_file_reader;
use csv_types::*;
use std::path::PathBuf;

/// Types to deserialise csv files.
pub mod csv_types {
    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code, missing_docs)]
    pub struct HeaderCSV {
        pub base_id: u32,
        pub no_cont: u32,
        pub cont_chance: u32,
        pub contmap_id: u32,
        pub cont_stage_idmin: u32,
        pub cont_stage_idmax: u32,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code, missing_docs)]
    pub struct Line2CSV {
        pub width: u32,
        pub base_hp: u32,
        pub _unknown_1: u32,
        pub _unknown_2: u32,
        pub background_id: u32,
        pub max_enemies: u32,
        pub animbase_id: u32,
        pub time_limit: u32,
        pub indestructible: u32,
        pub _unknown_3: Option<u32>,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code, missing_docs)]
    pub struct StageEnemyCSV {
        pub num: u32,
        pub amt: u32,
        pub start_frame: u32,
        pub respawn_frame_min: u32,
        pub respawn_frame_max: u32,
        pub base_hp: u32,
        pub layer_min: u32,
        pub layer_max: u32,
        pub boss_type: u32,
        pub magnification: Option<u32>,

        #[serde(default)]
        pub _unknown_1: Option<u32>,
        #[serde(default)]
        pub attack_magnification: Option<u32>,
        #[serde(default)]
        pub is_spawn_delay: Option<u32>,
        #[serde(default)]
        pub kill_count: Option<u32>,
    }
}

pub struct Stage {}

impl Stage {
    pub fn new(selector: &str) {
        let md = StageMeta::new(selector).unwrap();

        let stage_file = PathBuf::from("DataLocal").join(md.stage_file_name);
        let stage_file_reader = get_decommented_file_reader(stage_file).unwrap();
        let stage_data = Self::read_stage_csv(stage_file_reader);

        ()
    }

    pub fn read_stage_csv<R: std::io::Read>(reader: R) {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            // .from_reader(File::open(gd.join("DataLocal/stage00.csv")).unwrap())
            // .from_reader(File::open(gd.join("DataLocal/stage.csv")).unwrap())
            .from_reader(reader);

        let mut records = rdr.byte_records();

        let mut head = records.next().unwrap().unwrap();
        let csv_head: HeaderCSV = if head.len() <= 7 || head[6].is_empty() {
            let tmp = head;
            head = records.next().unwrap().unwrap();
            tmp.deserialize(None).unwrap()
        } else {
            // In EoC
            HeaderCSV {
                base_id: 0,
                no_cont: 0,
                cont_chance: 0,
                contmap_id: 0,
                cont_stage_idmin: 0,
                cont_stage_idmax: 0,
            }
            // ByteRecord::from(vec!["0", "0", "0", "0", "0", "0", ""])
            //     .deserialize(None)
            //     .unwrap()
        };
        let line_2 = head;
        let csv_line_2: Line2CSV = line_2.deserialize(None).unwrap();

        println!("{csv_head:?}");
        println!("{csv_line_2:?}");

        for result in rdr.byte_records() {
            let record: StageEnemyCSV = result.unwrap().deserialize(None).unwrap();
            if record.num == 0 {
                break;
            }
            println!("{:?}", record);
        }
    }
}

// /// temp
// pub fn read_csv_file(file_name: &str) {
//     Stage::read_stage_csv(get_decommented_file_reader(file_name).unwrap());
//     // check all stage files ig
//     // Encounters: check the head, if needs to be nexted then next it
//     // do split(',').next()
//     // if matches string version of target then do serde
//     // if is "0" then break
//     // Could make a tester that checks Ms. Sign with the idiomatic and the
//     // efficient way of doing it.
//     // Would need to benchmark it though.
//     // ByteRecord::from(thing.split().collect())
//     // Could even do just checking id, mag, amag
// }
