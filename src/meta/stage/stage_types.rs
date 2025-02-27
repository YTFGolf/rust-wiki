//! Deals with transforming pure values into usable data.

/*
Plan:
- Does the equivalent of STAGE_TYPES: contains data about stages.
- This or a sibling module deals with parsing selectors. This or a sibling
  module deals with turning this information into real-world data (e.g. file
  names like `MapStageDataA_000.csv`).
*/

#![allow(dead_code)]
type StageVariantID = u32;
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
// static STAGE_TYPES : [Option<StageType<'static>>; MAX_VARIANT_NUMBER];
