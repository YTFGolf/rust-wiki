//! Deals with raw CSV cat data.

use crate::{game_data::version::Version, regex_handler::static_regex};
use csv::ByteRecord;
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

/// CSV data about a cat.
pub type CombinedCatData = (CatCSV, CatCSV2);

#[derive(Debug, serde::Deserialize, Default)]
#[allow(missing_docs)]
/// Fixed CSV data.
pub struct CatCSV {
    pub hp: Massive,
    pub kb: Big,
    pub speed: Small,
    pub atk: Massive,
    pub tba: Big,
    pub range: Big,
    pub price: Big,
    pub respawn: Big,
    _uk8: Small,
    _width: Big,
    // should always be 320

    // 10
    pub targ_red: Bool,
    _uk2: Small,
    pub is_area: Bool,
    pub foreswing: Big,
    _uk14: Small,
    // "front"
    _uk15: Small,
    // "back"
    pub targ_float: Bool,
    pub targ_black: Bool,
    pub targ_metal: Bool,
    pub targ_traitless: Bool,

    // 20
    pub targ_angel: Bool,
    pub targ_alien: Bool,
    pub targ_zombie: Bool,
    pub has_strong: Bool,
    pub kb_chance: Percent,
    pub freeze_chance: Percent,
    pub freeze_duration: Big,
    pub slow_chance: Percent,
    pub slow_duration: Big,
    pub has_resist: Bool,

    // 30
    pub has_massive_damage: Bool,
    pub crit_chance: Percent,
    pub has_targets_only: Bool,
    pub has_double_bounty: Bool,
    pub has_base_destroyer: Bool,
    pub wave_chance: Percent,
    pub wave_level: Small,
    // used for both wave and mini-wave, see `is_mini_wave`
    pub weaken_chance: Percent,
    pub weaken_duration: Big,
    pub weaken_multiplier: Percent,

    // 40
    pub strengthen_hp: Percent,
    pub strengthen_multiplier: Big,
    pub survives_chance: Percent,
    pub has_metal: Bool,
    pub ld_base: i16,
    pub ld_range: i16,
    // if ld_range is neg then is omnistrike, need to consult the hitbox page
    pub immune_wave: Bool,
    pub has_wave_blocker: Bool,
    pub immune_kb: Bool,
    pub immune_freeze: Bool,

    // 50
    pub immune_slow: Bool,
    pub immune_weaken: Bool,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
#[allow(missing_docs)]
/// Data that may not exist. All fields default to `0` if not explicitly given.
pub struct CatCSV2 {
    // index = 52
    pub has_zombie_killer: Option<Bool>,
    // can be blank for whatever reason so needs `Option`
    pub has_witch_killer: Bool,
    _uk54: Small,
    _uk55: i8,
    // "loop" according to BCU, what that means is unclear
    pub immune_boss_shockwave: Bool,
    _uk57: i8,
    pub kamikaze: Small,
    // for some reason is like a bool but 2 is the true value
    pub mhit_atk2: Massive,

    // 60
    pub mhit_atk3: Massive,
    pub mhit_atk2_fswing: Big,
    pub mhit_atk3_fswing: Big,
    pub proc_on_hit1: Bool,
    pub proc_on_hit2: Bool,
    pub proc_on_hit3: Bool,
    _uk66: i8,
    pub death: i8,
    _uk68: Small,
    _uk69: Small,

    // 70
    pub barrier_break_chance: Percent,
    _uk71: Small,
    _uk72: Small,
    _uk73: Small,
    _uk74: Small,
    pub immune_warp: Bool,
    _uk76: Small,
    pub has_eva_angel_killer: Bool,
    pub targ_relic: Bool,
    pub immune_curse: Bool,

    // 80
    pub has_insane_resist: Bool,
    pub has_insane_damage: Bool,
    pub savage_blow_chance: Percent,
    pub savage_blow_percent: Big,
    pub dodge_chance: Percent,
    pub dodge_duration: Big,
    pub surge_chance: Percent,
    // like wave, this is dependent on `is_mini_surge`
    pub surge_spawn_quad: Big,
    pub surge_range_quad: Big,
    // for some reason both are 4 * actual range
    pub surge_level: Small,

