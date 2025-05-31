//! Module that gets information about enemy names and data.

use crate::wiki_data::file_handler::get_wiki_data_location;
use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

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
    /// Can use this as id.
    image_id: u32,
    #[serde(rename = "Name")]
    /// Common name.
    pub name: String,
    #[serde(rename = "Link")]
    /// Page (blank if same as name).
    pub link: Option<String>,
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
    /// Doge = 0.
    reverse_id_map: LazyLock<HashMap<String, u32>>,
}
impl EnemyDataContainer {
    /// Get the singular and plural names of an enemy based on their wiki id
    /// (Doge = 0).
    pub fn get_names(&self, id: u32) -> &EnemyName {
        &self.names[id as usize + 2]
    }
    /// Get the name of an enemy as used in Lua modules.
    pub fn get_common_name(&self, id: u32) -> &str {
        &self.get_data(id).name
    }
    /// Get the data of the enemy.
    pub fn get_data(&self, id: u32) -> &EnemyData {
        self.data.get(&id).unwrap()
    }
    /// Get unit's id from name. Case-insensitive. Uses common name.
    pub fn get_id_from_name(&self, name: &str) -> Option<&u32> {
        self.reverse_id_map.get(&name.to_lowercase())
    }
}
/// Contains enemy data.
pub static ENEMY_DATA: EnemyDataContainer = EnemyDataContainer {
    names: LazyLock::new(get_enemy_names),
    data: LazyLock::new(get_enemy_data),
    reverse_id_map: LazyLock::new(get_reverse_map),
};

fn get_enemy_names() -> Vec<EnemyName> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .comment(Some(b'#'))
        .from_path(get_wiki_data_location().join("EnemyLinkData.csv"));

    rdr.unwrap()
        .deserialize::<EnemyName>()
        .map(|r| r.unwrap())
        .collect()
}
fn get_enemy_data() -> HashMap<u32, EnemyData> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(get_wiki_data_location().join("EnemyNames.csv"));

    rdr.unwrap()
        .deserialize::<EnemyData>()
        .map(|r| {
            let enemy: EnemyData = r.unwrap();
            (enemy.image_id, enemy)
        })
        .collect()
}
fn get_reverse_map() -> HashMap<String, u32> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(get_wiki_data_location().join("EnemyNames.csv"));

    rdr.unwrap()
        .deserialize::<EnemyData>()
        .map(|r| {
            let enemy: EnemyData = r.unwrap();
            (enemy.name.to_lowercase(), enemy.image_id)
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::used_underscore_binding)]
// Need to check id in these tests but don't want it to give clippy a stroke.
mod tests {
    use super::*;
    use crate::{
        game_data::stage::parsed::stage_enemy::MS_SIGN, wikitext::text_utils::extract_name,
    };
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
            .from_path(get_wiki_data_location().join("EnemyNames.csv"))
            .unwrap();

        let it = rdr.deserialize::<EnemyData>();

        let mut seen = HashSet::new();
        it.map(|e| assert!(seen.insert(e.unwrap().image_id)))
            .for_each(drop);
    }

    #[test]
    fn test_num_equals_enemy() {
        const EMPTY_IDS: [u32; 4] = [0, 1, 21, 22];
        const WRONG_NAME_IDS: [u32; 0] = [];
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

    #[test]
    fn test_blank_link_is_none() {
        let hermit = ENEMY_DATA.get_data(354);
        assert_eq!(hermit.link, Some("Hermit Cat (Enemy)".into()));

        let doge = ENEMY_DATA.get_data(0);
        assert_eq!(doge.link, None);
    }

    #[test]
    fn test_reverse_map() {
        let id = 0;
        let name = "Doge";
        assert_eq!(*ENEMY_DATA.get_id_from_name(name).unwrap(), id);

        let id = MS_SIGN;
        let name = "Ms. Sign";
        assert_eq!(*ENEMY_DATA.get_id_from_name(name).unwrap(), id);

        let id = 644;
        let name = "E-644";
        assert_eq!(*ENEMY_DATA.get_id_from_name(name).unwrap(), id);
    }

    #[test]
    fn test_case_reverse_map() {
        assert_eq!(
            ENEMY_DATA.get_id_from_name("ms. sign"),
            ENEMY_DATA.get_id_from_name("MS. SIGN")
        );
    }
}
