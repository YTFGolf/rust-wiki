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
/*
// note that names may be changed and need to update whole file first

also fun fact legend quest has prefix D
*/

use super::types::StageType;
use crate::meta::stage::variant::StageVariantID;

const MAX_RAW_TYPES: usize = 0;
const RAW_STAGE_TYPES: [StageType; MAX_RAW_TYPES] = [];

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
