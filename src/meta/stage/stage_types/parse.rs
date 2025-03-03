//! Parse ID from various formats.

#![allow(unused_variables)]

use crate::meta::stage::{map_id::MapID, stage_id::StageID};

// stages

fn parse_general_stage_id(selector: &str) -> StageID {
    todo!()
}

// -----------------------------------------------------------------------------

// maps

fn parse_general_map_id(selector: &str)->MapID{
    todo!()
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
