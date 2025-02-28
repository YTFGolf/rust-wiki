//! Defines information that can be used to obtain or transform pure data from
//! or into usable formats.

#![allow(dead_code)]

use super::variant::StageVariantID;
type Regex = u32;

/// Constant reference to a stage type.
struct StageType<'a> {
    /// Custom enum sort of like StageTypeEnum, but also doubles as number and
    /// can index [`STAGE_TYPES`].
    pub variant_id: StageVariantID,
    /// Long name of thing
    pub name: &'a str,
    /// Used in MapStageData. If None then needs to be custom.
    pub map_code: Option<&'a str>,
    /// Used in stage data files, if not predictable.
    pub stage_code: Option<&'a str>,
    /// Overrides `stage_code`
    pub uses_r_prefix: bool,
    pub matcher_str: &'a str,
}
const MAX_VARIANT_NUMBER: usize = 37;
// store the data, store the map
const STAGE_TYPES: [Option<StageType<'static>>; MAX_VARIANT_NUMBER] =
    [const { None }; MAX_VARIANT_NUMBER];

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
                .flat_map(|x| x)
                .any(|st| st.variant_id == variant);
            assert!(is_in_map, "Variant {variant:?} not found in STAGE_TYPES.");

            let variant_num = usize::try_from(variant.num())
                .expect("Error when converting from stage variant number to usize.");
            assert!(
                variant_num <= MAX_VARIANT_NUMBER,
                "Variant {variant:?} has a value higher than {MAX_VARIANT_NUMBER}."
            )
        }
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
