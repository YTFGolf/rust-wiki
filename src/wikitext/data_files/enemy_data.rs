//! Module that gets information about enemy names and data.

use std::{collections::HashMap, sync::LazyLock};

use serde::Deserialize;

use crate::file_handler::{get_file_location, FileLocation};

#[derive(Debug, Deserialize)]
/// Name of enemy.
pub struct EnemyName {
    #[serde(rename = "ID")]
    /// Doge = 2.
    _id: u32,
    #[serde(rename = "Name")]
    /// Enemy name.
    pub name: String,
    #[serde(rename = "Plural")]
    /// Plural name.
    pub plural: String,
}
#[derive(Debug, Deserialize)]
/// Enemy data.
pub struct EnemyData {
    #[serde(rename = "Image")]
    _image: u32,
    #[serde(rename = "Name")]
    /// Common name.
    pub name: String,
    #[serde(rename = "Link")]
    /// Page (blank if same as name).
    pub link: String,
    #[serde(rename = "HP")]
    /// Enemy HP.
    pub hp: u32,
    #[serde(rename = "Attack")]
    /// Enemy AP.
    pub attack: u32,
}
/// Contains the names and enemy data.
pub struct EnemyDataContainer {
    /// Doge = 2.
    names: LazyLock<Vec<EnemyName>>,
    /// Doge = 0.
    data: LazyLock<HashMap<u32, EnemyData>>,
}
impl EnemyDataContainer {
    /// Get the name of an enemy based on their wiki id (Doge = 0).
    pub fn get_name(&self, id: u32) -> &EnemyName {
        &self.names[id as usize + 2]
    }
    /// Get the name of an enemy as used in Lua modules.
    pub fn get_common_name(&self, id: u32) -> &str {
        &self.get_data(id).name
    }
    /// Get the data of the enemy.
    pub fn get_data(&self, id: u32) -> &EnemyData {
        &self.data.get(&id).unwrap()
    }
}
/// Contains enemy data.
pub static ENEMY_DATA: EnemyDataContainer = EnemyDataContainer {
    names: LazyLock::new(get_enemy_names),
    data: LazyLock::new(get_enemy_data),
};

fn get_enemy_names() -> Vec<EnemyName> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .comment(Some(b'#'))
        .from_path(get_file_location(FileLocation::WikiData).join("EnemyLinkData.csv"));

    rdr.unwrap()
        .deserialize::<EnemyName>()
        .into_iter()
        .map(|r| r.unwrap().into())
        .collect()
}
fn get_enemy_data() -> HashMap<u32, EnemyData> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(get_file_location(FileLocation::WikiData).join("EnemyNames.csv"));

    rdr.unwrap()
        .deserialize::<EnemyData>()
        .into_iter()
        .map(|r| {
            let enemy: EnemyData = r.unwrap();
            (enemy._image, enemy)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wikitext::wiki_utils::extract_name;
    use std::collections::HashSet;

    #[test]
    fn test_id_equals_index() {
        for (i, enemy) in ENEMY_DATA.names.iter().enumerate() {
            assert_eq!(enemy._id as usize, i);
        }
    }

    #[test]
    fn test_no_duplicate_data_keys() {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(get_file_location(FileLocation::WikiData).join("EnemyNames.csv"))
            .unwrap();

        let it = rdr.deserialize::<EnemyData>().into_iter();

        let mut seen = HashSet::new();
        it.map(|e| assert!(seen.insert(e.unwrap()._image)))
            .for_each(drop);
    }

    #[test]
    fn test_num_equals_enemy() {
        const EMPTY_IDS: [u32; 4] = [0, 1, 21, 22];
        const WRONG_NAME_IDS: [u32; 2] = [276, 277];
        // Charlotte (Snake), Charlotte (Doll)
        for enemy in ENEMY_DATA.names.iter() {
            if EMPTY_IDS.contains(&enemy._id) || WRONG_NAME_IDS.contains(&enemy._id) {
                continue;
            }
            assert_eq!(
                ENEMY_DATA.get_data(enemy._id - 2).name,
                extract_name(&enemy.name)
            );
        }
    }
}
