//! Parse [`MapID`] from various formats.

use super::{get_variant_from_code, is_single_map, is_single_stage, StageTypeParseError};
use crate::meta::stage::{
    map_id::{MapID, MapSize},
    stage_types::data::SELECTOR_SEPARATOR,
    variant::StageVariantID as T,
};
use regex::Regex;
use std::sync::LazyLock;

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
    LazyLock::new(|| Regex::new(r"^MapStageData([^_]*)_(\d*)\.csv$").unwrap());

/// Parse map file name into a [`MapID`].
pub fn parse_map_file(file_name: &str) -> Result<MapID, StageTypeParseError> {
    /*
    Formats:
    - stageNormal
    - MapStageData
    */
    const MAP_STAGE_DATA: &str = "MapStageData";
    const STAGE_NORMAL: &str = "stageNormal";

    if !(file_name.starts_with(STAGE_NORMAL) || file_name.starts_with(MAP_STAGE_DATA)) {
        return Err(StageTypeParseError::InvalidFormat);
    }

    // MapStageData is simpler so we do that first
    if file_name.starts_with(MAP_STAGE_DATA) {
        // I could do some really neat and efficient low-level stuff or I could
        // just whack a regex on and call it a day.
        let (_, [stype, map]): (&str, [&str; 2]) =
            MAP_STAGE_DATA_PAT.captures(file_name).unwrap().extract();
        return parse_map_selector(&format!("{stype} {map}"));
    }

    /*
    Formats:
    - stageNormal0.csv = eoc
    - stageNormal1_{num}.csv = itf
    - stageNormal2_{num}.csv = cotc
    - stageNormal{type}_{map}_Z.csv = outbreak
    - stageNormal2_2_Invasion.csv = filibuster
    */

    let mut remaining_chars = file_name[STAGE_NORMAL.len()..].chars();
    let chap_num = remaining_chars.next().unwrap().to_digit(10).unwrap();

    match remaining_chars.next().unwrap() {
        '.' => return Ok(MapID::from_components(T::MainChapters, 0)),
        // must be EoC, return
        '_' => (),
        // any other stage, do nothing
        _ => return Err(StageTypeParseError::InvalidFormat),
    }

    // - stageNormal1_{num}.csv = itf
    // - stageNormal2_{num}.csv = cotc
    // - stageNormal{type}_{map}_Z.csv = outbreak
    // - stageNormal2_2_Invasion.csv = filibuster
    // cursor is around 2_|2

    let map_num = remaining_chars.next().unwrap().to_digit(10).unwrap();

    match remaining_chars.next().unwrap() {
        '.' => {
            return Ok(MapID::from_components(
                T::MainChapters,
                chap_num * 3 + map_num,
            ))
        }
        '_' => (),
        _ => return Err(StageTypeParseError::InvalidFormat),
    }

    // - stageNormal{type}_{map}_Z.csv = outbreak
    // - stageNormal2_2_Invasion.csv = filibuster
    // cursor is around 2_2_|I

    match remaining_chars.next().unwrap() {
        'Z' => Ok(MapID::from_numbers(chap_num + 20, map_num)),
        'I' => Ok(MapID::from_components(T::Filibuster, 0)),
        // I for Invasion
        _ => Err(StageTypeParseError::InvalidFormat),
    }
}

/// Captures `["s01001"]` from
/// `"*https://battlecats-db.com/stage/s01001-999.html"`.
static DB_REFERENCE_FULL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*?https://battlecats-db.com/stage/(s\d+)[\-\d]*\.html").unwrap()
});
/// Captures `["01001"]` in `"s01001-999"`.
static DB_REFERENCE_MAP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^s(\d{5})[\-\d]*").unwrap());

