use crate::game_data::version::version_data::CacheableVersionData;
use std::path::Path;

#[derive(Debug, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Level-up scale multiplier per 10 levels.
///
/// All values are multiplied by 100 to avoid using floats.
pub struct UnitLevelRaw {
    /// Level scale multiplier up to level 10.
    pub until_10: u8,
    /// Level scale multiplier up to level 20.
    pub until_20: u8,
    /// Level scale multiplier up to level 30.
    pub until_30: u8,
    /// Level scale multiplier up to level 40.
    pub until_40: u8,
    /// Level scale multiplier up to level 50.
    pub until_50: u8,
    /// Level scale multiplier up to level 60.
    pub until_60: u8,
    /// Level scale multiplier up to level 70.
    pub until_70: u8,
    /// Level scale multiplier up to level 80.
    pub until_80: u8,
    /// Level scale multiplier up to level 90.
    pub until_90: u8,
    /// Level scale multiplier up to level 100.
    pub until_100: u8,
    /// Level scale multiplier up to level 110.
    pub until_110: u8,
    /// Level scale multiplier up to level 120.
    pub until_120: u8,
    /// Level scale multiplier up to level 130.
    pub until_130: u8,
    /// Level scale multiplier up to level 140.
    pub until_140: u8,
    /// Level scale multiplier up to level 150.
    pub until_150: u8,
    /// Level scale multiplier up to level 160.
    pub until_160: u8,
    /// Level scale multiplier up to level 170.
    pub until_170: u8,
    /// Level scale multiplier up to level 180.
    pub until_180: u8,
    /// Level scale multiplier up to level 190.
    pub until_190: u8,
    /// Level scale multiplier up to level 200.
    pub until_200: u8,

    /// Should be empty.
    #[serde(default)]
    rest: Vec<u8>,
}
impl UnitLevelRaw {
    /// Raw, pre-treasure stat at level.
    pub fn get_raw_stat_at_level(&self, stat: u32, level: u8) -> u32 {
        todo!()
    }

    /// Includes treasure bonuses.
    pub fn get_stat_at_level(&self, stat: u32, level: u8) -> u32 {
        todo!()
    }
}

fn get_unitlevel(path: &Path) -> Vec<UnitLevelRaw> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path.join("DataLocal/unitlevel.csv"))
        .unwrap();

    rdr.byte_records()
        .map(|record| {
            let result = record.unwrap();
            let unit: UnitLevelRaw = result.deserialize(None).unwrap();
            unit
        })
        .collect()
}

#[derive(Debug)]
/// Container for [`UnitLevelRaw`] data.
pub struct UnitLevelContainer {
    units: Vec<UnitLevelRaw>,
}
impl UnitLevelContainer {
    /// Get [`UnitLevelRaw`] line for a unit.
    pub fn get_unit(&self, id: u32) -> Option<&UnitLevelRaw> {
        self.units.get(id as usize)
    }
}
impl CacheableVersionData for UnitLevelContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            units: get_unitlevel(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_CONFIG, game_data::version::Version};

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();
        let unitlevel = version.get_cached_file::<UnitLevelContainer>();
        let units = &unitlevel.units;

        for unit in units {
            assert_eq!(unit.rest, Vec::<u8>::new());
        }
    }

    fn get_unitlevel(id: u32, version: &Version) -> &UnitLevelRaw {
        let unitlevel = version.get_cached_file::<UnitLevelContainer>();
        unitlevel.get_unit(id).unwrap()
    }

    #[test]
    fn standard_1() {
        let mohawk = get_unitlevel(0, TEST_CONFIG.version.current_version());

        let init_hp = 200;
        let init_ap = 8;

        assert_eq!(mohawk.get_raw_stat_at_level(init_hp, 1), init_hp);
        assert_eq!(mohawk.get_raw_stat_at_level(init_ap, 1), init_ap);
        assert_eq!(mohawk.get_raw_stat_at_level(init_hp, 1), init_hp * 25 / 10);
        assert_eq!(mohawk.get_raw_stat_at_level(init_ap, 1), init_ap * 25 / 10);
    }
}
