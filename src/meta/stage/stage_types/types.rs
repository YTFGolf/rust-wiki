//! Defines types to use in this module.

use super::super::variant::StageVariantID;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
/// Constant reference to a stage type.
pub struct StageType {
    /// Variant ID of the stage type.
    pub variant_id: StageVariantID,
    /// Full readable name of the stage type.
    pub name: &'static str,
    /// Code used in map data files. None means that it will need to be figured
    /// out manually.
    pub map_code: Option<&'static str>,
    /// Code used in stage data files.
    pub stage_code: StageCodeType,
    /// Regex matcher for the stage type's common name.
    pub common_name_match_str: &'static str,
}
