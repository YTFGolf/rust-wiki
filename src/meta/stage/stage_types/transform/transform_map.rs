//! Transform [`MapID`] data into various formats.

use super::transform_util::CustomVariantID as T;
use crate::meta::stage::{
    map_id::{MainType, MapID},
    stage_types::{get_stage_type, types::StageCodeType},
};

/// Get map's data file name.
pub fn map_data_file(map_id: &MapID) -> String {
    let stype = get_stage_type(map_id.variant()).data;
    if let Some(code) = stype.map_code {
        return format!("MapStageData{code}_{num:03}.csv", num = map_id.num());
    }

    let variant: T = stype.variant_id.into();

    match variant {
        T::Filibuster => "stageNormal2_2_Invasion.csv".into(),
        T::MainChapters => {
            let main = map_id
                .main_type()
                .unwrap_or_else(|| panic!("Main chapter {n} out of bounds!", n = map_id.num()));

            match main {
                MainType::EoC => "stageNormal0.csv".into(),
                MainType::ItF => format!("stageNormal1_{num}.csv", num = map_id.num() - 3),
                MainType::CotC => format!("stageNormal2_{num}.csv", num = map_id.num() - 6),
            }
        }
        T::EocOutbreak | T::ItfOutbreak | T::CotcOutbreak => {
            let stype = map_id.variant().num() - 20;
            // 20 = EoC Z = stageNormal0_{stage}_Z.csv
            let num = map_id.num();
            format!("stageNormal{stype}_{num}_Z.csv")
        }
    }
}

/// Get the image code used in map and stage name files.
pub fn map_img_code(map: &MapID) -> String {
    let stype = get_stage_type(map.variant()).data;
    match stype.stage_code {
        StageCodeType::Map | StageCodeType::RPrefix => stype.map_code.unwrap().to_lowercase(),
        StageCodeType::Other(code) => code.to_lowercase(),
        StageCodeType::Custom => "main".to_lowercase(),
        // sort of a placeholder
    }
}
