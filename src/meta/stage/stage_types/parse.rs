//! Parse ID from various formats.

#![allow(unused_variables, missing_docs)]

/*
Important note: selectors in this file are custom behaviour, so any time that
selectors are updated then the docs also need to be updated.
*/

// -----------------------------------------------------------------------------

// Temporary implementation for refactoring.

use crate::data::stage::raw::stage_metadata::LegacyStageMeta;

impl From<StageID> for LegacyStageMeta {
    fn from(value: StageID) -> Self {
        todo!()
    }
}

// -----------------------------------------------------------------------------

use crate::meta::stage::{
    map_id::{MapID, MapSize},
    stage_id::{StageID, StageSize},
    stage_types::data::{get_stage_type, SELECTOR_SEPARATOR},
    variant::StageVariantID,
};
use regex::Regex;
use std::sync::LazyLock;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
/// Error when parsing the stage type.
pub enum StageTypeParseError {
    /// Invalid stage type variant "matcher".
    UnknownMatcher,
    /// No map number provided when necessary.
    NoMapNumber,
    /// No stage number provided when necessary.
    NoStageNumber,
    /// Map or stage number is invalid.
    InvalidNumber,
    /// Selector is not in a valid format for the given function.
    InvalidFormat,
}

// stages

pub fn parse_general_stage_id(selector: &str) -> Option<StageID> {
    // Could check selectors before functions but this only really gets done on
    // a mass scale from files.
    if let Ok(st) = parse_stage_file(selector) {
        return Some(st);
    };
    if let Ok(st) = parse_stage_selector(selector) {
        return Some(st);
    };
    if let Ok(st) = parse_stage_ref(selector) {
        return Some(st);
    };

    None
}

type T = StageVariantID;

static GENERAL_STAGE_PAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^stage([\D]*)([\d]*)_([\d]*)\.csv$").unwrap());
fn parse_stage_file(file_name: &str) -> Result<StageID, StageTypeParseError> {
    const FILE_BEGIN: &str = "stage";
    if !(file_name.starts_with(FILE_BEGIN) || file_name.ends_with(".csv")) {
        return Err(StageTypeParseError::InvalidFormat);
    } else if file_name == "stageSpace09_Invasion_00.csv" {
        return Ok(StageID::from_components(T::Filibuster, 0, 0));
    }

    // Remaining formats:
    // - eoc: "stagexx.csv" -> (Main, 0, xx)
    // - other: "stage{code}{map}_{stage}.csv"

    let remaining_chars = &file_name[FILE_BEGIN.len()..];
    let bytes = remaining_chars.as_bytes();

    if bytes[0].is_ascii_digit() {
        // must be eoc if next is digit
        let num = remaining_chars[0..=1]
            .parse::<StageSize>()
            .map_err(|_| StageTypeParseError::InvalidNumber)?;
        return Ok(StageID::from_components(T::MainChapters, 0, num));
    }

    // Now all things must follow the format of `GENERAL_STAGE_PAT`.
    let (_, caps): (&str, [&str; 3]) = GENERAL_STAGE_PAT.captures(file_name).unwrap().extract();
    // I don't know how to do this idiomatically or efficiently so it's Regex
    // time.
    // let [var, map, stage] = caps;
    match caps[0] {
        "W" => {
            let [_, map, stage] = caps;
            let map = map.parse::<MapSize>().unwrap() - 1;
            // W04 = ItF 1 = main 3
            let stage = stage.parse().unwrap();
            Ok(StageID::from_components(T::MainChapters, map, stage))
        }
        "Space" => {
            let [_, map, stage] = caps;
            let map = map.parse::<MapSize>().unwrap() - 1;
            // Space07 = CotC 1 = main 6
            let stage = stage.parse().unwrap();
            Ok(StageID::from_components(T::MainChapters, map, stage))
        }
        "Z" => {
            let [_, map, stage] = caps;
            let map = map.parse().unwrap();
            let stage = stage.parse().unwrap();

            match map {
                (0..=2) => Ok(StageID::from_components(T::EocOutbreak, map, stage)),
                (4..=6) => Ok(StageID::from_components(T::ItfOutbreak, map - 4, stage)),
                (7..=8) => Ok(StageID::from_components(T::CotcOutbreak, map - 7, stage)),
                // 8 in current version
                x => panic!("Map number {map:?} found in file name parser."),
            }
        }
        _ => parse_stage_selector(&caps.join(&SELECTOR_SEPARATOR.to_string())),
    }
}

