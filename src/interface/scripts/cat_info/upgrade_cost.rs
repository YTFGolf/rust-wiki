//! Deals with upgrade costs.

use crate::{
    Config,
    game_data::cat::{
        parsed::{
            cat::Cat,
            unitbuy::{CatUnlock, UnlockCurrency},
        },
        raw::unitexp::XPCostScale,
    },
    interface::error_handler::InfallibleWrite,
    wikitext::section::{Section, SectionTitle},
};
use num_format::{Locale, ToFormattedString};
use std::fmt::Write;

const RR: [u32; 10] = [
    5_000, 8_000, 12_200, 17_800, 24_800, 33_200, 43_000, 54_200, 66_800, 80_800,
];
const SR: [u32; 10] = [
    6_250, 8_200, 12_400, 17_800, 24_800, 42_400, 64_500, 93_000, 148_000, 298_000,
];
const UR: [u32; 10] = [
    7_800, 9_800, 14_800, 21_800, 42_500, 64_300, 93_200, 118_000, 197_400, 513_500,
];
const LR: [u32; 10] = [
    7_800, 9_800, 14_800, 21_800, 42_500, 64_300, 93_200, 118_000, 197_400, 513_500,
];
const EXL: [u32; 10] = [
    7_800, 9_800, 14_800, 21_800, 42_500, 64_300, 93_200, 118_000, 197_400, 513_500,
];
const EX1: [u32; 10] = [
    1_500, 2_000, 3_200, 4_800, 6_800, 9_200, 12_000, 15_200, 18_800, 22_800,
];
const EX2: [u32; 10] = [
    2_000, 3_500, 6_200, 9_800, 14_300, 19_700, 26_000, 33_200, 41_300, 50_300,
];
const EX3: [u32; 10] = [
    1_300, 2_500, 4_000, 6_000, 8_500, 11_500, 15_000, 19_000, 23_500, 28_500,
];
const EX4: [u32; 10] = [
    3_500, 5_600, 8_540, 12_460, 17_360, 23_240, 30_100, 37_940, 46_760, 56_560,
];
const EX5: [u32; 10] = [
    4_375, 5_740, 8_680, 12_460, 17_360, 29_680, 45_150, 65_100, 103_600, 208_600,
];

const SECTION_TITLE: SectionTitle = SectionTitle::Blank;

fn upgrade_cost_mix_normal_unitexp(max: u8, initial: u16, costs: [u32; 10]) -> Section {
    let mut t = String::from("{{Upgrade Cost|MIX\n");
    writeln!(t, "|{}", initial.to_formatted_string(&Locale::en)).infallible_write();
    for level in costs.iter().skip(1) {
        writeln!(t, "|{}", level.to_formatted_string(&Locale::en)).infallible_write();
    }
    const NORMAL_UNITEXP_MULTIPLIER_10: u32 = 2;
    writeln!(
        t,
        "|{}",
        (costs[0] * NORMAL_UNITEXP_MULTIPLIER_10).to_formatted_string(&Locale::en)
    )
    .infallible_write();

    if max != 50 {
        writeln!(t, "|max = {}", max.to_formatted_string(&Locale::en)).infallible_write();
    }

    Section::new(SECTION_TITLE, (t + "}}").into())
}

/// Get upgrade costs section.
pub fn upgrade_cost(cat: &Cat, _config: &Config) -> Section {
    if cat.unitexp != XPCostScale::Normal {
        assert_eq!(cat.id, 643);
        todo!("superfeline");
    }

    let max = cat.unitbuy.max_levels.max_nat;
    let initial = match cat.unitbuy.unlock {
        CatUnlock {
            unlock_cost: i,
            unlock_currency: UnlockCurrency::XP,
            ..
        } => i,
        _ => 0,
    };
    let costs = cat.unitbuy.upgrade_costs.costs;
    println!("{costs:?}");

    let code = match (max, initial, costs) {
        (50, 0, c) if c == RR => "RR",
        (50, 0, c) if c == SR => "SR",
        (60, 0, c) if c == UR => "UR",
        // note the level 60
        (50, 0, c) if c == LR => "LR",
        (50, 0, c) if c == EXL => "EXL",
        (50, 0, c) if c == EX1 => "EX1",
        (50, 0, c) if c == EX2 => "EX2",
        (50, 0, c) if c == EX3 => "EX3",
        (50, 0, c) if c == EX4 => "EX4",
        (50, 0, c) if c == EX5 => "EX5",
        (a, b, c) => return upgrade_cost_mix_normal_unitexp(a, b, c),
    };

    Section::new(SECTION_TITLE, format!("{{{{Upgrade Cost|{code}}}}}").into())
}

/*

{{Upgrade Cost|MIX
|999999
|999999
|999999
|999999
|999999
|999999
|999999
|999999
|999999
|999999
|mult1 = 1
|max = 20
}}

*/

/*
{{Upgrade Cost|MIX
|0
|400
|700
|1,100
|1,600
|2,200
|2,900
|3,700
|4,600
|5,600
|400
|max = 20
}}
*/
