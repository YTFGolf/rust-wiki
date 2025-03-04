//! Parse ID from various formats.

#![allow(unused_variables, missing_docs)]

use crate::meta::stage::{stage_types::data::get_stage_type, variant::StageVariantID};
use strum::IntoEnumIterator;

pub mod parse_map;
pub mod parse_stage;

/*
Important note: selectors in this module are custom behaviour, so any time that
selectors are updated then the docs also need to be updated.
*/

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
