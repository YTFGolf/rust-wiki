//! Module that deals with getting information about stages.
use std::path::PathBuf;

use crate::file_handler::get_decommented_file_reader;

use super::stage_metadata::StageMeta;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct HeaderCSV {
    base_id: u32,
    no_cont: u32,
    cont_chance: u32,
    contmap_id: u32,
    cont_stage_idmin: u32,
    cont_stage_idmax: u32,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Line2CSV {
    width: u32,
    base_hp: u32,
    unknown_1: u32,
    unknown_2: u32,
    background_id: u32,
    max_enemies: u32,
    animbase_id: u32,
    time_limit: u32,
    indestructible: u32,
    unknown_3: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct StageEnemyCSV {
    num: u32,
    amt: u32,
    start_frame: u32,
    respawn_frame_min: u32,
    respawn_frame_max: u32,
    base_hp: u32,
    layer_min: u32,
    layer_max: u32,
    boss_type: u32,
    magnification: Option<u32>,

    #[serde(default)]
    unknown_1: Option<u32>,
    #[serde(default)]
    attack_magnification: Option<u32>,
    #[serde(default)]
    is_spawn_delay: Option<u32>,
    #[serde(default)]
    kill_count: Option<u32>,
}

pub struct Stage {}

impl Stage {
    pub fn new(selector: &str) {
        let md = StageMeta::new(selector).unwrap();

        let path = PathBuf::from("DataLocal").join(md.stage_file_name);
        let reader = get_decommented_file_reader(path).unwrap();
        let data = Self::read_stage_csv(reader);

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
