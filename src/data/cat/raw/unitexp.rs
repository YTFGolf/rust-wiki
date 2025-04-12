//! Deals with level-up cost multipliers.
//!
//! Because (as of 14.3) Superfeline is the *only* unit with different
//! multipliers (constant price instead of increasing every 10 levels) this
//! doesn't use [`CacheableVersionData`][CacheableVersionData] but instead uses
//! constants. Unit tests check these constants for correctness.
//!
//! [CacheableVersionData]: crate::data::version::version_data

#[derive(Debug, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Level-up cost multiplier per 10 levels.
///
/// All values are multiplied by 10 to avoid using floats.
pub struct UnitExp {
    /// XP cost multiplier up to level 10.
    pub until_10: u8,
    /// XP cost multiplier up to level 20.
    pub until_20: u8,
    /// XP cost multiplier up to level 30.
    pub until_30: u8,
    /// XP cost multiplier up to level 40.
    pub until_40: u8,
    /// XP cost multiplier up to level 50.
    pub until_50: u8,
    /// XP cost multiplier up to level 60.
    pub until_60: u8,
    /// XP cost multiplier up to level 70.
    pub until_70: u8,
    /// XP cost multiplier up to level 80.
    pub until_80: u8,
    /// XP cost multiplier up to level 90.
    pub until_90: u8,
    /// XP cost multiplier up to level 100.
    pub until_100: u8,
    /// XP cost multiplier up to level 110.
    pub until_110: u8,
    /// XP cost multiplier up to level 120.
    pub until_120: u8,
    /// XP cost multiplier up to level 130.
    pub until_130: u8,
    /// XP cost multiplier up to level 140.
    pub until_140: u8,
    /// XP cost multiplier up to level 150.
    pub until_150: u8,
    /// XP cost multiplier up to level 160.
    pub until_160: u8,
    /// XP cost multiplier up to level 170.
    pub until_170: u8,
    /// XP cost multiplier up to level 180.
    pub until_180: u8,
    /// XP cost multiplier up to level 190.
    pub until_190: u8,
    /// XP cost multiplier up to level 200.
    pub until_200: u8,
}

/// Multipliers for nearly every enemy in the game.
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

/// Multipliers for superfeline.
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

/// Levelling enum to avoid using a big object.
pub enum Levelling {
    /// Default level cost growth.
    Normal,
    /// Superfeline level cost growth.
    Superfeline,
}
impl Levelling {
    /// Get levelling from unit id.
    pub const fn from_id(id: u32) -> Self {
        match id {
            643 => Self::Superfeline,
            _ => Self::Normal,
        }
    }

    /// Get [`UnitExp`] levelling multipliers.
    pub const fn get_levelling(&self) -> UnitExp {
        match self {
            Levelling::Normal => DEFAULT,
            Levelling::Superfeline => SUPERFELINE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TEST_CONFIG;
    use std::path::Path;

    fn get_unitexp(path: &Path) -> Vec<UnitExp> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path.join("DataLocal/unitexp.csv"))
            .unwrap();

        rdr.byte_records()
            .map(|record| {
                let result = record.unwrap();
                assert_eq!(result.len(), std::mem::size_of::<UnitExp>());
                // make sure that the struct definition is up-to-date
                let unit: UnitExp = result.deserialize(None).unwrap();
                unit
            })
            .collect()
    }

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();
        let units = get_unitexp(&version.get_file_path(""));

        for (i, unit) in units.into_iter().enumerate() {
            let level = Levelling::from_id(i as u32);
            assert_eq!(level.get_levelling(), unit);
        }
    }

    #[test]
    fn test_units() {
        // print them out just for show
        let version = TEST_CONFIG.version.current_version();
        let unitexp = get_unitexp(&version.get_file_path(""));

        let test_units = [("cat", 0), ("sfeline", 643)];
        for (name, id) in test_units {
            println!("{name} ({id}) = {:?}", unitexp[id]);
        }
    }
}
