//! Module that deals with `unitbuy.csv` data.

use crate::game_data::version::version_data::CacheableVersionData;
use csv::ByteRecord;
use std::{fmt::Debug, path::Path};

#[derive(Debug, serde::Deserialize, Default)]
#[allow(missing_docs)]
pub struct UnitBuyRaw {
    /// Amount of stages to clear in chapter before unit is available (e.g. 0
    /// for most cats, 48 for Bahamut).
    pub stage_available: u8,
    /// Amount of [`Self::unlock_currency`] that unit requires to be unlocked.
    pub unlock_cost: u16,
    /// Cost to upgrade to level 1.
    ///
    /// You can't actually upgrade to level 1, but this number is used to
    /// calculate the cost of upgrading to level 11, 21 etc.
    pub upgrade_to_1: u32,
    /// Cost to upgrade to level 2.
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
    /// 0 = xp, 1 = catfood, 2 = capsule/free/"other".
    pub unlock_currency: u8,
    /// 0 = normal, 1 = special, etc.
    pub rarity: u8,
    /// Order in the Cat Guide.
    pub cro_order: i32,
    /// 2 for bahamut, 1 for actress, mr, panties, skirt, valk.
    pub chap_available: u8,
    pub sell_xp: u32,
    _uk17: u8,
    /// Level cap after chapter 2.
    pub max_xp_level_ch2: u8,
    /// Initial max plus level. Is extended through user rank rewards.
    pub initial_max_plus: u8,

    // 20
    /// What level the unit evolves into their true form, 100 for Superfeline
    /// and 30 for other normal cats; -1 for all others.
    pub evol_level: i8,
    /// 2 for iron wall, 10 for everyone else. Probably level they evolve into
    /// evolved form.
    _uk21: u8,
    /// Level cap before chapter 2.
    pub max_xp_level_ch1: u8,
    /// ID of tf evolution.
    pub true_num: u32,
    pub ultra_num: u32,
    /// Level that Catfruit evolution becomes available. -1 if CF evolution is
    /// impossible.
    // need to check what happens when evol_level exists with this. evol_level
    // takes priority in code currently
    pub true_cf_evol_level: i8,
    pub ultra_cf_evol_level: i8,
    /// XP to evolve to true form.
    pub true_evol_xp: u32,
    /// ID of first item required to upgrade to tf.
    pub true_cf_item1: u8,
    /// Amount of first item required to upgrade to tf.
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
    /// -1 for normal cats, 30 for every cat that can go to 30. 31 for iron
    /// wall, 21 for Metal, 2 for units with max level 1. Perhaps first catseye
    /// level?
    _uk49: i8,

    // 50
    pub max_nat_level: u8,
    pub max_plus_level: u8,
    _uk52: u8,
    _uk53: u16,
    _uk54: u16,
    _uk55: u8,
    /// 0 for normals and metal cat, 2 for everyone else.
    _uk56: u8,
    /// E.g. `90500` for 09.05.00 = 9.5.0.
    pub update_released: i64,
    pub sell_np: u8,
    _uk59: u32,

    // 60
    /// 1 for superfeline, 0 for everyone else.
    _uk60: u8,
    pub ancient_egg_id_norm: i8,
    pub ancient_egg_id_evo: i8,

    #[serde(default)]
    /// Placeholder to avoid errors when new updates come around.
    pub rest: Vec<i32>,
}

fn parse_unitbuy_error(e: &csv::Error, result: &ByteRecord) -> impl Debug {
    let index = match e.kind() {
        csv::ErrorKind::Deserialize { pos: _, err } => err.field().unwrap(),
        _ => unimplemented!(),
    };

    String::from_utf8(result[index as usize].into()).unwrap()
}

fn get_unitbuy(path: &Path) -> Vec<UnitBuyRaw> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path.join("DataLocal/unitbuy.csv"))
        .unwrap();

    rdr.byte_records()
        .map(|record| {
            let result = record.unwrap();
            let unit: UnitBuyRaw = result.deserialize(None).unwrap_or_else(|e| {
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
/// Container for [`UnitBuyRaw`] data.
pub struct UnitBuyContainer {
    units: Vec<UnitBuyRaw>,
}
impl UnitBuyContainer {
    /// Get [`UnitBuyRaw`] line for a unit.
    pub fn get_unit(&self, id: u32) -> Option<&UnitBuyRaw> {
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
