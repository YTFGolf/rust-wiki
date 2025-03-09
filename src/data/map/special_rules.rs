//! Deals with `SpecialRulesMap.json`.

use crate::{data::version::version_data::CacheableVersionData, meta::stage::map_id::MapID};
use raw::{RawRuleData, RawRuleType, RulesMap};
use std::{collections::HashMap, fs::File};

/// Size of numeric parameters to rules.
type ParamSize = u32;
/// Size of the `"ContentsType"` numeric field.
type ContentsSize = u8;
/// Size of the `"RuleType"` numeric keys.
type RuleKeySize = u8;

/// Raw types used for deserialising.
mod raw {
    use super::{ContentsSize, ParamSize, RuleKeySize};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    pub struct RawRuleType {
        #[serde(rename = "Parameters")]
        pub parameters: Vec<ParamSize>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RawRuleData {
        #[serde(rename = "ContentsType")]
        pub contents_type: ContentsSize,
        #[serde(rename = "RuleType")]
        pub rule_type: HashMap<RuleKeySize, RawRuleType>,
        #[serde(rename = "RuleNameLabel")]
        pub rule_name_label: Option<String>,
        #[serde(rename = "RuleExplanationLabel")]
        _rule_explanation_label: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    pub struct RulesMap {
        #[serde(rename = "MapID")]
        pub map_id: HashMap<u32, RawRuleData>,
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Exact meaning is unclear.
pub enum ContentsType {
    /// Only used in Colosseum stages.
    Colosseum = 0,
    /// Only used in 12th anniversary stages.
    Anni12 = 1,
}
impl From<ContentsSize> for ContentsType {
    fn from(value: ContentsSize) -> Self {
        let ctype = match value {
            0 => Self::Colosseum,
            1 => Self::Anni12,
            _ => unreachable!(),
        };

        assert_eq!(ctype.clone() as ContentsSize, value);
        ctype
    }
}

/// Rarities in game.
const AMT_RARITIES: usize = 6;
/// Rule with single parameter.
type Single = [ParamSize; 1];
/// Rule with parameters for each rarity.
type Rarity = [ParamSize; AMT_RARITIES];
type AwesomeCatSpawn = [ParamSize; 3];

/// Type of special rule.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuleType {
    /// Param is starting cash in ¢.
    TrustFund(Single),
    /// Param is frames that global cooldown is set to.
    CooldownEquality(Single),
    /// Limit each rarity to specified amount of deploys.
    ///
    /// Used in Only One Rarity, where each param is 1.
    RarityLimit(Rarity),
    /// Param is global unit cost.
    CheapLabor(Single),
    /// Restrict either the price or the cooldown of enemies in each rarity.
    ///
    /// Used in multiple different Colosseum rules. A rarity's value is its
    /// cost/cd as a percentage of its usual.
    RestrictPriceOrCd1(Rarity),
    /// See [RuleType::RestrictPriceOrCd1]. Since these never appear
    /// individually, it's impossible to tell them apart.
    RestrictPriceOrCd2(Rarity),
    /// Param is max units that can be spawned in battle.
    DeployLimit(Single),
    /// Speculative.
    ///
    /// Only used in the context of `[16, 1, 60]`. Based on what is known this
    /// is probably `[bitmask, extra_spawned, delayf]`.
    AwesomeCatSpawn(AwesomeCatSpawn),
}
/// Item in [RawRuleData::rule_type].
type RawRuleItem = (RuleKeySize, RawRuleType);
impl From<RawRuleItem> for RuleType {
    fn from(value: RawRuleItem) -> Self {
        let params = &value.1.parameters;
        match value.0 {
            0 => Self::TrustFund(Self::to_arr(params)),
            1 => Self::CooldownEquality(Self::to_arr(params)),
            //
            3 => Self::RarityLimit(Self::to_arr(params)),
            4 => Self::CheapLabor(Self::to_arr(params)),
            5 => Self::RestrictPriceOrCd1(Self::to_arr(params)),
            6 => Self::RestrictPriceOrCd2(Self::to_arr(params)),
            7 => Self::DeployLimit(Self::to_arr(params)),
            8 => Self::AwesomeCatSpawn(Self::to_arr(params)),
            id => panic!("Unknown rule id: {id}"),
        }
    }
}
impl RuleType {
    /// Copy `params` to statically sized array.
    fn to_arr<const N: usize>(params: &[ParamSize]) -> [ParamSize; N] {
        assert_eq!(params.len(), N, "Params is incorrect size!");
        let mut arr = [0; N];
        arr[..N].copy_from_slice(&params[..N]);
        arr
    }
}

/// Represents all special rules for an individual map.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecialRule {
    /// Unclear what the purpose is, other than war funds.
    pub contents_type: ContentsType,
    /// All of the map's rules.
    pub rule_type: Vec<RuleType>,
    /// `"ResLocal/localizable"` name label
    pub rule_name_label: Option<String>,
    // rule_explanation_label: Option<String>,
}
impl From<RawRuleData> for SpecialRule {
    fn from(value: RawRuleData) -> Self {
        let contents_type = value.contents_type.into();
        let mut rule_type = value
            .rule_type
            .into_iter()
            .map(RuleType::from)
            .collect::<Vec<_>>();

        rule_type.sort();
        Self {
            contents_type,
            rule_type,
            rule_name_label: value.rule_name_label,
        }
    }
}

/// Map of all map ids to their special rules.
#[derive(Debug)]
pub struct SpecialRules {
    map: HashMap<u32, SpecialRule>,
}
impl SpecialRules {
    /// Get the map data that `map_id` corresponds to.
    pub fn get_map(&self, map_id: &MapID) -> Option<&SpecialRule> {
        self.map.get(&map_id.mapid())
    }
}
impl From<RulesMap> for SpecialRules {
    fn from(value: RulesMap) -> Self {
        let map = value
            .map_id
            .into_iter()
            .map(|(id, raw)| (id, raw.into()))
            .collect();
        Self { map }
    }
}
impl CacheableVersionData for SpecialRules {
    fn init_data(path: &std::path::Path) -> Self {
        let data: RulesMap = serde_json::from_reader(
            File::open(path.join("DataLocal/SpecialRulesMap.json")).unwrap(),
        )
        .unwrap();
        data.into()
    }
}

// Don't need to test for no duplicates since the file is JSON, which means all
// map ids will be unique.
