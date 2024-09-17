use std::{collections::HashMap, sync::LazyLock};

use csv::ByteRecord;

use crate::file_handler::{get_file_location, FileLocation};

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
/// Data stored in the map option CSV.
pub struct MapOptionCSV {
    /// ID of map.
    ///
    /// Roughly follows the format of `str(type_id * 1000 + map_id)`, except for
    /// CotC Zombie Outbreaks where it's `str(22000 + map_id)`. Also used in
    /// [stage_option].
    pub map_id: u32,
    /// Highest crown difficulty the stage goes up to.
    pub stars_max: u32,
    /// Magnification on 1-crown difficulty.
    pub star1: u32,
    /// Magnification on 2-crown difficulty.
    pub star2: u32,
    /// Magnification on 3-crown difficulty.
    pub star3: u32,
    /// Magnification on 4-crown difficulty.
    pub star4: u32,

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
    pub star_mask: u32,
    /// "Hide after clearing".
    // TODO figure out difference between this and max_clears
    _クリア後非表示: u32,
    /// Something to do with double xp ads?
    _xp2倍広告: u32,
    /// Don't trust this.
    _jpname: &'static str,
}

/// Hashmap of the `"DataLocal/Map_option.csv"` file. Individual records are left unparsed.
pub static MAP_OPTION: LazyLock<HashMap<u32, ByteRecord>> = LazyLock::new(|| get_map_option());

fn get_map_option() -> HashMap<u32, ByteRecord> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        // technically does have headers but that's an issue for another day
        .flexible(true)
        .from_path(get_file_location(FileLocation::GameData).join("DataLocal/Map_option.csv"))
        .unwrap();

    let mut records = rdr.byte_records();
    records.next();

    let records_iter = records.into_iter().map(|a| {
        let b = a.unwrap();
        (
            std::str::from_utf8(&b[0]).unwrap().parse::<u32>().unwrap(),
            b,
        )
    });

    records_iter.collect()
}
