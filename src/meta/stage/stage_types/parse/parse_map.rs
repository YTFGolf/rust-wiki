use super::{get_variant_from_code, is_single_map, is_single_stage, StageTypeParseError};
use crate::meta::stage::{
    map_id::{MapID, MapSize},
    stage_types::data::SELECTOR_SEPARATOR,
    variant::StageVariantID,
};

pub fn parse_general_map_id(selector: &str) -> MapID {
    todo!()
}

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

    let Some(map_num) = iter.next() else {
        return Err(StageTypeParseError::NoMapNumber);
    };
    let Ok(map_num) = map_num.parse::<MapSize>() else {
        return Err(StageTypeParseError::InvalidNumber);
    };

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
                assert!((3..=5).contains(&map_num));
                return Ok(MapID::from_components(variant, map_num));
            }
            "cotc" | "space" => {
                let map_num = map_num + 5;
                // cotc 1 = "cotc 1" = "main 6"
                assert!((6..=8).contains(&map_num));
                return Ok(MapID::from_components(variant, map_num));
            }
            _ => (),
            // if you put main or 3 then I assume you know what you're doing
        }
    }

    Ok(MapID::from_components(variant, map_num))
}

// fn parse_map_from_iterator<'a, T>()
// where
//     T: Iterator<Item = &'a str>,
// {
//     todo!()
// }
// I'm okay with this being a monolith for now.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::stage::stage_types::data::get_stage_type;

    #[test]
    fn assert_main_selector() {
        // DO NOT CHANGE THIS TEST WITHOUT UPDATING `parse_map_selector`
        let desired: Vec<&str> = "main|EoC|ItF|W|CotC|Space|3".split('|').collect();
        let main = get_stage_type(StageVariantID::MainChapters);
        assert_eq!(desired, main.matcher.arr);
    }

    #[test]
    #[should_panic = "assertion failed: (3..=5).contains(&map_num)"]
    fn test_invalid_number_low_itf() {
        let _ = parse_map_selector("itf 0");
    }

    #[test]
    #[should_panic = "assertion failed: (6..=8).contains(&map_num)"]
    fn test_invalid_number_low_cotc() {
        let _ = parse_map_selector("cotc 0");
    }

    #[test]
    #[should_panic = "assertion failed: (6..=8).contains(&map_num)"]
    fn test_invalid_number_high() {
        let _ = parse_map_selector("cotc 4");
    }
}
