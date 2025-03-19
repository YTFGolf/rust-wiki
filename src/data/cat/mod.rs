#![allow(missing_docs, unused_imports, dead_code)]

use super::version::Version;
use csv::{ByteRecord, StringRecord};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
};

type Massive = u32;
type Big = u16;
type Percent = u8;
type Small = u8;
type Bool = u8;

#[derive(Debug, serde::Deserialize)]
pub struct CatCSV {
    hp: Massive,
    kb: Big,
    speed: Small,
    atk: Massive,
    tba: Big,
    range: Big,
    price: Big,
    respawn: Big,
    _uk8: Small,
    width: Big,
    // should always be 320

    // 10
    targ_red: Bool,
    _uk2: Small,
    is_area: Bool,
    foreswing: Big,
    _uk14: Small,
    // "front"
    _uk15: Small,
    // "back"
    targ_float: Bool,
    targ_black: Bool,
    targ_metal: Bool,
    targ_traitless: Bool,

    // 20
    targ_angel: Bool,
    targ_alien: Bool,
    targ_zombie: Bool,
    has_strong: Bool,
    kb_chance: Percent,
    freeze_chance: Percent,
    freeze_time: Big,
    slow_chance: Percent,
    slow_time: Big,
    has_resist: Bool,

    // 30
    has_massive_dmg: Bool,
    crit_chance: Percent,
    has_targets_only: Bool,
    has_double_bounty: Bool,
    has_base_destroyer: Bool,
    wave_chance: Percent,
    wave_level: Small,
    // used for both wave and mini-wave, see `is_mini_wave`
    weaken_chance: Percent,
    weaken_duration: Big,
    weaken_multiplier: Percent,

    // 40
    strengthen_chance: Percent,
    strengthen_multiplier: Big,
    survives_chance: Percent,
    has_metal: Bool,
    ld_min: Big,
    ld_range: i32,
    immune_wave: Bool,
    wave_blocker: Bool,
    immune_kb: Bool,
    immune_freeze: Bool,

    // 50
    immune_slow: Bool,
    immune_weaken: Bool,
}

#[derive(Debug, serde::Deserialize)]
struct CatCSV2 {
    // // index = 52
    // has_zkill: Bool,
    // has_wkill: Bool,
    // _uk54: Small,
    // _uk55: i8,
    // // "loop", appears to be something to do with multihit
    // immune_boss_shockwave: Bool,
    // _uk57: i8,
    // kamikaze: Bool,
    // mhit_atk2: Big,

    // // 60
    // mhit_atk3: Big,
    // mhit_atk2_fswing: Big,
    // mhit_atk3_fswing: Big,
    // proc_on_hit1: Bool,
    // proc_on_hit2: Bool,
    // proc_on_hit3: Bool,
    // _uk66: i8,
    // death: Small,
    // _uk68: Small,
    // _uk69: Small,

    // // 70
    // barrier_break: Percent,
    // _uk71: Small,
    // _uk72: Small,
    // _uk73: Small,
    // _uk74: Small,
    // immune_warp: Percent,
    // _uk76: Small,
    // witch_killer_2: Bool,
    // // ???s
}

fn read_data_file(file_name: &str, version: &Version) {
    let stage_file = PathBuf::from("DataLocal").join(file_name);
    let reader = BufReader::new(File::open(version.get_file_path(&stage_file)).unwrap());

    // let mut rdr = csv::ReaderBuilder::new()
    //     .has_headers(false)
    //     // .flexible(true)
    //     .from_reader(reader);

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line
            .split("//")
            .next()
            .expect("Shouldn't panic on first next.")
            .trim_matches([',', ' ']);
        if line.is_empty() {
            continue;
        }

        let record = ByteRecord::from_iter(line.split(','));
        let cat: CatCSV = ByteRecord::from_iter(record.iter())
            .deserialize(None)
            .unwrap();
        println!("{len} {cat:?}", len = record.len());
    }

    // for result in rdr.byte_records() {
    //     let record = result.unwrap();

    //     if record.len() > 52 {
    //         let a: CatCSV2 = ByteRecord::from_iter(record.iter().skip(52))
    //             .deserialize(None)
    //             .unwrap();
    //         println!("{a:?}");
    //     }
    // }
}

#[test]
fn do_thing() {
    use crate::config::TEST_CONFIG;
    let version = TEST_CONFIG.version.current_version();

    let re = Regex::new(r"^unit\d").unwrap();
    let dir = &version.get_file_path("DataLocal");

    let files = std::fs::read_dir(dir).unwrap();

    let filter = files.filter_map(move |f| {
        let file_name = f.unwrap().file_name().into_string().unwrap();

        if re.is_match(&file_name) {
            Some(file_name)
        } else {
            None
        }
    });

    for file in filter {
        println!("{file}");
        read_data_file(&file, version);
    }
}
