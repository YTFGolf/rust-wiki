#![allow(dead_code)]

use std::{fmt::Debug, path::Path};
use csv::{ByteRecord, Error};

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
    max_xp_upgrade: u8,
    _uk19: u8,

    // 20
    _uk20: i8,
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
        }
    }
}
