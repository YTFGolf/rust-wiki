//! Module that deals with `unitbuy.csv` data.

use crate::game_data::version::version_data::CacheableVersionData;
use csv::{ByteRecord, Error};
use std::{fmt::Debug, path::Path};

#[derive(Debug, serde::Deserialize, Default)]
/// Raw data for a combo.
pub struct ComboDataRaw {
    _id_or_something_or_maybe_order: u16,
    /// -1 = unavailable, see cat combo filter for other things.
    pub unlock_type: i16,
    /// ID of unit #1, -1 if not there.
    unit_id1: i16,
    /// Form of unit #1, -1 if not there.
    unit_form1: i8,
    /// ID of unit #2, -1 if not there.
    unit_id2: i16,
    /// Form of unit #2, -1 if not there.
    unit_form2: i8,
    /// ID of unit #3, -1 if not there.
    unit_id3: i16,
    /// Form of unit #3, -1 if not there.
    unit_form3: i8,
    /// ID of unit #4, -1 if not there.
    unit_id4: i16,
    /// Form of unit #4, -1 if not there.
    unit_form4: i8,

    // 10
    /// ID of unit #5, -1 if not there.
    unit_id5: i16,
    /// Form of unit #5, -1 if not there.
    unit_form5: i8,
    /// Effect; effect is named in Nyancombo1_en.
    combo_effect_num: u8,
    /// Intensity; intensity is named in Nyancombo2_en.
    combo_intensity_num: u8,
    /// Always -1.
    _uk14: i8,
    #[serde(default)]
    /// Placeholder to avoid errors when new updates come around.
    pub rest: Vec<i32>,
}

fn parse_nyancombodata_error(e: &Error, result: &ByteRecord) -> impl Debug {
    let index = match e.kind() {
        csv::ErrorKind::Deserialize { pos: _, err } => err.field().unwrap(),
        _ => unimplemented!(),
    };

    String::from_utf8(result[index as usize].into()).unwrap()
}

fn get_combodata(path: &Path) -> Vec<ComboDataRaw> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path.join("DataLocal/NyancomboData.csv"))
        .unwrap();

    rdr.byte_records()
        .map(|record| {
            let result = record.unwrap();
            let unit: ComboDataRaw = result.deserialize(None).unwrap_or_else(|e| {
                panic!(
                    "Error when parsing record {result:?}: {e}. Item was {item:?}.",
                    item = parse_nyancombodata_error(&e, &result)
                )
            });
            unit
        })
        .collect()
}

#[derive(Debug)]
/// Container for [`ComboDataRaw`] data.
pub struct CombosDataContainer {
    combos: Vec<ComboDataRaw>,
}
impl CacheableVersionData for CombosDataContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            combos: get_combodata(path),
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
        let combos = version.get_cached_file::<CombosDataContainer>();
        for combo in &combos.combos {
            println!("{combo:?}");
        }
        panic!()
    }
}