static DB_REFERENCE_FULL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*?https://battlecats-db.com/stage/(s[\d\-]+).html").unwrap());
/// Captures `["01", "001", "999"]` in `"s01001-999"`.
static DB_REFERENCE_STAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^s(\d{2})(\d{3})\-(\d{2,})$").unwrap());
// could possibly factor out the \d{2}\d{3} to be mapid
fn parse_stage_ref(reference: &str) -> Result<StageID, StageTypeParseError> {
    let reference = DB_REFERENCE_FULL.replace(reference, "$1");

    match DB_REFERENCE_STAGE.captures(&reference) {
        Some(caps) => {
            let chapter: u32 = caps[1].parse().unwrap();
            let submap: u32 = caps[2].parse().unwrap();
            let stage: u32 = caps[3].parse::<u32>().unwrap() - 1;
            Ok(StageID::from_numbers(chapter, submap, stage))
        }
        None => Err(StageTypeParseError::InvalidFormat),
    }
}

pub fn parse_stage_selector(selector: &str) -> Result<StageID, StageTypeParseError> {
    let map = parse_map_selector(selector)?;

    if is_single_stage(map.variant()) {
        return Ok(StageID::from_map(map, 0));
    }

    if is_single_map(map.variant())
        || map == MapID::from_components(StageVariantID::MainChapters, 0)
    {
        // if single map (inc. EoC) then just need last number
        if !selector.contains(SELECTOR_SEPARATOR) {
            return Err(StageTypeParseError::NoStageNumber);
        }

        let last_num = selector.rsplit(SELECTOR_SEPARATOR).next().unwrap();
        let last_num = last_num
            .parse::<StageSize>()
            .map_err(|_| StageTypeParseError::InvalidNumber)?;

        return Ok(StageID::from_map(map, last_num));
    }

    let mut iter = selector.split(SELECTOR_SEPARATOR);
    let stage_num = iter.nth(2).ok_or(StageTypeParseError::NoStageNumber)?;
    let stage_num = stage_num
        .parse::<StageSize>()
        .map_err(|_| StageTypeParseError::InvalidNumber)?;

    return Ok(StageID::from_map(map, stage_num));
}

// -----------------------------------------------------------------------------

// maps

pub fn parse_general_map_id(selector: &str) -> MapID {
    todo!()
}

fn get_variant_from_code(compare: &str) -> Option<StageVariantID> {
    for variant in StageVariantID::iter() {
        let stype = get_stage_type(variant);
        if stype.matcher.re.is_match(compare) {
            return Some(stype.data.variant_id);
        }
        // I think regex is probably faster than arr.contains
    }

    None
}

/// Variant only has a single stage.
fn is_single_stage(v: StageVariantID) -> bool {
    type T = StageVariantID;
    matches!(v, T::Challenge | T::Filibuster)
}

/// Variant only has a single map but multiple stages.
fn is_single_map(v: StageVariantID) -> bool {
    type T = StageVariantID;
    matches!(v, T::AkuRealms | T::Labyrinth)
}

pub fn parse_map_selector(selector: &str) -> Result<MapID, StageTypeParseError> {
    let mut iter = selector.split(SELECTOR_SEPARATOR);
    let compare = iter
        .next()
        .expect("I literally have no clue how this would fail.");

    let variant = match get_variant_from_code(compare) {
        None => return Err(StageTypeParseError::UnknownMatcher),
        Some(v) => v,
    };

    if is_single_stage(variant) || is_single_map(variant) {
        // if type only has 1 stage/map then map num will always be 0
        return Ok(MapID::from_components(variant, 0));
    };

    let Some(map_num) = iter.next() else {
        return Err(StageTypeParseError::NoMapNumber);
    };
    let Ok(map_num) = map_num.parse::<MapSize>() else {
        return Err(StageTypeParseError::InvalidNumber);
    };

    if variant == StageVariantID::MainChapters {
        // has to have separate logic depending on what you put as your selector

        // THIS IS HARDCODED, DO NOT UPDATE THIS WITHOUT UPDATING
        // `assert_main_selector`
        match compare.to_lowercase().as_str() {
            "eoc" => return Ok(MapID::from_components(variant, 0)),
            // eoc has 1 chapter that is number 0
            "itf" | "w" => return Ok(MapID::from_components(variant, map_num + 2)),
            // itf 1 = "itf 1" = "main 3"
            "cotc" | "space" => return Ok(MapID::from_components(variant, map_num + 5)),
            // cotc 1 = "cotc 1" = "main 6"
            _ => (),
            // if you put main or 3 then I assume you know what you're doing
        }
    }

    Ok(MapID::from_components(variant, map_num))
}

// fn parse_map_from_iterator<'a, T>()
// where
//     T: Iterator<Item = &'a str>,
// {
//     todo!()
// }
// I'm okay with this being a monolith for now.

#[cfg(test)]
mod tests_general {
    use super::*;

    #[test]
    fn assert_main_selector() {
        // DO NOT CHANGE THIS TEST WITHOUT UPDATING `parse_map_selector`
        let desired: Vec<&str> = "main|EoC|ItF|W|CotC|Space|3".split('|').collect();
        let main = get_stage_type(StageVariantID::MainChapters);
        assert_eq!(desired, main.matcher.arr);
    }
}

