//! Transform ID data into various formats.

use super::data::get_stage_type;
use crate::meta::stage::{
    map_id::MapID, stage_id::StageID, stage_types::types::StageCodeType,
    variant::StageVariantID as T,
};

// stages

/// Get stage's data file name when stype is custom.
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

/// Get stage's data file name.
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

fn _map_data_file(_map_id: MapID) -> String {
    todo!()
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::stage_types::iter_stage_types;

    #[test]
    fn test_custom_stypes() {
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

    #[test]
    fn test_outbreak_coercion() {
        // Note: it is extremely important to test this alongside the parse
        // module.
        let stage = StageID::from_components(T::EocOutbreak, 1, 47);
        // EoC Moon 2
        assert_eq!(stage_data_file(&stage), "stageZ01_49.csv");

        let stage = StageID::from_components(T::EocOutbreak, 2, 47);
        // EoC Moon 3
        assert_eq!(stage_data_file(&stage), "stageZ02_50.csv");

        let stage = StageID::from_components(T::ItfOutbreak, 0, 47);
        // check that doesn't do the same thing for itf/cotc
        assert_eq!(stage_data_file(&stage), "stageZ04_47.csv");
    }
}
