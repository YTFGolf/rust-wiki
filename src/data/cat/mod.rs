#![allow(missing_docs, unused_imports, dead_code)]

use super::version::Version;
use csv::{ByteRecord, StringRecord};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

type Massive = u32;
type Big = u16;
type Percent = u8;
type Small = u8;
type Bool = u8;

type OpMassive = Option<u32>;
type OpBig = Option<u16>;
type OpPercent = Option<u8>;
type OpSmall = Option<u8>;
type OpBool = Option<u8>;

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
    ld_base: Big,
    ld_range: i32,
    // if ld_range is neg then is omnistrike, need to consult the hitbox page
    immune_wave: Bool,
    wave_blocker: Bool,
    immune_kb: Bool,
    immune_freeze: Bool,

    // 50
    immune_slow: Bool,
    immune_weaken: Bool,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
struct CatCSV2 {
    // index = 52
    has_zkill: OpBool,
    has_wkill: OpBool,
    _uk54: OpSmall,
    _uk55: Option<i8>,
    // "loop", appears to be something to do with multihit
    immune_boss_shockwave: OpBool,
    _uk57: Option<i8>,
    kamikaze: OpBool,
    mhit_atk2: OpBig,

    // 60
    mhit_atk3: OpBig,
    mhit_atk2_fswing: OpBig,
    mhit_atk3_fswing: OpBig,
    proc_on_hit1: OpBool,
    proc_on_hit2: OpBool,
    proc_on_hit3: OpBool,
    _uk66: Option<i8>,
    death: Option<i8>,
    _uk68: OpSmall,
    _uk69: OpSmall,

    // 70
    barrier_break: OpPercent,
    _uk71: OpSmall,
    _uk72: OpSmall,
    _uk73: OpSmall,
    _uk74: OpSmall,
    immune_warp: OpPercent,
    _uk76: OpSmall,
    witch_killer_2: OpBool,
    // ???
    targ_relic: OpBool,
    immune_curse: OpBool,

    // 80
    has_insane_resist: OpBool,
    has_insane_damage: OpBool,
    savage_blow_chance: OpPercent,
    savage_blow_percent: OpBig,
    dodge_chance: OpPercent,
    dodge_duration: OpBig,
    surge_chance: OpPercent,
    // like wave is dependent on `is_mini_surge`
    surge_param_0: OpBig,
    surge_param_1: OpBig,
    // TODO figure out what these are
    surge_level: OpSmall,

    // 90
    immune_toxic: OpBool,
    immune_surge: OpBool,
    curse_chance: OpPercent,
    curse_duration: OpBig,
    is_mini_wave: OpBool,
    shield_pierce_chance: OpPercent,
    targ_aku: OpBool,
    has_colossus_slayer: OpBool,
    soulstrike: OpBool,
    second_ld_is_different: OpBool,

    // 100
    second_ld_base: OpBig,
    second_ld_range: Option<i32>,
    third_ld_is_different: OpBool,
    third_ld_base: OpBig,
    third_ld_range: Option<i32>,
    has_behemoth_slayer: OpBool,
    bslayer_dodge_chance: OpPercent,
    bslayer_dodge_duration: OpBig,
    is_mini_surge: OpBool,
    counter_surge: OpBool,

    // 110
    conjure_unit: Option<i16>,
    // for some godforsaken reason this can be -1
    has_sage_slayer: OpBool,
    metal_killer_percent: OpPercent,
    explosion_chance: OpPercent,
    explosion_range: OpBig,
    _uk115: OpSmall,
    immune_explosion: OpBool,

    rest: Vec<i32>,
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
            .trim_matches(|c: char| c.is_whitespace() || c == ',');
        if line.is_empty() {
            continue;
        }

        let record = ByteRecord::from_iter(line.split(','));
        let cat: CatCSV = ByteRecord::from_iter(record.iter())
            .deserialize(None)
            .expect("Error when converting to fixed cat data");
        println!("{len} {cat:?}", len = record.len());

        // println!("{:?}", record.iter().skip(52).collect::<Vec<_>>());
        let a: CatCSV2 = ByteRecord::from_iter(record.iter().skip(52))
            .deserialize(None)
            .expect("Error when converting to extra cat data");
        println!("{a:?}");
    }
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
