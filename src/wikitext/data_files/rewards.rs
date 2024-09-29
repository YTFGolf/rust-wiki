use crate::file_handler::{get_file_location, FileLocation};
use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Deserialize)]
pub struct TreasureEntry {
    _id: u32,
    pub name: String,
}

type MapStructure = HashMap<u32, TreasureEntry>;
pub struct TreasureMap {
    map: LazyLock<MapStructure>,
}
impl TreasureMap {
    fn get_treasure(&self, id: u32) -> &TreasureEntry {
        self.map
            .get(&id)
            .unwrap_or_else(|| panic!("Treasure id not found: {id}."))
    }
    pub fn get_treasure_name(&self, id: u32) -> &str {
        &self.get_treasure(id).name
    }
}

pub static Treasure_data: TreasureMap = TreasureMap {
    map: LazyLock::new(get_treasure_data),
};

fn get_treasure_data() -> MapStructure {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(get_file_location(FileLocation::WikiData).join("Treasures.csv"));

    rdr.unwrap()
        .byte_records()
        .map(|result| {
            let map = result.unwrap().deserialize::<TreasureEntry>(None).unwrap();
            (map._id, map)
        })
        .collect()
}
