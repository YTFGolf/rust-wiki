//! Module that deals with the `Map_option` file.

use crate::data::version::version_data::CacheableVersionData;
use csv::ByteRecord;
use std::{collections::HashMap, num::NonZero, path::Path};

#[derive(Debug, serde::Deserialize)]
/// Data stored in the map option CSV.
pub struct MapOptionCSV {
    /// ID of map.
    ///
    /// Roughly follows the format of `str(type_id * 1000 + map_id)`, except for
    /// CotC Zombie Outbreaks where it's `str(22000 + map_id)`. Also used in
    /// [stage_option][map_id].
    ///
    /// [map_id]: crate::data::stage::raw::stage_option::StageOptionCSV::map_id
    pub map_id: u32,
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
    /// Reset type e.g. Facing Danger disappears after clearing once. Values are
    /// unclear.
    _reset_type: u32,
    /// Maximum map clears before resets or something (unclear).
    pub max_clears: u32,
    /// "Display order", no clue.
    _表示順: u32,
    /// Gauntlet cooldown.
    pub cooldown: u32,
    /// "Challenge flag".
    _挑戦フラグ: u32,
    /// Binary representation of map's star difficulty.
    pub star_mask: u16,
    /// "Hide after clearing".
    // TODO figure out difference between this and max_clears
    _クリア後非表示: u32,
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
    /// Get the map data that `map_id` corresponds to.
    pub fn get_map(&self, map_id: u32) -> Option<MapOptionCSV> {
        Some(
            self.map
                .get_key_value(&map_id)?
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
    use crate::config::CONFIG;
    use std::{collections::HashSet, io::Cursor};

    #[test]
    fn test_mo() {
        let s = CONFIG
            .current_version
            .get_cached_file::<MapOption>()
            .get_map(0)
            .unwrap();

        assert_eq!(s.crown_4, 300);
    }

    /// Assert that 1 <= difficulty <= 4, map id is not in `seen`, crown 1 =
    /// 100.
    fn assert_conditions(record_parsed: MapOptionCSV, seen: &HashSet<u32>) -> u32 {
        let map_id = record_parsed.map_id;
        let d: u8 = record_parsed.max_difficulty.into();

        assert!((1..=4).contains(&d));
        assert!(!seen.contains(&map_id));
        assert_eq!(record_parsed._crown_1, 100);
        map_id
    }

    #[test]
    fn assert_parses_and_no_duplicates_and_correct_fields() {
        let version = &CONFIG.current_version;
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

            let map_id = assert_conditions(record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic]
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

            let map_id = assert_conditions(record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic]
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

            let map_id = assert_conditions(record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic]
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

            let map_id = assert_conditions(record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic]
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

            let map_id = assert_conditions(record_parsed, &seen);
            seen.insert(map_id);
        }
    }

    #[test]
    #[should_panic]
    fn assert_crown_1_checker_works() {
        // line is "0,4,100,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり"
        let reader = Cursor::new(
            "0,5,101,150,200,300,0,0,0,0,0,0,7,0,0,レジェンドステージ：伝説のはじまり\n\
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

            let map_id = assert_conditions(record_parsed, &seen);
            seen.insert(map_id);
        }
    }
}
