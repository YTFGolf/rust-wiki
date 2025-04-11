use crate::data::version::version_data::CacheableVersionData;
use csv::{ByteRecord, Error};
use std::{fmt::Debug, path::Path};

#[derive(Debug, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub struct UnitExp {
    until_10: u8,
    until_20: u8,
    until_30: u8,
    until_40: u8,
    until_50: u8,
    until_60: u8,
    until_70: u8,
    until_80: u8,
    until_90: u8,
    until_100: u8,
    until_110: u8,
    until_120: u8,
    until_130: u8,
    until_140: u8,
    until_150: u8,
    until_160: u8,
    until_170: u8,
    until_180: u8,
    until_190: u8,
    until_200: u8,
}

fn parse_unitexp_error(e: &Error, result: &ByteRecord) -> impl Debug {
    let index = match e.kind() {
        csv::ErrorKind::Deserialize { pos: _, err } => err.field().unwrap(),
        _ => unimplemented!(),
    };

    String::from_utf8(result[index as usize].into()).unwrap()
}

fn get_unitexp(path: &Path) -> Vec<UnitExp> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_path(path.join("DataLocal/unitexp.csv"))
        .unwrap();

    rdr.byte_records()
        .map(|record| {
            let result = record.unwrap();
            assert_eq!(result.len(), 20);
            let unit: UnitExp = result.deserialize(None).unwrap_or_else(|e| {
                panic!(
                    "Error when parsing record {result:?}: {e}. Item was {item:?}.",
                    item = parse_unitexp_error(&e, &result)
                )
            });
            unit
        })
        .collect()
}

#[derive(Debug)]
struct UnitExpContainer {
    units: Vec<UnitExp>,
}
impl UnitExpContainer {
    pub fn get_unit(&self, id: u32) -> &UnitExp {
        &self.units[id as usize]
    }
}
impl CacheableVersionData for UnitExpContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            units: get_unitexp(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TEST_CONFIG;

    const DEFAULT: UnitExp = UnitExp {
        until_10: 10,
        until_20: 20,
        until_30: 30,
        until_40: 35,
        until_50: 40,
        until_60: 45,
        until_70: 50,
        until_80: 55,
        until_90: 60,
        until_100: 65,
        until_110: 70,
        until_120: 75,
        until_130: 80,
        until_140: 85,
        until_150: 90,
        until_160: 95,
        until_170: 100,
        until_180: 105,
        until_190: 110,
        until_200: 115,
    };
    const SUPERFELINE: UnitExp = UnitExp {
        until_10: 10,
        until_20: 10,
        until_30: 10,
        until_40: 10,
        until_50: 10,
        until_60: 10,
        until_70: 10,
        until_80: 10,
        until_90: 10,
        until_100: 10,
        until_110: 10,
        until_120: 10,
        until_130: 10,
        until_140: 10,
        until_150: 10,
        until_160: 10,
        until_170: 10,
        until_180: 10,
        until_190: 10,
        until_200: 10,
    };

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();
        let unitexp = version.get_cached_file::<UnitExpContainer>();
        let units = &unitexp.units;

        for (i, unit) in units.iter().enumerate() {
            match i {
                643 => assert_eq!(unit, &SUPERFELINE),
                _ => assert_eq!(unit, &DEFAULT),
            }
        }
    }

    #[test]
    fn test_units() {
        // print them out just for show
        let version = TEST_CONFIG.version.current_version();
        let unitexp = version.get_cached_file::<UnitExpContainer>();

        let test_units = [("cat", 0), ("sfeline", 643)];
        for (name, id) in test_units {
            println!("{name} ({id}) = {:?}", unitexp.get_unit(id));
        }
    }
}
