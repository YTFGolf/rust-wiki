use super::super::iter_stage_types;
use crate::game_data::meta::stage::variant::StageVariantID;

/// Get the [`StageVariantID`] the code corresponds to.
pub fn get_variant_from_code(code: &str) -> Option<StageVariantID> {
    for stype in iter_stage_types() {
        if stype.matcher.re.is_match(code) {
            return Some(stype.data.variant_id);
        }
        // I think regex is probably faster than arr.contains
    }

    None
}

/// Variant only has a single stage.
pub fn is_single_stage(v: StageVariantID) -> bool {
    type T = StageVariantID;
    matches!(v, T::Challenge | T::Filibuster)
}

/// Variant only has a single map but multiple stages.
pub fn is_single_map(v: StageVariantID) -> bool {
    type T = StageVariantID;
    matches!(v, T::AkuRealms | T::Labyrinth)
}