#[cfg(test)]
mod tests_stage {
    use super::*;
    use StageVariantID as T;

    #[test]
    fn test_parse_selector_sol() {
        let answer = StageID::from_components(T::SoL, 0, 0);

        let st = parse_stage_selector("SoL 0 0").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_selector("sol 0 0").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_selector("n 0 0").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_selector("rn 0 0").unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_parse_selector_ex() {
        let answer = StageID::from_components(T::Extra, 0, 0);

        let st = parse_stage_selector("eXTRA 0 0").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_selector("extra 0 0").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_selector("4 0 0").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_selector("RE 0 0").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_selector("EX 0 0").unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_parse_selector_main() {
        let eoc1 = StageID::from_components(T::MainChapters, 0, 0);
        let st = parse_stage_selector("eoc 0").unwrap();
        assert_eq!(st, eoc1);
        let st = parse_stage_selector("main 0 0").unwrap();
        assert_eq!(st, eoc1);
        let st = parse_stage_selector("3 0 0").unwrap();
        assert_eq!(st, eoc1);

        let itf1 = StageID::from_components(T::MainChapters, 3, 0);
        let st = parse_stage_selector("itf 1 0").unwrap();
        assert_eq!(st, itf1);
        let st = parse_stage_selector("main 3 0").unwrap();
        assert_eq!(st, itf1);
        let st = parse_stage_selector("3 3 0").unwrap();
        assert_eq!(st, itf1);

        let cotc1 = StageID::from_components(T::MainChapters, 6, 0);
        let st = parse_stage_selector("cotc 1 0").unwrap();
        assert_eq!(st, cotc1);
        let st = parse_stage_selector("main 6 0").unwrap();
        assert_eq!(st, cotc1);
        let st = parse_stage_selector("3 6 0").unwrap();
        assert_eq!(st, cotc1);
    }

    #[test]
    fn test_parse_single_stage() {
        let filibuster = StageID::from_components(T::Filibuster, 0, 0);
        let st = parse_stage_selector("filibuster").unwrap();
        assert_eq!(st, filibuster);
        let st = parse_stage_selector("filibuster 30 12").unwrap();
        assert_eq!(st, filibuster);

        let challenge = StageID::from_components(T::Challenge, 0, 0);
        let st = parse_stage_selector("challenge").unwrap();
        assert_eq!(st, challenge);
        let st = parse_stage_selector("challenge 30 12").unwrap();
        assert_eq!(st, challenge);
    }

    #[test]
    fn test_parse_single_map() {
        let st = parse_stage_selector("aku 0").unwrap();
        assert_eq!(st, StageID::from_components(T::AkuRealms, 0, 0));
        let st = parse_stage_selector("aku 1").unwrap();
        assert_eq!(st, StageID::from_components(T::AkuRealms, 0, 1));
        let st = parse_stage_selector("aku 0 1").unwrap();
        assert_eq!(st, StageID::from_components(T::AkuRealms, 0, 1));
        let st = parse_stage_selector("aku 0 0 0 0 0 1").unwrap();
        assert_eq!(st, StageID::from_components(T::AkuRealms, 0, 1));

        let st = parse_stage_selector("labyrinth 0").unwrap();
        assert_eq!(st, StageID::from_components(T::Labyrinth, 0, 0));
        let st = parse_stage_selector("labyrinth 1").unwrap();
        assert_eq!(st, StageID::from_components(T::Labyrinth, 0, 1));
        let st = parse_stage_selector("labyrinth 0 1").unwrap();
        assert_eq!(st, StageID::from_components(T::Labyrinth, 0, 1));
        let st = parse_stage_selector("labyrinth 0 0 0 0 0 1").unwrap();
        assert_eq!(st, StageID::from_components(T::Labyrinth, 0, 1));

        let st = parse_stage_selector("eoc 0").unwrap();
        assert_eq!(st, StageID::from_components(T::MainChapters, 0, 0));
        let st = parse_stage_selector("eoc 1").unwrap();
        assert_eq!(st, StageID::from_components(T::MainChapters, 0, 1));
        let st = parse_stage_selector("eoc 0 1").unwrap();
        assert_eq!(st, StageID::from_components(T::MainChapters, 0, 1));
        let st = parse_stage_selector("eoc 0 0 0 0 0 1").unwrap();
        assert_eq!(st, StageID::from_components(T::MainChapters, 0, 1));
    }

    #[test]
    fn test_parse_selector_fail() {
        let st = parse_stage_selector("invalid_selector 0 0");
        assert_eq!(st, Err(StageTypeParseError::UnknownMatcher));
    }

    #[test]
    fn test_from_selector() {
        let st = parse_stage_selector("N 0 0").unwrap();
        assert_eq!(st, StageID::from_components(T::SoL, 0, 0));

        let st = parse_stage_selector("sol 0 0").unwrap();
        assert_eq!(st, StageID::from_components(T::SoL, 0, 0));

        let st = parse_stage_selector("T 0 0").unwrap();
        assert_eq!(st, StageID::from_components(T::Dojo, 0, 0));

        let st = parse_stage_selector("EX 0 0").unwrap();
        assert_eq!(st, StageID::from_components(T::Extra, 0, 0));

        let st = parse_stage_selector("COTC 1 0").unwrap();
        assert_eq!(st, StageID::from_components(T::MainChapters, 6, 0));
    }

    #[test]
    fn test_from_file() {
        let st = parse_stage_file("stageRN000_00.csv").unwrap();
        assert_eq!(st, StageID::from_components(T::SoL, 0, 0));

        let st = parse_stage_file("stageRT000_00.csv").unwrap();
        assert_eq!(st, StageID::from_components(T::Dojo, 0, 0));

        let st = parse_stage_file("stageL000_00.csv").unwrap();
        assert_eq!(st, StageID::from_components(T::Labyrinth, 0, 0));

        let st = parse_stage_file("stageEX000_00.csv").unwrap();
        assert_eq!(st, StageID::from_components(T::Extra, 0, 0));
    }

    #[test]
    fn test_from_file_main() {
        let st = parse_stage_file("stageSpace07_00.csv").unwrap();
        assert_eq!(st, StageID::from_components(T::MainChapters, 6, 0));

        let st = parse_stage_file("stageZ00_00.csv").unwrap();
        assert_eq!(st, StageID::from_components(T::EocOutbreak, 0, 0));
    }

    #[test]
    fn test_from_ref() {
        let answer = StageID::from_components(T::SoL, 0, 0);

        let st = parse_stage_ref("*https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_ref("https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = parse_stage_ref("s00000-01").unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_new() {
        let selector = "*https://battlecats-db.com/stage/s01382-03.html";
        assert_eq!(
            parse_stage_ref(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_general_stage_id(selector).unwrap(),
            StageID::from_components(T::Event, 382, 2)
        );

        let selector = "ItF 1 48";
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            StageID::from_components(T::MainChapters, 3, 48)
        );

        let selector = "DM 0";
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            StageID::from_components(T::AkuRealms, 0, 0)
        );

        let selector = "Filibuster";
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            StageID::from_components(T::Filibuster, 0, 0)
        );

        let selector = "itfz 1 0";
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_stage_selector(selector).unwrap(),
            StageID::from_components(T::ItfOutbreak, 1, 0)
        );

        let selector = "stageRN013_05.csv";
        assert_eq!(
            parse_stage_file(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_stage_file(selector).unwrap(),
            StageID::from_components(T::SoL, 13, 5)
        );

        let selector = "stageRN000_00.csv";
        assert_eq!(
            parse_stage_file(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_stage_file(selector).unwrap(),
            StageID::from_components(T::SoL, 0, 0)
        );

        let selector = "stageW04_05.csv";
        assert_eq!(
            parse_stage_file(selector).unwrap(),
            parse_general_stage_id(selector).unwrap()
        );
        assert_eq!(
            parse_stage_file(selector).unwrap(),
            StageID::from_components(T::MainChapters, 3, 5)
        );

        let selector = "stageW04_05.csv";
        assert_eq!(
            parse_general_stage_id(&String::from(selector)),
            parse_general_stage_id(selector)
        );
        assert_eq!(
            parse_general_stage_id(&String::from(selector)).unwrap(),
            StageID::from_components(T::MainChapters, 3, 5)
        );
    }
}

/*
Test:
// check that all functions return a value (except custom on identifier)
// assert all non-custom work the conventional way
// Test (in another module) every available alias
// check case-insensitivity
// check failed cases to ensure failure is graceful
 */

/*
Plan:
- Does the equivalent of STAGE_TYPES: contains data about stages.
- Below applies to two separate sibling modules. All tests from LegacyStageMeta
  also get moved over to here. Siblings can use file names to implement a
  temporary from parser for LegacyStageMeta.
- This or a sibling module deals with parsing selectors. This or a sibling
  module deals with turning this information into real-world data (e.g. file
  names like `MapStageDataA_000.csv`).
*/

// impl StageType {
//     /// Code used in map files.
//     fn map_code(&self) -> Option<&'static str> {
//         self.map_code
//     }

//     /// Code used in stage files.
//     fn stage_code(&self) -> &StageCodeType {
//         &self.stage_code
//     }

//     // Get identifier (map,rprefix=map,other=other,custom=unimplemented)
//     // fn stage_ident(&self) -> &'static str {
//     //     todo!()
//     // }
// }
