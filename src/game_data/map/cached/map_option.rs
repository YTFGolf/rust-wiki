//! Module that deals with the `Map_option` file.

use crate::game_data::{meta::stage::map_id::MapID, version::version_data::CacheableVersionData};
use csv::ByteRecord;
use std::{collections::HashMap, num::NonZero, path::Path};

#[derive(Debug, serde::Deserialize)]
/// Data stored in the map option CSV.
pub struct MapOptionCSV {
    /// Map's mapid.
    pub mapid: u32,
    /// Highest crown difficulty the stage goes up to.
    pub max_difficulty: NonZero<u8>,
    /// Magnification on 1-crown difficulty (will always be 100).
    _crown_1: u32,
    /// Magnification on 2-crown difficulty.
    pub crown_2: u32,
    /// Magnification on 3-crown difficulty.
    pub crown_3: u32,
    /// Magnification on 4-crown difficulty.
    pub crown_4: u32,

    /// Type of stage? (E.g. Catfruit, XP).
    _ゲリラset: u32,
    /// Reset type. See [`ResetType`][ResetType].
    ///
    /// [ResetType]: crate::data::map::parsed::map::ResetType
    pub reset_type: u8,
    /// Amount of stages that can be cleared before the map disappears. If
    /// `cooldown` is set, the event goes into cooldown rather than
    /// disappearing.
    pub max_clears: u32,
    /// Probably something to do with where it appears on the legend stages
    /// screen.
    pub display_order: u32,
    /// Gauntlet cooldown.
    pub cooldown: u32,
    /// "Challenge flag".
    _挑戦フラグ: u32,
    /// Binary representation of map's star difficulty.
    pub star_mask: u16,
    /// Hide the map when it is cleared.
    pub hidden_upon_clear: u8,
    /// Something to do with double xp ads?
    _xp2倍広告: u32,
    /// Don't trust this.
    #[serde(skip)]
    _jpname: &'static str,
}

#[derive(Debug)]
/// Container for map option data.
pub struct MapOption {
    map: HashMap<u32, ByteRecord>,
}
impl CacheableVersionData for MapOption {
    fn init_data(path: &std::path::Path) -> Self {
        Self {
            map: get_map_option(path),
        }
    }
}
impl MapOption {
    /// Get the map option data for map if exists.
    pub fn get_map(&self, map_id: &MapID) -> Option<MapOptionCSV> {
        Some(
            self.map
                .get_key_value(&map_id.mapid())?
                .1
                .deserialize(None)
                .unwrap(),
        )
    }
}

