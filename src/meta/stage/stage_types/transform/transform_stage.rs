//! Transform [`StageID`] data into various formats.

use super::transform_util::CustomVariantID as T;
use crate::meta::stage::{
    map_id::MainType,
    stage_id::StageID,
    stage_types::{get_stage_type, types::StageCodeType},
};

/// Get stage's data file name when stype is custom.
fn custom_stage_data_file(stage_id: &StageID) -> String {
    let variant: T = stage_id.variant().into();
    match variant {
        T::Filibuster => "stageSpace09_Invasion_00.csv".into(),
        T::EocOutbreak | T::ItfOutbreak | T::CotcOutbreak => {
            let map = match variant {
                T::EocOutbreak => stage_id.map().num(),
                T::ItfOutbreak => stage_id.map().num() + 4,
                // Z04 = outbreak 1
                T::CotcOutbreak => stage_id.map().num() + 7,
                // Z07 = outbreak 1
                _ => unreachable!(),
            };

            let mut stage = stage_id.num();
            if variant == T::EocOutbreak {
                stage = match (map, stage) {
                    (1, 47) => 49,
                    (2, 47) => 50,
                    _ => stage,
                };
                // Chapter 2 Moon = `stageZ01_49.csv`
                // Chapter 3 Moon = `stageZ02_50.csv`
                // `parse` will transform this into stage 47 so `transform`
                // needs to transform it back.
            }

            format!("stageZ{map:02}_{stage:02}.csv")
        }
        T::MainChapters => {
            let main = stage_id.map().main_type().unwrap_or_else(|| {
                panic!("Main chapter {n} out of bounds!", n = stage_id.map().num())
            });

            match main {
                MainType::EoC => format!("stage{stage:02}.csv", stage = stage_id.num()),
                MainType::ItF => {
                    format!(
                        "stageW{map:02}_{stage:02}.csv",
                        map = stage_id.map().num() + 1,
                        // main 3 = W04
                        stage = stage_id.num()
                    )
                }
                MainType::CotC => {
                    format!(
                        "stageSpace{map:02}_{stage:02}.csv",
                        map = stage_id.map().num() + 1,
                        // main 6 = Space07
                        stage = stage_id.num()
                    )
                }
            }
        }
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
        "stage{code}{map:03}_{stage:02}.csv",
        map = stage_id.map().num(),
        stage = stage_id.num()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::{
        stage_types::parse::parse_stage::parse_stage_file, variant::StageVariantID as T,
    };

    #[test]
    fn test_general_case() {
        let earthshaker = StageID::from_components(T::SoL, 0, 0);
        assert_eq!(stage_data_file(&earthshaker), "stageRN000_00.csv");
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

        // make sure this goes both ways
        let stage = StageID::from_components(T::EocOutbreak, 1, 47);
        // EoC Moon 2
        assert_eq!(parse_stage_file(&stage_data_file(&stage)).unwrap(), stage);
    }
}
