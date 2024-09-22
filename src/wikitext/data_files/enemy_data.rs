//! Module that gets information about enemy names and data.

use std::sync::LazyLock;

use serde::Deserialize;

use crate::{file_handler::{get_file_location, FileLocation}, wikitext::wiki_utils::extract_name};

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
/// Contains the names and enemy data.
pub struct EnemyDataContainer {
    /// Doge = 2.
    pub names: LazyLock<Vec<EnemyName>>,
    /// Doge = 0.
    _data: (),
}
impl EnemyDataContainer {
    /// Get the name of an enemy based on their wiki id (Doge = 0).
    pub fn get_name(&self, id: u32) -> &EnemyName {
        &self.names[id as usize + 2]
    }
    /// Get the name of an enemy as used in Lua modules.
    pub fn get_common_name(&self, id: u32) -> &str {
        // TODO use name in _data
        // TODO fix Charlotte (Doll)
        extract_name(&self.get_name(id).name)
    }
}
/// Contains enemy data.
pub static ENEMY_DATA: EnemyDataContainer = EnemyDataContainer {
    names: LazyLock::new(get_enemy_names),
    _data: (),
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
