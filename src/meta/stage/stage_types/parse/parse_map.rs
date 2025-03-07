//! Parse [`MapID`] from various formats.

use super::{get_variant_from_code, is_single_map, is_single_stage, StageTypeParseError};
use crate::meta::stage::{
    map_id::{MapID, MapSize},
    stage_types::data::SELECTOR_SEPARATOR,
    variant::StageVariantID,
};
use regex::Regex;
use std::sync::LazyLock;

fn _parse_general_map_id(_selector: &str) -> MapID {
    todo!()
}
/// Parse string of unknown format into a [`MapID`].
pub fn parse_general_map_id(selector: &str) -> Option<MapID> {
    if let Ok(st) = parse_map_file(selector) {
        return Some(st);
    };
    if let Ok(st) = parse_map_selector(selector) {
        return Some(st);
    };
    if let Ok(st) = parse_map_ref(selector) {
        return Some(st);
    };

    None
}

/// Captures `["A", "000"]` from `"MapStageDataA_000.csv"`.
static MAP_STAGE_DATA_PAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^MapStageData([^_]*)_(\d*).csv$").unwrap());

/// Parse map file name into a [`MapID`].
pub fn parse_map_file(file_name: &str) -> Result<MapID, StageTypeParseError> {
    /*
    Formats:
    - stageNormal
    - MapStageData
    */
    // MapStageData is simpler so we do that first
    if file_name.starts_with("MapStageData") {
        // I could do some really neat and efficient low-level stuff or I could
        // just whack a regex on and call it a day.
        let (_, [stype, map]): (&str, [&str; 2]) =
            MAP_STAGE_DATA_PAT.captures(file_name).unwrap().extract();
        return parse_map_selector(&(stype.to_string() + map));
    }

    /*
    Formats:
    - stageNormal0.csv = eoc
    - stageNormal1_{num}.csv = itf
    - stageNormal2_{num}.csv = cotc
    - stageNormal{type}_{map}_Z.csv = outbreak
    - stageNormal2_2_Invasion.csv = filibuster
    */
    todo!()
}

/// Parse battle-cats.db reference into [`MapID`].
pub fn parse_map_ref(selector: &str) -> Result<MapID, StageTypeParseError> {
    todo!()
}

