//! Module that deals with `unitbuy.csv` data.

#![allow(missing_docs)]

use crate::game_data::version::version_data::CacheableVersionData;
use csv::ByteRecord;
use std::{fmt::Debug, path::Path};
use strum::FromRepr;

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

fn parse_nyancombodata_error(e: &csv::Error, result: &ByteRecord) -> impl Debug {
    let index = match e.kind() {
        csv::ErrorKind::Deserialize { pos: _, err } => err.field().unwrap(),
        _ => unimplemented!(),
    };

    String::from_utf8(result[index as usize].into()).unwrap()
}

pub fn get_combodata(path: &Path) -> Vec<ComboDataRaw> {
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

#[repr(i16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromRepr)]
pub enum ComboUnlockType {
    Unavailable = -1,
    Beginning = 1,
    ItF1 = 4,
    ItF2 = 5,
    ItF3 = 6,
    Rank1450 = 10001,
    Rank2150 = 10002,
    Rank2700 = 10003,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ComboUnit {
    id: i16,
    /// 0 = normal form.
    form: i8,
}
impl ComboUnit {
    const fn new(id: i16, form: i8) -> Self {
        Self { id, form }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ComboData {
    unlock_type: ComboUnlockType,
    units: Vec<ComboUnit>,
    /// Effect; effect is named in Nyancombo1_en.
    combo_effect_num: u8,
    /// Intensity; intensity is named in Nyancombo2_en.
    combo_intensity_num: u8,
}
impl From<ComboDataRaw> for ComboData {
    fn from(value: ComboDataRaw) -> Self {
        let unlock_type = ComboUnlockType::from_repr(value.unlock_type).unwrap();
        let combo_effect_num = value.combo_effect_num;
        let combo_intensity_num = value.combo_intensity_num;

        let mut units = Vec::new();

        let mut add_unit = |id, form| {
            if id >= 0 && form >= 0 {
                units.push(ComboUnit::new(id, form))
            }
        };

        add_unit(value.unit_id1, value.unit_form1);
        add_unit(value.unit_id2, value.unit_form2);
        add_unit(value.unit_id3, value.unit_form3);
        add_unit(value.unit_id4, value.unit_form4);
        add_unit(value.unit_id5, value.unit_form5);

        Self {
            unlock_type,
            units,
            combo_effect_num,
            combo_intensity_num,
        }
    }
}

#[derive(Debug)]
/// Container for [`ComboDataRaw`] data.
pub struct CombosDataContainer {
    #[allow(dead_code)]
    combos: Vec<ComboData>,
}
impl CacheableVersionData for CombosDataContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            combos: get_combodata(path).into_iter().map(Into::into).collect(),
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
        let combos = get_combodata(version.location());
        for combo in combos {
            assert_eq!(combo.rest, Vec::<i32>::new());
            assert_eq!(combo._uk14, -1);
            // println!("{combo:?}");
        }
    }
}
