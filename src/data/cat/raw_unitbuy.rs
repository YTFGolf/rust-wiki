#![allow(dead_code)]

use std::path::Path;

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

fn get_unitbuy(path: &Path) -> Vec<UnitBuy> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path.join("DataLocal/unitbuy.csv"))
        .unwrap();

    rdr.byte_records()
        .map(|record| {
            let result = record.unwrap();
            let unit: UnitBuy = result
                .deserialize(None)
                .unwrap_or_else(|e| panic!("Error when parsing record {result:?}: {e}"));
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

        let ids = [0, 8, 25, 177, 543, 626, 643, 658];
        for id in ids {
            println!("{:?}", units[id])
        }

        #[allow(unused)]
        for (i, unit) in units.iter().enumerate() {
            // if unit._uk12 != 0 { println!("{i}: {:?}", unit) }
            // if unit.max_xp_upgrade != 20 { println!("{i}: {:?}", unit.max_xp_upgrade) }
        }
    }
}