/// Parse map selector to [`MapID`].
pub fn parse_map_selector(selector: &str) -> Result<MapID, StageTypeParseError> {
    let mut iter = selector.split(SELECTOR_SEPARATOR);
    let compare = iter
        .next()
        .expect("I literally have no clue how this would fail.");

    let variant = match get_variant_from_code(compare) {
        None => return Err(StageTypeParseError::UnknownMatcher),
        Some(v) => v,
    };

    if is_single_stage(variant) || is_single_map(variant) {
        // if type only has 1 stage/map then map num will always be 0
        return Ok(MapID::from_components(variant, 0));
    };

    let map_num = iter
        .next()
        .ok_or(StageTypeParseError::NoMapNumber)?
        .parse::<MapSize>()
        .map_err(|_| StageTypeParseError::InvalidNumber)?;

    if variant == StageVariantID::MainChapters {
        // has to have separate logic depending on what you put as your selector

        // THIS IS HARDCODED, DO NOT UPDATE THIS WITHOUT UPDATING
        // `assert_main_selector`
        match compare.to_lowercase().as_str() {
            "eoc" => return Ok(MapID::from_components(variant, 0)),
            // eoc has 1 chapter that is number 0
            "itf" | "w" => {
                let map_num = map_num + 2;
                // itf 1 = "itf 1" = "main 3"
                // assert!((3..=5).contains(&map_num));
                return Ok(MapID::from_components(variant, map_num));
            }
            "cotc" | "space" => {
                let map_num = map_num + 5;
                // cotc 1 = "cotc 1" = "main 6"
                // assert!((6..=8).contains(&map_num));
                return Ok(MapID::from_components(variant, map_num));
            }
            _ => (),
            // if you put main or 3 then I assume you know what you're doing
        }
    }

    Ok(MapID::from_components(variant, map_num))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::stage_types::data::get_stage_type;

    #[test]
    fn assert_main_selector() {
        // DO NOT CHANGE THIS TEST WITHOUT UPDATING `parse_map_selector`

        // Basically just checks that the hardcoded match values are accurate.
        // It should always be the below values in lowercase, excluding main and
        // 3 because those will use consistent map numbers.

        let desired: Vec<&str> = "main|EoC|ItF|W|CotC|Space|3".split('|').collect();
        let main = get_stage_type(StageVariantID::MainChapters);
        assert_eq!(desired, main.matcher.arr);
    }

    // #[test]
    // #[should_panic = "assertion failed: (3..=5).contains(&map_num)"]
    // fn test_invalid_number_low_itf() {
    //     let _ = parse_map_selector("itf 0");
    // }

    // #[test]
    // #[should_panic = "assertion failed: (6..=8).contains(&map_num)"]
    // fn test_invalid_number_low_cotc() {
    //     let _ = parse_map_selector("cotc 0");
    // }

    // #[test]
    // #[should_panic = "assertion failed: (6..=8).contains(&map_num)"]
    // fn test_invalid_number_high() {
    //     let _ = parse_map_selector("cotc 4");
    // }
    /*
        #[test]
        fn test_from_file() {
            let st = parse_stage_file("stageRN000_00.csv").unwrap();
            assert_eq!(st, StageID::from_components(T::SoL, 0, 0));

            let st = parse_stage_file("stageRT000_00.csv").unwrap();
            assert_eq!(st, StageID::from_components(T::Dojo, 0, 0));

            let st = parse_stage_file("stageL000_00.csv").unwrap();
            assert_eq!(st, StageID::from_components(T::Labyrinth, 0, 0));

            let st = parse_stage_file("stageEX000_00.csv").unwrap();
            assert_eq!(st, StageID::from_components(T::Extra, 0, 0));
        }

        #[test]
        fn test_from_file_main() {
            let st = parse_stage_file("stageSpace07_00.csv").unwrap();
            assert_eq!(st, StageID::from_components(T::MainChapters, 6, 0));

            let st = parse_stage_file("stageZ00_00.csv").unwrap();
            assert_eq!(st, StageID::from_components(T::EocOutbreak, 0, 0));
        }


        #[test]
        fn test_from_ref() {
            let answer = StageID::from_components(T::SoL, 0, 0);

            let st = parse_stage_ref("*https://battlecats-db.com/stage/s00000-01.html").unwrap();
            assert_eq!(st, answer);
            let st = parse_stage_ref("https://battlecats-db.com/stage/s00000-01.html").unwrap();
            assert_eq!(st, answer);
            let st = parse_stage_ref("s00000-01").unwrap();
            assert_eq!(st, answer);
        }

        #[test]
        fn test_general_parse() {
            let selector = "*https://battlecats-db.com/stage/s01382-03.html";
            assert_eq!(
                parse_stage_ref(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_general_stage_id(selector).unwrap(),
                StageID::from_components(T::Event, 382, 2)
            );

            let selector = "ItF 1 48";
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                StageID::from_components(T::MainChapters, 3, 48)
            );

            let selector = "DM 0";
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                StageID::from_components(T::AkuRealms, 0, 0)
            );

            let selector = "Filibuster";
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                StageID::from_components(T::Filibuster, 0, 0)
            );

            let selector = "itfz 1 0";
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_stage_selector(selector).unwrap(),
                StageID::from_components(T::ItfOutbreak, 1, 0)
            );

            let selector = "stageRN013_05.csv";
            assert_eq!(
                parse_stage_file(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_stage_file(selector).unwrap(),
                StageID::from_components(T::SoL, 13, 5)
            );

            let selector = "stageRN000_00.csv";
            assert_eq!(
                parse_stage_file(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_stage_file(selector).unwrap(),
                StageID::from_components(T::SoL, 0, 0)
            );

            let selector = "stageW04_05.csv";
            assert_eq!(
                parse_stage_file(selector).unwrap(),
                parse_general_stage_id(selector).unwrap()
            );
            assert_eq!(
                parse_stage_file(selector).unwrap(),
                StageID::from_components(T::MainChapters, 3, 5)
            );

            let selector = "stageW04_05.csv";
            assert_eq!(
                parse_general_stage_id(&String::from(selector)),
                parse_general_stage_id(selector)
            );
            assert_eq!(
                parse_general_stage_id(&String::from(selector)).unwrap(),
                StageID::from_components(T::MainChapters, 3, 5)
            );
        }

        #[test]
        fn test_random_properties() {
            const NUM_ITERATIONS: usize = 20;
            for var in StageVariantID::iter() {
                if var == T::MainChapters {
                    // main will need to be a bit more delicate
                    continue;
                }

                for _ in 0..NUM_ITERATIONS {
                    let (map, stage) = if is_single_stage(var) {
                        (0, 0)
                    } else if is_single_map(var) {
                        (0, random::<StageSize>() % 1000)
                    } else if var.is_outbreak() {
                        (random::<MapSize>() % 3, random::<StageSize>() % 1000)
                    } else {
                        (random::<MapSize>() % 1000, random::<StageSize>() % 1000)
                    };

                    // assert all parse functions get the same result and the stage
                    // file stuff is bidirectional
                    let st = StageID::from_components(var, map, stage);
                    let file_name = stage_data_file(&st);
                    assert_eq!(
                        file_name,
                        stage_data_file(&parse_stage_file(&file_name).unwrap())
                    );
                    assert_eq!(
                        st,
                        parse_stage_selector(&format!("{} {map} {stage}", var.num())).unwrap()
                    );
                    assert_eq!(
                        st,
                        parse_stage_ref(&format!("s{:02}{:03}-{:02}", var.num(), map, stage + 1))
                            .unwrap()
                    );
                }
            }
        }

        #[test]
        fn test_random_properties_main() {
            const NUM_ITERATIONS: usize = 20;
            let var = T::MainChapters;

            // eoc
            for _ in 0..NUM_ITERATIONS {
                let (map, stage) = (0, random::<StageSize>() % 100);
                // can only have 2 digits

                let st = StageID::from_components(var, map, stage);
                let file_name = stage_data_file(&st);
                assert_eq!(
                    file_name,
                    stage_data_file(&parse_stage_file(&file_name).unwrap())
                );
                assert_eq!(
                    st,
                    parse_stage_selector(&format!("{} {map} {stage}", var.num())).unwrap()
                );
                assert_eq!(
                    st,
                    parse_stage_ref(&format!("s{:02}{:03}-{:02}", var.num(), map, stage + 1)).unwrap()
                );
            }

            // itf
            for _ in 0..NUM_ITERATIONS {
                let (map, stage) = (random::<MapSize>() % 3 + 3, random::<StageSize>() % 1000);

                let st = StageID::from_components(var, map, stage);
                let file_name = stage_data_file(&st);
                assert_eq!(
                    file_name,
                    stage_data_file(&parse_stage_file(&file_name).unwrap())
                );
                assert_eq!(
                    st,
                    parse_stage_selector(&format!("{} {map} {stage}", var.num())).unwrap()
                );
                assert_eq!(
                    st,
                    parse_stage_ref(&format!("s{:02}{:03}-{:02}", var.num(), map, stage + 1)).unwrap()
                );
            }

            // cotc
            for _ in 0..NUM_ITERATIONS {
                let (map, stage) = (random::<MapSize>() % 3 + 6, random::<StageSize>() % 1000);

                let st = StageID::from_components(var, map, stage);
                let file_name = stage_data_file(&st);
                assert_eq!(
                    file_name,
                    stage_data_file(&parse_stage_file(&file_name).unwrap())
                );
                assert_eq!(
                    st,
                    parse_stage_selector(&format!("{} {map} {stage}", var.num())).unwrap()
                );
                assert_eq!(
                    st,
                    parse_stage_ref(&format!("s{:02}{:03}-{:02}", var.num(), map, stage + 1)).unwrap()
                );
            }
        }
    */
}