fn get_map_option(path: &Path) -> HashMap<u32, ByteRecord> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        // technically does have headers but that's an issue for another day
        .flexible(true)
        .from_path(path.join("DataLocal/Map_option.csv"))
        .unwrap();

    let mut records = rdr.byte_records();
    records.next();

    let records_iter = records.map(|record| {
        let result = record.unwrap();
        (
            std::str::from_utf8(&result[0])
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            result,
        )
    });

    records_iter.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;
    use std::{collections::HashSet, io::Cursor};

    #[test]
    fn test_mo() {
        let s = TEST_CONFIG
            .version
            .current_version()
            .get_cached_file::<MapOption>()
            .get_map(&MapID::from_numbers(0, 0))
            .unwrap();

        assert_eq!(s.crown_4, 300);
    }

    /// Assert that 1 <= difficulty <= 4, map id is not in `seen`, crown 1 =
    /// 100.
    #[allow(clippy::used_underscore_binding)]
    fn assert_conditions(record_parsed: &MapOptionCSV, seen: &HashSet<u32>) -> u32 {
        let map_id = record_parsed.mapid;
        let d: u8 = record_parsed.max_difficulty.into();

        assert!((1..=4).contains(&d));
        assert!(!seen.contains(&map_id));
        assert_eq!(record_parsed._crown_1, 100);
        map_id
    }

    #[test]
    fn assert_parses_and_no_duplicates_and_correct_fields() {
        let version = &TEST_CONFIG.version.current_version();
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // technically does have headers but that's an issue for another day
            .flexible(true)
            .from_path(version.get_file_path("DataLocal/Map_option.csv"))
            .unwrap();

        let mut rdr = rdr;
        let mut records = rdr.byte_records();
        records.next();

        let mut seen = HashSet::<u32>::new();
        for result in records {
            let record = result.unwrap();
            let record_parsed: MapOptionCSV = record.deserialize(None).unwrap();

            let map_id = assert_conditions(&record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic = "err: DeserializeError { field: Some(2), kind: ParseInt(ParseIntError { kind: InvalidDigit }) }"]
    // -100 is not a valid u32
    fn assert_parse_checker_works() {
        // line is "0,4,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり"
        let reader = Cursor::new(
            "0,4,-100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり",
        );
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // technically does have headers but that's an issue for another day
            .flexible(true)
            .from_reader(reader);

        let mut rdr = rdr;
        let records = rdr.byte_records();

        let mut seen = HashSet::<u32>::new();
        for result in records {
            let record = result.unwrap();
            let record_parsed: MapOptionCSV = record.deserialize(None).unwrap();

            let map_id = assert_conditions(&record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic = "assertion failed: !seen.contains(&map_id)"]
    // two lines with same map id so should fail
    fn assert_dupe_checker_works() {
        // line is "0,4,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり"
        let reader = Cursor::new(
            "0,4,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり\n\
             0,2,100,120,150,150,0,0,0,8000,0,0,1536,0,0,異次元コロシアム",
        );
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // technically does have headers but that's an issue for another day
            .flexible(true)
            .from_reader(reader);

        let mut rdr = rdr;
        let records = rdr.byte_records();

        let mut seen = HashSet::<u32>::new();
        for result in records {
            let record = result.unwrap();
            let record_parsed: MapOptionCSV = record.deserialize(None).unwrap();

            let map_id = assert_conditions(&record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic = "Message(\"invalid value: integer `0`, expected a nonzero u8\")"]
    // fails on first line as crown is 0
    fn assert_difficulty_checker_works_0() {
        // line is "0,4,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり"
        let reader = Cursor::new(
            "0,0,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり\n\
             0,2,100,120,150,150,0,0,0,8000,0,0,1536,0,0,異次元コロシアム",
        );
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // technically does have headers but that's an issue for another day
            .flexible(true)
            .from_reader(reader);

        let mut rdr = rdr;
        let records = rdr.byte_records();

        let mut seen = HashSet::<u32>::new();
        for result in records {
            let record = result.unwrap();
            let record_parsed: MapOptionCSV = record.deserialize(None).unwrap();

            let map_id = assert_conditions(&record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic = "assertion failed: (1..=4).contains(&d)"]
    // fails on first line as crown is 5
    fn assert_difficulty_checker_works_5() {
        // line is "0,4,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり"
        let reader = Cursor::new(
            "0,5,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり\n\
             0,2,100,120,150,150,0,0,0,8000,0,0,1536,0,0,異次元コロシアム",
        );
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // technically does have headers but that's an issue for another day
            .flexible(true)
            .from_reader(reader);

        let mut rdr = rdr;
        let records = rdr.byte_records();

        let mut seen = HashSet::<u32>::new();
        for result in records {
            let record = result.unwrap();
            let record_parsed: MapOptionCSV = record.deserialize(None).unwrap();

            let map_id = assert_conditions(&record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic = "left: 101\n right: 100"]
    // fails on first line as 1-crown mag is not 100
    fn assert_crown_1_checker_works() {
        // line is "0,4,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり"
        let reader = Cursor::new(
            "0,4,101,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり\n\
             0,2,100,120,150,150,0,0,0,8000,0,0,1536,0,0,異次元コロシアム",
        );
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            // technically does have headers but that's an issue for another day
            .flexible(true)
            .from_reader(reader);

        let mut rdr = rdr;
        let records = rdr.byte_records();

        let mut seen = HashSet::<u32>::new();
        for result in records {
            let record = result.unwrap();
            let record_parsed: MapOptionCSV = record.deserialize(None).unwrap();

            let map_id = assert_conditions(&record_parsed, &seen);
            seen.insert(map_id);
        }
    }
}
