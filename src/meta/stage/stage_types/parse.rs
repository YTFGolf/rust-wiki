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
}

// stages

fn parse_general_stage_id(selector: &str) -> StageID {
    todo!()
    // from_file;
    // from_selector;
    // from_ref;
}

fn parse_stage_file(file_name:&str)->StageID{
    todo!()
}

// fn parse_stage_ref(ref:&str)->StageID

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
    fn parse_selector_sol() {
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
    fn parse_selector_ex() {
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
    fn parse_selector_main() {
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
    fn parse_single_stage() {
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
    fn parse_single_map() {
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
    fn parse_selector_fail() {
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
