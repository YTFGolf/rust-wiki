//! Module that deals with the `DropItem.csv` file.

use crate::game_data::{
    meta::stage::map_id::MapID,
    version::{
        Version,
        version_data::{CacheableVersionData, CvdCreateError, CvdResult},
    },
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error, path::Path};

type StageSize = u8;
type ChanceSize = u8;

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
/// Items dropped in the map.
pub struct DropItemRaw {
    pub mapid: u32,
    pub star1: f64,
    pub star2: f64,
    pub star3: f64,
    pub star4: f64,
    pub stage1: StageSize,
    pub stage2: StageSize,
    pub stage3: StageSize,
    pub stage4: StageSize,
    pub stage5: StageSize,

    // 10
    pub stage6: StageSize,
    pub stage7: StageSize,
    pub stage8: StageSize,
    pub nothing: ChanceSize,
    pub bricks: ChanceSize,
    pub feathers: ChanceSize,
    pub coal: ChanceSize,
    pub sprockets: ChanceSize,
    pub gold: ChanceSize,
    pub meteorite: ChanceSize,

    // 20
    pub beast_bones: ChanceSize,
    pub ammonite: ChanceSize,
    #[serde(default)]
    pub brick_z: Option<ChanceSize>,
    #[serde(default)]
    pub feathers_z: Option<ChanceSize>,
    #[serde(default)]
    pub coal_z: Option<ChanceSize>,
    #[serde(default)]
    pub sprockets_z: Option<ChanceSize>,
    #[serde(default)]
    pub gold_z: Option<ChanceSize>,
    #[serde(default)]
    pub meteorite_z: Option<ChanceSize>,
    #[serde(default)]
    pub beast_bones_z: Option<ChanceSize>,
    #[serde(default)]
    pub ammonite_z: Option<ChanceSize>,
    // length = 30
}

type DropItemMap = HashMap<u32, DropItemRaw>;
fn get_drop_item(path: &Path) -> Result<DropItemMap, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(path.join("DataLocal/DropItem.csv"))
        .map_err(Box::new)?;

    rdr.byte_records()
        .map(|record| {
            let result = record.map_err(Box::new)?;
            let drop: DropItemRaw = result.deserialize(None).map_err(Box::new)?;
            Ok((drop.mapid, drop))
        })
        .collect()
}

#[derive(Debug)]
/// Container for [`DropItem`] data.
pub struct DropItem {
    map: DropItemMap,
}
impl DropItem {
    /// Get `drop_item` for specific map if it exists.
    pub fn get_drop_item(&self, map_id: &MapID) -> Option<&DropItemRaw> {
        self.map.get(&map_id.mapid())
    }
}
impl CacheableVersionData for DropItem {
    fn create(version: &Version) -> CvdResult<Self> {
        Ok(Self {
            map: get_drop_item(version.location()).map_err(CvdCreateError::throw)?,
        })
    }
}
