//! Module that deals with `unitbuy.csv` data.

use crate::game_data::version::version_data::CacheableVersionData;
use csv::{ByteRecord, Error};
use std::{fmt::Debug, path::Path};

#[derive(Debug, serde::Deserialize)]
#[allow(missing_docs)]
pub struct UnitBuy {
    pub stage_available: u8,
    pub unlock_cost: u16,
    pub upgrade_to_1: u32,
    pub upgrade_to_2: u32,
    pub upgrade_to_3: u32,
    pub upgrade_to_4: u32,
    pub upgrade_to_5: u32,
    pub upgrade_to_6: u32,
    pub upgrade_to_7: u32,
    pub upgrade_to_8: u32,

    // 10
    pub upgrade_to_9: u32,
    pub upgrade_to_10: u32,
    pub unlock_currency: u8,
    // appears to be 0 = xp, 1 = catfood, 2 = capsule/free
    pub rarity: u8,
    pub cro_order: i32,
    pub chap_available: u8,
    // 2 for bahamut, 1 for actress, mr, panties, skirt, valk
    pub sell_xp: u32,
    _uk17: u8,
    pub max_xp_level_ch2: u8,
    pub initial_max_plus: u8,

    // 20
    pub evol_level: i8,
    // only exists for normal cats, 100 for sf and 30 for others. -1 for all
    // other cats.
    _uk21: u8,
    // 2 for iron wall, 10 for everyone else
    pub max_xp_level_ch1: u8,
    pub true_num: u32,
    pub ultra_num: u32,
    pub true_cf_evol_level: i8,
    pub ultra_cf_evol_level: i8,
    pub true_evol_xp: u32,
    pub true_cf_item1: u8,
    pub true_cf_cost1: u8,

    // 30
    pub true_cf_item2: u8,
    pub true_cf_cost2: u8,
    pub true_cf_item3: u8,
    pub true_cf_cost3: u8,
    pub true_cf_item4: u8,
    pub true_cf_cost4: u8,
    pub true_cf_item5: u8,
    pub true_cf_cost5: u8,
    pub ultra_evol_xp: u32,
    pub ultra_cf_item1: u8,

    // 40
    pub ultra_cf_cost1: u8,
    pub ultra_cf_item2: u8,
    pub ultra_cf_cost2: u8,
    pub ultra_cf_item3: u8,
    pub ultra_cf_cost3: u8,
    pub ultra_cf_item4: u8,
    pub ultra_cf_cost4: u8,
    pub ultra_cf_item5: u8,
    pub ultra_cf_cost5: u8,
    _uk49: i8,
    // -1 for normal cats, 30 for every cat that can go to 30. 31 for iron wall,
    // 21 for Metal, 2 for units with max level 1. Perhaps first catseye level?

    // 50
    pub max_nat_level: u8,
    pub max_plus_level: u8,
    _uk52: u8,
    _uk53: u16,
    _uk54: u16,
    _uk55: u8,
    _uk56: u8,
    // is 0 for normals and metal cat, 2 for everyone else
    pub update_released: i64,
    // e.g. `90500` for 09.05.00 = 9.5.0
    pub sell_np: u8,
    _uk59: u32,

    // 60
    _uk60: u8,
    // is 1 if cat is superfeline
    pub ancient_egg_id_norm: i8,
    pub ancient_egg_id_evo: i8,

    #[serde(default)]
    pub rest: Vec<i32>,
}

fn parse_unitbuy_error(e: &Error, result: &ByteRecord) -> impl Debug {
    let index = match e.kind() {
        csv::ErrorKind::Deserialize { pos: _, err } => err.field().unwrap(),
        _ => unimplemented!(),
    };

    String::from_utf8(result[index as usize].into()).unwrap()
}

fn get_unitbuy(path: &Path) -> Vec<UnitBuy> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path.join("DataLocal/unitbuy.csv"))
        .unwrap();

    rdr.byte_records()
        .map(|record| {
            let result = record.unwrap();
            let unit: UnitBuy = result.deserialize(None).unwrap_or_else(|e| {
                panic!(
                    "Error when parsing record {result:?}: {e}. Item was {item:?}.",
                    item = parse_unitbuy_error(&e, &result)
                )
            });
            unit
        })
        .collect()
}

#[derive(Debug)]
/// Container for [`UnitBuy`] data.
pub struct UnitBuyContainer {
    units: Vec<UnitBuy>,
}
impl UnitBuyContainer {
    /// Get [`UnitBuy`] line for a unit.
    pub fn get_unit(&self, id: u32) -> Option<&UnitBuy> {
        self.units.get(id as usize)
    }
}
impl CacheableVersionData for UnitBuyContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            units: get_unitbuy(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();
        let unitbuy = version.get_cached_file::<UnitBuyContainer>();
        let units = &unitbuy.units;

        for unit in units {
            if unit.ancient_egg_id_norm != -1 || unit.ancient_egg_id_evo != -1 {
                assert_eq!(unit.ancient_egg_id_norm, 0);
                assert!(unit.ancient_egg_id_evo > 0);
            }
            assert!(unit.rest.is_empty());
        }
    }
}
