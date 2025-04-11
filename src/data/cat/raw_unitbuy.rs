use std::path::Path;

#[derive(Debug, serde::Deserialize)]
#[allow(missing_docs)]
pub struct UnitBuy {
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
