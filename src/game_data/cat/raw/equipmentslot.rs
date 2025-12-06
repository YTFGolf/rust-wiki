//! Talent orb slots used by cats.

use crate::game_data::version::{
    Version,
    version_data::{CacheableVersionData, CvdCreateError, CvdCreateHandler, CvdResult},
};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
/// Descriptor of equipment slots.
pub struct EquipmentSlotItem {
    unit_id: usize,
    /// Amount of talent orb slots.
    pub amt_slots: u8,
    #[serde(default)]
    condition_1: u8,
    #[serde(default)]
    condition_2: u8,
}

fn get_equipmentslot(path: &Path) -> Result<Vec<EquipmentSlotItem>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path.join("DataLocal/equipmentslot.csv"))?;

    let mut records = rdr.byte_records();
    records.next();

    let records_iter = records.map(|record| {
        let result = record?;
        result.deserialize::<EquipmentSlotItem>(None)
    });

    records_iter.collect()
}

#[derive(Debug, Default)]
/// Container for equipment slots.
pub struct EquipmentSlotContainer {
    slots: Vec<EquipmentSlotItem>,
}

impl CacheableVersionData for EquipmentSlotContainer {
    fn create(version: &Version) -> CvdResult<Self> {
        let slots = get_equipmentslot(version.location()).map_err(|e| CvdCreateError {
            handler: CvdCreateHandler::Default(Default::default()),
            err: Box::new(e),
        })?;
        Ok(Self { slots })
    }
}

impl EquipmentSlotContainer {
    /// Get equipment slots for the unit with id.
    pub fn get_slot_item(&self, cat_id: usize) -> Option<&EquipmentSlotItem> {
        self.slots.iter().find(|slot| slot.unit_id == cat_id)
    }
}
