use super::super::iter_stage_types;
use crate::meta::stage::variant::StageVariantID;

#[derive(Debug, PartialEq)]
/// Error when parsing the stage type.
pub enum StageTypeParseError {
    /// Invalid "matcher" (variant code, e.g. `"main"`).
    UnknownMatcher,
    /// No map number provided when necessary.
    NoMapNumber,
    /// No stage number provided when necessary.
    NoStageNumber,
    /// Map or stage number is invalid (e.g. negative, contains letters).
    InvalidNumber,
    /// Selector is not in a valid format for the given function (e.g. is a file
    /// name when the function is db refs).
    InvalidFormat,
}

/// Get the [`StageVariantID`] the code corresponds to.
fn get_variant_from_code(code: &str) -> Option<StageVariantID> {
    for stype in iter_stage_types() {
        if stype.matcher.re.is_match(code) {
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
