//! Defines information that can be used to obtain or transform pure data from
//! or into usable formats.

#![allow(dead_code)]
// I don't want this to be too spaghetti-y so might become its own entire
// module.

use super::variant::StageVariantID;

/// Type of stage code used.
pub enum StageCodeType {
    /// Code is the same as map (Aku Realms, Labyrinth, Championships e.g.).
    Map,
    /// Code is map with an R at the start (most stages).
    RPrefix,
    /// Code is completely different (EX), map name images use this different
    /// code rather than map code.
    Other(&'static str),
    /// Requires custom logic to deal with the stage code.
    Custom,
}

/// Constant reference to a stage type.
pub struct StageType {
    /// Variant ID of the stage type.
    variant_id: StageVariantID,
    /// Full readable name of the stage type.
    name: &'static str,
    /// Code used in map data files. None means that it will need to be figured
    /// out manually.
    map_code: Option<&'static str>,
    /// Code used in stage data files.
    stage_code: StageCodeType,
    /// Regex matcher for the stage type.
    matcher_str: &'static str,
}
/*
Functions to:
- Get map code (str)
- Get stage code (str)
- Get identifier (map,rprefix=map,other=other,custom=unimplemented)
// note that names may be changed and need to update whole file first

also fun fact legend quest has prefix D
*/

const MAX_VARIANT_NUMBER: usize = 37;
// store the data, store the map
const STAGE_TYPES: [Option<StageType>; MAX_VARIANT_NUMBER] = [const { None }; MAX_VARIANT_NUMBER];

const fn variant_to_index(variant: StageVariantID) -> usize {
    variant.num() as usize
}

/// Get variant's stage type.
pub fn get_stage_type(variant: StageVariantID) -> &'static StageType {
    let i = variant_to_index(variant);
    match &STAGE_TYPES[i] {
        Some(v) => v,
        None => panic!("Variant is not initialised properly!"),
    }
}

// function to get the regex matching done properly.
// "z 1|z 2|z 3" e.g.
// All will get their map codes, stage codes and numbers added automatically
// if begins with z then there is a special case, maybe this could tell that

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_variants() {
        // assert that array has all variants and numbers don't exceed
        // [`MAX_VARIANT_NUMBER`].
        for variant in StageVariantID::iter() {
            let is_in_map = STAGE_TYPES
                .iter()
                .flatten()
                .filter(|st| st.variant_id == variant)
                .collect::<Vec<_>>();
            let len = is_in_map.len();
            assert_eq!(len, 1, "{variant:?} should appear exactly once.");

            let variant_num = usize::try_from(variant.num())
                .expect("Error when converting from stage variant number to usize.");
            assert!(
                variant_num <= MAX_VARIANT_NUMBER,
                "Variant {variant:?} has a value higher than {MAX_VARIANT_NUMBER}."
            );
        }
    }

    #[test]
    fn test_has_valid_map_and_stage_codes() {
        todo!()
        // check that all functions return a value (except custom on identifier)
    }

    #[test]
    fn assert_matchers_are_unique() {
        todo!()
        // check that no duplicate match possibilities exist
    }
}

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
