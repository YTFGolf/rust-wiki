//! Transform ID data into various formats.

use crate::meta::stage::map_id::MapID;
pub mod transform_stage;

// maps

fn _map_data_file(map_id: MapID) -> String {
    // if let Some(code) = map_id {

    // }
    todo!()
}

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
