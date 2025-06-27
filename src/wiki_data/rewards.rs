//! Get information about stage rewards.

use crate::wiki_data::file_handler::get_wiki_data_location;
use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Deserialize)]
/// Entry in the Treasures.csv file.
pub struct TreasureEntry {
    id: u32,
    /// Name of treasure.
    pub name: String,
}

type MapStructure = HashMap<u32, TreasureEntry>;
/// Container for [TREASURE_DATA].
pub struct TreasureMap {
    map: LazyLock<MapStructure>,
}
impl TreasureMap {
    fn get_treasure(&self, id: u32) -> &TreasureEntry {
        self.map
            .get(&id)
            .unwrap_or_else(|| panic!("Treasure id not found: {id}."))
    }
    /// Get the name of the treasure.
    pub fn get_treasure_name(&self, id: u32) -> &str {
        &self.get_treasure(id).name
    }
}

/// Contains data about treasures.
pub static TREASURE_DATA: TreasureMap = TreasureMap {
    map: LazyLock::new(get_treasure_data),
};
// TODO support Gatyaitembuy.csv

fn get_treasure_data() -> MapStructure {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(get_wiki_data_location().join("Treasures.csv"));

    rdr.unwrap()
        .byte_records()
        .map(|result| {
            let map = result.unwrap().deserialize::<TreasureEntry>(None).unwrap();
            (map.id, map)
        })
        .collect()
}
