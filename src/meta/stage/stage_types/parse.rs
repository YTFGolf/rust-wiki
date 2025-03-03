//! Parse ID from various formats.

#![allow(unused_variables)]

use strum::IntoEnumIterator;

use crate::meta::stage::{
    map_id::MapID,
    stage_id::StageID,
    stage_types::data::{get_stage_type, SELECTOR_SEPARATOR},
    variant::StageVariantID,
};

#[derive(Debug, PartialEq)]
/// Error when parsing the stage type.
pub enum StageTypeParseError {
    /// The stage type selector given was invalid.
    Invalid,
}

// stages

fn parse_general_stage_id(selector: &str) -> StageID {
    todo!()
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
        None => return Err(StageTypeParseError::Invalid),
        Some(v) => v,
    };

    if is_single_stage(variant) || is_single_map(variant) {
        // if type only has 1 stage/map then map num will always be 0
        return Ok(MapID::from_components(variant, 0));
    };

    if variant == StageVariantID::MainChapters {
        // has to have separate logic depending on what you put as your selector

        // THIS IS HARDCODED, DO NOT UPDATE THIS WITHOUT UPDATING
        // `assert_main_selector`
        todo!()
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_main_selector() {
        // DO NOT CHANGE THIS TEST WITHOUT UPDATING `parse_map_selector`
        let desired: Vec<&str> = "main|EoC|ItF|W|CotC|Space|3".split('|').collect();
        let main = get_stage_type(StageVariantID::MainChapters);
        assert_eq!(desired, main.matcher.arr);
    }
}

/*
Test:
// check that all functions return a value (except custom on identifier)
// assert all non-custom work the conventional way
// Test (in another module) every available alias
// check case-insensitivity
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

// function to get the regex matching done properly.
// "z 1|z 2|z 3" e.g.
// All will get their map codes, stage codes and numbers added automatically
// if begins with z then there is a special case, maybe this could tell that

/*
    mod tests {
        use super::*;

        #[test]
        fn test_get_selector_type() {
            assert_eq!(get_selector_type("ITF").unwrap().type_enum, T::MainChapters);
            assert_eq!(get_selector_type("itf").unwrap().type_enum, T::MainChapters);
            assert_eq!(get_selector_type("itf2"), None);
        }

        #[test]
        fn test_get_stage_type_code() {
            assert_eq!(get_stage_type_code(T::MainChapters), LEGACY_STAGE_TYPES[3]);
        }
    }
*/
