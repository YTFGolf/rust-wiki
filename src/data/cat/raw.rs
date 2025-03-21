#![allow(missing_docs, unused_imports, dead_code)]

use crate::data::version::Version;
use csv::{ByteRecord, StringRecord};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Could reasonably either go above 65,535 or be multiplied to go above (e.g.
/// stats).
type Massive = u32;
/// Big number from 0 to 65,535.
type Big = u16;
/// 0-100.
type Percent = u8;
/// 0-256.
type Small = u8;
/// 0 or 1.
type Bool = u8;

type OpBig = Option<u16>;
// TODO if 0 and None are basically synonymous then this can be removed

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
    has_zkill: Option<Bool>,
    // can be blank for whatever reason so needs `Option`
    has_wkill: Bool,
    _uk54: Small,
    _uk55: i8,
    // "loop", appears to be something to do with multihit
    immune_boss_shockwave: Bool,
    _uk57: i8,
    kamikaze: Bool,
    mhit_atk2: OpBig,

    // 60
    mhit_atk3: OpBig,
    mhit_atk2_fswing: OpBig,
    mhit_atk3_fswing: OpBig,
    proc_on_hit1: Bool,
    proc_on_hit2: Bool,
    proc_on_hit3: Bool,
    _uk66: i8,
    death: Option<i8>,
    _uk68: Small,
    _uk69: Small,

    // 70
    barrier_break_chance: Percent,
    _uk71: Small,
    _uk72: Small,
    _uk73: Small,
    _uk74: Small,
    immune_warp: Percent,
    _uk76: Small,
    witch_killer_2: Bool,
    // ???
    targ_relic: Bool,
    immune_curse: Bool,

    // 80
    has_insane_resist: Bool,
    has_insane_damage: Bool,
    savage_blow_chance: Percent,
    savage_blow_percent: OpBig,
    dodge_chance: Percent,
    dodge_duration: OpBig,
    surge_chance: Percent,
    // like wave is dependent on `is_mini_surge`
    surge_param_0: OpBig,
    surge_param_1: OpBig,
    // TODO figure out what these are
    surge_level: Small,

    // 90
    immune_toxic: Bool,
    immune_surge: Bool,
    curse_chance: Percent,
    curse_duration: OpBig,
    is_mini_wave: Bool,
    shield_pierce_chance: Percent,
    targ_aku: Bool,
    has_colossus_slayer: Bool,
    soulstrike: Bool,
    second_ld_is_different: Bool,

    // 100
    second_ld_base: Option<i32>,
    second_ld_range: Option<i32>,
    third_ld_is_different: Bool,
    third_ld_base: OpBig,
    third_ld_range: Option<i32>,
    has_behemoth_slayer: Bool,
    bslayer_dodge_chance: Percent,
    bslayer_dodge_duration: OpBig,
    is_mini_surge: Bool,
    counter_surge: Bool,

    // 110
    conjure_unit: Option<i16>,
    // for some godforsaken reason this can be -1
    has_sage_slayer: Bool,
    metal_killer_percent: Percent,
    explosion_chance: Percent,
    explosion_range: OpBig,
    _uk115: Small,
    immune_explosion: Bool,

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
        let _cat: CatCSV = ByteRecord::from_iter(record.iter())
            .deserialize(None)
            .expect("Error when converting to fixed cat data");
        // println!("{len} {cat:?}", len = record.len());

        // println!("{:?}", record.iter().skip(52).collect::<Vec<_>>());
        let a: CatCSV2 = ByteRecord::from_iter(record.iter().skip(52))
            .deserialize(None)
            .expect("Error when converting to extra cat data");

        assert!(
            a.rest.is_empty(),
            "Remaining fields not empty, found {:?}",
            a.rest
        );

        // if a.rest.len() > 0 || !matches!(a.conjure_unit, None | Some(0) | Some(-1)) {
        // if a.rest.len() > 0 && a.rest != [0, 0] {
        //     println!("{len} {cat:?}", len = record.len());
        //     println!("{a:?}");
        // }
    }
}

/// Get a list of all cat data files in the game.
pub fn get_cat_files(version: &Version) -> impl Iterator<Item = String> {
    let re = Regex::new(r"^unit\d").unwrap();
    let dir = &version.get_file_path("DataLocal");

    let files = std::fs::read_dir(dir).unwrap();

    files.filter_map(move |f| {
        let file_name = f.unwrap().file_name().into_string().unwrap();

        if re.is_match(&file_name) {
            Some(file_name)
        } else {
            None
        }
    })
}

#[test]
fn do_thing() {
    use crate::config::TEST_CONFIG;
    let version = TEST_CONFIG.version.current_version();

    for file in get_cat_files(version) {
        println!("{file}");
        read_data_file(&file, version);
        // assert that `rest` is empty
    }
}
