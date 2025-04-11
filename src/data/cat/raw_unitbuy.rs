#![allow(dead_code)]

use csv::{ByteRecord, Error};
use std::{fmt::Debug, path::Path};

#[derive(Debug, serde::Deserialize)]
#[allow(missing_docs)]
pub struct UnitBuy {
    stage_available_after: u8,
    unlock_cost: u16,
    upgrade_to_1: u32,
    upgrade_to_2: u32,
    upgrade_to_3: u32,
    upgrade_to_4: u32,
    upgrade_to_5: u32,
    upgrade_to_6: u32,
    upgrade_to_7: u32,
    upgrade_to_8: u32,

    // 10
    upgrade_to_9: u32,
    upgrade_to_10: u32,
    unlock_method: u8,
    // appears to be 0 = xp, 1 = catfood, 2 = capsule/free
    rarity: u8,
    cro_order: i16,
    _uk15: u8,
    // 2 for bahamut, 1 for actress, mr, panties, skirt, valk
    sell_xp: u32,
    _uk17: u8,
    max_xp_level_ch2: u8,
    initial_max_plus: u8,

    // 20
    evol_level: i8,
    // only exists for normal cats, 100 for sf and 30 for others. -1 for all
    // other cats.
    _uk21: u8,
    // 2 for iron wall, 10 for everyone else
    max_xp_level_ch1: u8,
    true_num: u32,
    ultra_num: u32,
    true_cf_evol_level: i8,
    ultra_cf_evol_level: i8,
    true_evol_xp: u32,
    true_cf_item1: u8,
    true_cf_cost1: u8,

    // 30
    true_cf_item2: u8,
    true_cf_cost2: u8,
    true_cf_item3: u8,
    true_cf_cost3: u8,
    true_cf_item4: u8,
    true_cf_cost4: u8,
    true_cf_item5: u8,
    true_cf_cost5: u8,
    ultra_evol_xp: u32,
    ultra_cf_item1: u8,

    // 40
    ultra_cf_cost1: u8,
    ultra_cf_item2: u8,
    ultra_cf_cost2: u8,
    ultra_cf_item3: u8,
    ultra_cf_cost3: u8,
    ultra_cf_item4: u8,
    ultra_cf_cost4: u8,
    ultra_cf_item5: u8,
    ultra_cf_cost5: u8,
    _uk49: i8,
    // -1 for normal cats, 30 for every cat that can go to 30. 31 for iron wall,
    // 21 for Metal, 2 for units with max level 1. Perhaps first catseye level?

    // 50
    max_nat_level: u8,
    max_plus_level: u8,
    _uk52: u8,
    _uk53: u16,
    _uk54: u16,
    _uk55: u8,
    _uk56: u8,
    // is 0 for normals and metal cat, 2 for everyone else
    update_released: String,
    // e.g. "90500" for 09.05.00 = 9.5.0
    sell_np: u8,
    _uk59: u32,

    // 60
    _uk60: u8,
    // is 1 if cat is superfeline
    ancient_egg_id_norm: i8,
    ancient_egg_id_evo: i8,

    #[serde(default)]
    rest: Vec<i32>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TEST_CONFIG;

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();
        let path = version.get_file_path("");
        let units = get_unitbuy(&path);

        let test_units = [
            ("cat", 0),
            ("titan", 8),
            ("bahamut", 25),
            ("cancan", 32),
            ("dio", 177),
            ("metal", 200),
            ("dasli", 543),
            ("cat modoki", 626),
            ("sfeline", 643),
            ("courier", 658),
        ];
        for (name, id) in test_units {
            println!("{name} ({id}) = {:?}\n", units[id]);
        }

        #[allow(unused)]
        for (i, unit) in units.iter().enumerate() {
            // if unit._uk12 != 0 { println!("{i}: {:?}", unit) }
            // if unit.max_xp_upgrade != 20 { println!("{i}: {:?}", unit.max_xp_upgrade) }
            // if ![0, 9].contains(&unit.initial_max_plus) { println!("{i}: {:?}", unit.initial_max_plus) }
            // if unit._uk22 != 10 { println!("{i}: {:?}", unit._uk22) }
            // if unit.max_xp_level_ch2 != 20 || unit.max_xp_level_ch1 != 10  { println!("{i}: {:?}, {:?}", unit.max_xp_level_ch2, unit.max_xp_level_ch1) }
            // if unit._uk49 != 30 { println!("{i}: {:?}", unit._uk49) }
            // if unit._uk52 != 0 { println!("{i}: {:?}", unit._uk52) }
            // if unit._uk53 != 0 { println!("{i}: {:?}", unit._uk53) }
            // if unit._uk54 != 0 { println!("{i}: {:?}", unit._uk54) }
            // if unit._uk55 != 0 { println!("{i}: {:?} {:?}", unit._uk55, unit.ultra_evol_xp) }
            // if unit._uk56 != 2 { println!("{i}: {:?}", unit._uk56) }
            // if unit._uk59 != 0 { println!("{i}: {:?}", unit._uk59) }
            // if unit._uk60 != 0 { println!("{i}: {:?}", unit._uk60) }
            // if unit.ancient_egg_id_norm != -1 { println!("{i}: {:?}", unit.ancient_egg_id_norm); assert_ne!(unit.ancient_egg_id_evo, 0) }
            // if unit.ancient_egg_id_evo != -1 { println!("{i}: {:?}", unit.ancient_egg_id_evo); assert_eq!(unit.ancient_egg_id_norm, 0) }
            // if unit._uk61 != -1 { println!("{i}: {:?}", unit._uk61) }

            if unit.ancient_egg_id_norm != -1 || unit.ancient_egg_id_evo != -1 {
                assert_eq!(unit.ancient_egg_id_norm, 0);
                assert_ne!(unit.ancient_egg_id_evo, 0);
            }
            assert!(unit.rest.is_empty());
        }
    }
}