/// Parse battle-cats.db reference into [`MapID`].
pub fn parse_map_ref(reference: &str) -> Result<MapID, StageTypeParseError> {
    let reference = DB_REFERENCE_FULL.replace(reference, "$1");

    match DB_REFERENCE_MAP.captures(&reference) {
        Some(cap) => {
            let mapid: u32 = cap[1].parse().unwrap();
            Ok(MapID::from_mapid(mapid))
        }
        None => Err(StageTypeParseError::InvalidFormat),
    }
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

    if variant == T::MainChapters {
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
    use crate::meta::stage::stage_types::transform::transform_map::map_data_file;
    use crate::meta::stage::variant::StageVariantID;
    use rand::random;
    use strum::IntoEnumIterator;

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

    #[test]
    fn test_from_file() {
        let st = parse_map_file("MapStageDataN_000.csv").unwrap();
        assert_eq!(st, MapID::from_components(T::SoL, 0));

        let st = parse_map_file("MapStageDataT_000.csv").unwrap();
        assert_eq!(st, MapID::from_components(T::Dojo, 0));

        let st = parse_map_file("MapStageDataL_000.csv").unwrap();
        assert_eq!(st, MapID::from_components(T::Labyrinth, 0));

        let st = parse_map_file("MapStageDataRE_000.csv").unwrap();
        assert_eq!(st, MapID::from_components(T::Extra, 0));
    }

    #[test]
    fn test_from_file_main() {
        let st = parse_map_file("stageNormal2_0.csv").unwrap();
        assert_eq!(st, MapID::from_components(T::MainChapters, 6));

        let st = parse_map_file("stageNormal0_0_Z.csv").unwrap();
        assert_eq!(st, MapID::from_components(T::EocOutbreak, 0));
    }

    #[test]
    fn test_from_ref() {
        let answer = MapID::from_components(T::SoL, 0);

        let st = parse_map_ref("*https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = parse_map_ref("https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = parse_map_ref("s00000-01").unwrap();
        assert_eq!(st, answer);

        let st = parse_map_ref("*https://battlecats-db.com/stage/s00000.html").unwrap();
        assert_eq!(st, answer);
        let st = parse_map_ref("https://battlecats-db.com/stage/s00000.html").unwrap();
        assert_eq!(st, answer);
        let st = parse_map_ref("s00000").unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_general_parse() {
        let selector = "*https://battlecats-db.com/stage/s01382-03.html";
        assert_eq!(
            parse_map_ref(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_general_map_id(selector).unwrap(),
            MapID::from_components(T::Event, 382)
        );

        let selector = "ItF 1 48";
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            MapID::from_components(T::MainChapters, 3)
        );

        let selector = "DM 0";
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            MapID::from_components(T::AkuRealms, 0)
        );

        let selector = "Filibuster";
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            MapID::from_components(T::Filibuster, 0)
        );

        let selector = "itfz 1 0";
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_map_selector(selector).unwrap(),
            MapID::from_components(T::ItfOutbreak, 1)
        );

        let selector = "stageRN013_05.csv";
        assert_eq!(
            parse_map_file(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_map_file(selector).unwrap(),
            MapID::from_components(T::SoL, 13)
        );

        let selector = "stageRN000_00.csv";
        assert_eq!(
            parse_map_file(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_map_file(selector).unwrap(),
            MapID::from_components(T::SoL, 0)
        );

        let selector = "stageW04_05.csv";
        assert_eq!(
            parse_map_file(selector).unwrap(),
            parse_general_map_id(selector).unwrap()
        );
        assert_eq!(
            parse_map_file(selector).unwrap(),
            MapID::from_components(T::MainChapters, 3)
        );

        let selector = "stageW04_05.csv";
        assert_eq!(
            parse_general_map_id(&String::from(selector)),
            parse_general_map_id(selector)
        );
        assert_eq!(
            parse_general_map_id(&String::from(selector)).unwrap(),
            MapID::from_components(T::MainChapters, 3)
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
                let map = if is_single_stage(var) || is_single_map(var) {
                    0
                } else if var.is_outbreak() {
                    random::<MapSize>() % 3
                } else {
                    random::<MapSize>() % 1000
                };

                // assert all parse functions get the same result and the
                // map file stuff is bidirectional
                let st = MapID::from_components(var, map);
                let file_name = map_data_file(&st);
                assert_eq!(
                    file_name,
                    map_data_file(&parse_map_file(&file_name).unwrap())
                );
                assert_eq!(
                    st,
                    parse_map_selector(&format!("{} {map}", var.num())).unwrap()
                );
                assert_eq!(
                    st,
                    parse_map_ref(&format!("s{:02}{:03}", var.num(), map)).unwrap()
                );
            }
        }
    }

    #[test]
    fn test_random_properties_main() {
        const NUM_ITERATIONS: usize = 20;
        let var = T::MainChapters;

        // eoc
        for _ in 0..1 {
            let (map) = (0);
            // can only have 2 digits

            let st = MapID::from_components(var, map);
            let file_name = map_data_file(&st);
            assert_eq!(
                file_name,
                map_data_file(&parse_map_file(&file_name).unwrap())
            );
            assert_eq!(
                st,
                parse_map_selector(&format!("{} {map}", var.num())).unwrap()
            );
            assert_eq!(
                st,
                parse_map_ref(&format!("s{:02}{:03}", var.num(), map)).unwrap()
            );
        }

        // itf
        for _ in 0..NUM_ITERATIONS {
            let (map) = (random::<MapSize>() % 3 + 3);

            let st = MapID::from_components(var, map);
            let file_name = map_data_file(&st);
            assert_eq!(
                file_name,
                map_data_file(&parse_map_file(&file_name).unwrap())
            );
            assert_eq!(
                st,
                parse_map_selector(&format!("{} {map}", var.num())).unwrap()
            );
            assert_eq!(
                st,
                parse_map_ref(&format!("s{:02}{:03}", var.num(), map)).unwrap()
            );
        }

        // cotc
        for _ in 0..NUM_ITERATIONS {
            let (map) = (random::<MapSize>() % 3 + 6);

            let st = MapID::from_components(var, map);
            let file_name = map_data_file(&st);
            assert_eq!(
                file_name,
                map_data_file(&parse_map_file(&file_name).unwrap())
            );
            assert_eq!(
                st,
                parse_map_selector(&format!("{} {map}", var.num())).unwrap()
            );
            assert_eq!(
                st,
                parse_map_ref(&format!("s{:02}{:03}", var.num(), map)).unwrap()
            );
        }
    }
}
