//! Transform ID data into various formats.

use super::data::get_stage_type;
use crate::meta::stage::{
    map_id::MapID, stage_id::StageID, stage_types::types::StageCodeType, variant::StageVariantID,
};

// stages

type T = StageVariantID;
fn custom_stage_data_file(stage_id: &StageID) -> String {
    // assert!(matches!(
    //     stage_id.variant(),
    //     T::MainChapters | T::EocOutbreak | T::ItfOutbreak | T::CotcOutbreak | T::Filibuster
    // ));
    // main, zombie, filibuster
    // test to assure this are in `test_custom_stypes`
    match stage_id.variant() {
        T::Filibuster => "stageSpace09_Invasion_00.csv".to_string(),
        T::EocOutbreak | T::ItfOutbreak | T::CotcOutbreak => {
            let map = match stage_id.variant() {
                T::EocOutbreak => stage_id.map().num(),
                T::ItfOutbreak => stage_id.map().num() + 4,
                // Z04 = outbreak 1
                T::CotcOutbreak => stage_id.map().num() + 7,
                // Z07 = outbreak 1
                _ => unreachable!(),
            };

            format!("stageZ{map:02}_{stage:02}.csv", stage = stage_id.num())
        }
        T::MainChapters => match stage_id.map().num() {
            0 => format!("stage{stage:02}.csv", stage = stage_id.num()),
            3..=5 => {
                format!(
                    "stageW{map:02}_{stage:02}.csv",
                    map = stage_id.map().num() + 1,
                    // main 3 = W04
                    stage = stage_id.num()
                )
            }
            6..=8 => {
                format!(
                    "stageSpace{map:02}_{stage:02}.csv",
                    map = stage_id.map().num() + 1,
                    // main 6 = Space07
                    stage = stage_id.num()
                )
            }
            n => panic!("Main chapter {n} out of bounds!"),
        },
        _ => unreachable!(),
    }
}

pub fn stage_data_file(stage_id: &StageID) -> String {
    type C = StageCodeType;
    let stype = get_stage_type(stage_id.variant()).data;

    let code = match stype.stage_code {
        C::Map => stype.map_code.unwrap().to_string(),
        C::RPrefix => "R".to_string() + stype.map_code.unwrap(),
        C::Other(o) => o.to_string(),
        C::Custom => return custom_stage_data_file(stage_id),
    };
    // to avoid tostringing I could use buffers but idc

    format!(
        "stage{code}{map:03}_{stage:03}.csv",
        map = stage_id.map().num(),
        stage = stage_id.num()
    )
}

// -----------------------------------------------------------------------------

// maps

fn map_data_file(map_id: MapID) -> String {
    todo!()
}

// -----------------------------------------------------------------------------

// general

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::variant::StageVariantID;
    use strum::IntoEnumIterator;

    #[test]
    fn test_custom_stypes() {
        for var in StageVariantID::iter() {
            let stype = get_stage_type(var).data;
            if stype.map_code == None || stype.stage_code == StageCodeType::Custom {
                assert!(matches!(
                    var,
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