    // 90
    pub immune_toxic: Bool,
    pub immune_surge: Bool,
    pub curse_chance: Percent,
    pub curse_duration: Big,
    pub is_mini_wave: Bool,
    pub shield_pierce_chance: Percent,
    pub targ_aku: Bool,
    pub has_colossus_slayer: Bool,
    pub has_soulstrike: Bool,
    pub second_ld_is_different: Bool,

    // 100
    pub second_ld_base: i16,
    pub second_ld_range: i16,
    pub third_ld_is_different: Bool,
    pub third_ld_base: i16,
    pub third_ld_range: i16,
    pub has_behemoth_slayer: Bool,
    pub bslayer_dodge_chance: Percent,
    pub bslayer_dodge_duration: Big,
    pub is_mini_surge: Bool,
    pub has_counter_surge: Bool,

    // 110
    pub conjure_unit_id: i16,
    // for some godforsaken reason this can be -1 or 0 to represent doesn't summon
    pub has_sage_slayer: Bool,
    pub metal_killer_percent: Percent,
    pub explosion_chance: Percent,
    pub explosion_spawn_quad: Big,
    // for some reason is 4 * actual range
    _uk115: Small,
    pub immune_explosion: Bool,

    rest: Vec<i32>,
}

fn read_form_line(line: &str) -> CombinedCatData {
    let record = line.split(',').collect::<ByteRecord>();
    let fixed: CatCSV = record
        .iter()
        .collect::<ByteRecord>()
        .deserialize(None)
        .expect("Error when converting to fixed cat data");
    // TODO evaluate if this needs to have a trace
    // println!("{len} {cat:?}", len = record.len());

    // println!("{:?}", record.iter().skip(52).collect::<Vec<_>>());
    let var: CatCSV2 = record
        .iter()
        .skip(52)
        .collect::<ByteRecord>()
        .deserialize(None)
        .expect("Error when converting to extra cat data");

    (fixed, var)
}

/// Read a cat data file and return all of the unit's forms.
pub fn read_data_file(
    file_name: &str,
    version: &Version,
) -> impl Iterator<Item = CombinedCatData> + use<> {
    let stage_file = PathBuf::from("DataLocal").join(file_name);
    let reader = BufReader::new(File::open(version.get_file_path(&stage_file)).unwrap());

    reader.lines().filter_map(|line| {
        let line = line.unwrap();
        let line = line
            .split("//")
            .next()
            .expect("Shouldn't panic on first next.")
            .trim_matches(|c: char| c.is_whitespace() || c == ',');

        if line.is_empty() {
            None
        } else {
            Some(read_form_line(line))
        }
    })
}

/// Get a list of all cat data files in the game.
pub fn get_cat_files(version: &Version) -> impl Iterator<Item = String> {
    let re = static_regex(r"^unit\d");
    let dir = &version.get_file_path("DataLocal");

    let files = std::fs::read_dir(dir).unwrap();

    files.filter_map(move |f| {
        let file_name = f.unwrap().file_name().into_string().unwrap();
        // needs to be converted to string so regex works

        if re.is_match(&file_name) {
            Some(file_name)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();

        for file in get_cat_files(version) {
            println!("{file}");
            let forms = read_data_file(&file, version);

            #[allow(clippy::used_underscore_binding)]
            for (fixed, var) in forms {
                // if var.proc_on_hit1 == 0 && (var.proc_on_hit2 + var.proc_on_hit3 > 0) {
                //     println!("{fixed:?}, {var:?}");
                // }
                // if var.mhit_atk2 > 0 || var.mhit_atk3_fswing > 0 {
                //     println!("{}, {var:?}", var.proc_on_hit1 + var.proc_on_hit2 + var.proc_on_hit3);
                // }
                assert_eq!(fixed._width, 320);
                if var.kamikaze != 0 {
                    assert_eq!(var.kamikaze, 2);
                }
                assert!(
                    var.rest.is_empty(),
                    "Remaining fields not empty, found {:?}",
                    var.rest
                );
            }
        }
    }
}
