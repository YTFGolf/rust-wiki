//! Deals with `ScoreBonusMap.json`.

use crate::game_data::{meta::stage::map_id::MapID, version::version_data::CacheableVersionData};
use raw::{BonusesMap, RawBonusData, RawBonusType};
use std::{collections::HashMap, fs::File};

/// Size of numeric parameters to bonuses.
type ParamSize = u32;
/// Size of the `"BonusType"` numeric keys.
type BonusKeySize = u8;

/// Raw types used for deserialising.
mod raw {
    use super::{BonusKeySize, ParamSize};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    pub struct RawBonusType {
        #[serde(rename = "Parameters")]
        pub parameters: Vec<ParamSize>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RawBonusData {
        #[serde(rename = "BonusType")]
        pub bonus_type: HashMap<BonusKeySize, RawBonusType>,
        #[serde(rename = "BonusNameLabel")]
        pub bonus_name_label: Option<String>,
        #[serde(rename = "BonusExplanationLabel")]
        _bonus_explanation_label: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct BonusesMap {
        #[serde(rename = "MapID")]
        pub map_id: HashMap<u32, RawBonusData>,
    }
}

/// Bonus with single parameter.
type Single = [ParamSize; 1];

/// Type of score bonus.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BonusType {
    /// Parameter is base points for one target.
    Weaken(Single),
    /// Parameter is base points for one target.
    Knockback(Single),
    /// Parameter is base points for one target.
    Strong(Single),
    /// Used so this program can still run when in the wrong update.
    Placeholder(u8),
}
/// Item in [`RawBonusData::bonus_type`].
type RawBonusItem = (BonusKeySize, RawBonusType);
impl From<RawBonusItem> for BonusType {
    fn from(value: RawBonusItem) -> Self {
        let params = &value.1.parameters;
        match value.0 {
            0 => Self::Weaken(Self::to_arr(params)),
            3 => Self::Knockback(Self::to_arr(params)),
            13 => Self::Strong(Self::to_arr(params)),

            id => Self::Placeholder(id),
        }
    }
}
impl BonusType {
    /// Copy `params` to statically sized array.
    fn to_arr<const N: usize>(params: &[ParamSize]) -> [ParamSize; N] {
        assert_eq!(params.len(), N, "Params is incorrect size!");
        let mut arr = [0; N];
        arr[..N].copy_from_slice(&params[..N]);
        arr
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Possible bonus name label value.
pub enum BonusNameLabel {
    /// Weaken.
    Weaken,
    /// Knockback.
    Knockback,
    /// Strong.
    Strong,
    /// Placeholder.
    Placeholder(String),
}
impl<T: AsRef<str>> From<T> for BonusNameLabel {
    fn from(value: T) -> Self {
        match value.as_ref() {
            "BonusNameLabel000" => Self::Weaken,
            "BonusNameLabel001" => Self::Knockback,
            "BonusNameLabel002" => Self::Strong,
            label => Self::Placeholder(label.to_string()),
        }
    }
}
impl BonusNameLabel {
    /// Get string representation of bonus name label.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Weaken => "Weaken",
            Self::Knockback => "Knockback",
            Self::Strong => "Strong",
            BonusNameLabel::Placeholder(label) => {
                panic!("Error: unknown score bonus label {label:?}")
            }
        }
    }
}

/// Represents all score bonuses for an individual map.
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreBonus {
    /// All of the map's bonuses.
    pub bonus_type: Vec<BonusType>,
    /// `"ResLocal/localizable"` name label
    pub bonus_name_label: Option<BonusNameLabel>,
    // bonus_explanation_label: Option<String>,
}
impl From<RawBonusData> for ScoreBonus {
    fn from(value: RawBonusData) -> Self {
        let mut bonus_type = value
            .bonus_type
            .into_iter()
            .map(BonusType::from)
            .collect::<Vec<_>>();

        bonus_type.sort();
        Self {
            bonus_type,
            bonus_name_label: value.bonus_name_label.map(BonusNameLabel::from),
        }
    }
}

/// Map of all map ids to their score bonuses.
#[derive(Debug, Default)]
pub struct ScoreBonuses {
    map: HashMap<u32, ScoreBonus>,
}
impl ScoreBonuses {
    /// Get the map data that `map_id` corresponds to.
    pub fn get_map(&self, map_id: &MapID) -> Option<&ScoreBonus> {
        self.map.get(&map_id.mapid())
    }
}
impl From<BonusesMap> for ScoreBonuses {
    fn from(value: BonusesMap) -> Self {
        let map = value
            .map_id
            .into_iter()
            .map(|(id, raw)| (id, raw.into()))
            .collect();
        Self { map }
    }
}
impl CacheableVersionData for ScoreBonuses {
    fn init_data(path: &std::path::Path) -> Self {
        let Ok(file) = File::open(path.join("DataLocal/ScoreBonusMap.json")) else {
            return Self::default();
        };
        let data: BonusesMap = serde_json::from_reader(file).unwrap();
        data.into()
    }
}

// Don't need to test for no duplicates since the file is JSON, which means all
// map ids will be unique.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn assert_no_placeholders() {
        // both for unused score bonus and for string version
        let version = TEST_CONFIG.version.current_version();
        let bonuses = version.get_cached_file::<ScoreBonuses>();

        assert!(!bonuses.map.is_empty());
        // make sure hasn't defaulted

        for bonus in &bonuses.map {
            for rtype in &bonus.1.bonus_type {
                if let BonusType::Placeholder(n) = rtype {
                    panic!("Unknown ScoreBonus id: {n}");
                }
            }
            if let Some(BonusNameLabel::Placeholder(label)) = &bonus.1.bonus_name_label {
                panic!("Error: unknown score bonus label {label:?}")
            }
        }
    }
}
