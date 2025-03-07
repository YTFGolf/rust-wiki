//! Transform ID data into various formats.

use crate::meta::stage::variant::StageVariantID;

pub mod transform_map;
pub mod transform_stage;

/// Every [`StageVariantID`] that has a custom [`StageCodeType`].
#[derive(Debug, PartialEq)]
enum CustomVariantID {
    MainChapters,
    EocOutbreak,
    ItfOutbreak,
    CotcOutbreak,
    Filibuster,
}
impl CustomVariantID {
    fn new(variant: StageVariantID) -> Option<Self> {
        type T = StageVariantID;
        let custom = match variant {
            T::MainChapters => Self::MainChapters,
            T::EocOutbreak => Self::EocOutbreak,
            T::ItfOutbreak => Self::ItfOutbreak,
            T::CotcOutbreak => Self::CotcOutbreak,
            T::Filibuster => Self::Filibuster,
            _ => return None,
        };
        Some(custom)
    }
}
impl From<StageVariantID> for CustomVariantID {
    fn from(value: StageVariantID) -> Self {
        Self::new(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::stage_types::{iter_stage_types, types::StageCodeType};

    #[test]
    fn test_custom_stypes() {
        // test asssumptions about which stage types require custom logic
        for stype in iter_stage_types() {
            let data = stype.data;
            if data.map_code.is_none() || data.stage_code == StageCodeType::Custom {
                assert!(CustomVariantID::new(data.variant_id).is_some());
            } else {
                assert!(CustomVariantID::new(data.variant_id).is_none());
            }
        }
    }
}
