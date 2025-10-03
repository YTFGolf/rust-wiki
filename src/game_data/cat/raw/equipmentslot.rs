//! Talent orb slots used by cats.

use crate::game_data::version::version_data::CacheableVersionData;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
/// Descriptor of equipment slots.
pub struct EquipmentSlotItem {
    unit_id: usize,
    amt_slots: u8,
    #[serde(default)]
    condition_1: u8,
    #[serde(default)]
    condition_2: u8,
}

fn get_equipmentslot(path: &Path) -> Vec<EquipmentSlotItem> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path.join("DataLocal/equipmentslot.csv"))
        .unwrap();

    let mut records = rdr.byte_records();
    records.next();

    let records_iter = records.map(|record| {
        let result = record.unwrap();
        result.deserialize(None).unwrap()
    });

    records_iter.collect()
}

#[derive(Debug)]
/// Container for equipment slots.
pub struct EquipmentSlotContainer {
    slots: Vec<EquipmentSlotItem>,
}

impl CacheableVersionData for EquipmentSlotContainer {
    fn init_data(path: &Path) -> Self
    where
        Self: Sized,
    {
        Self {
            slots: get_equipmentslot(path),
        }
    }
}

impl EquipmentSlotContainer {
    /// Get equipment slots for the unit with id.
    pub fn get_slot_item(&self, cat_id: usize) -> Option<&EquipmentSlotItem> {
        self.slots.iter().find(|slot| slot.unit_id == cat_id)
    }
}
