//! Transform ID data into various formats.

pub mod transform_map;
pub mod transform_stage;

#[cfg(test)]
mod tests {
    use crate::meta::stage::{
        stage_types::{iter_stage_types, types::StageCodeType},
        variant::StageVariantID as T,
    };

    #[test]
    fn test_custom_stypes() {
        // test asssumptions about which stage types require custom logic
        for stype in iter_stage_types() {
            let data = stype.data;
            if data.map_code.is_none() || data.stage_code == StageCodeType::Custom {
                assert!(matches!(
                    data.variant_id,
                    T::MainChapters
                        | T::EocOutbreak
                        | T::ItfOutbreak
                        | T::CotcOutbreak
                        | T::Filibuster
                ));
            }
        }
    }
}
