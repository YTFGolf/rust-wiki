//! Deals with `SpecialRulesMap.json`.

use crate::{data::version::version_data::CacheableVersionData, meta::stage::map_id::MapID};
use raw::{RawRuleData, RawRuleType, RulesMap};
use std::{collections::HashMap, fs::File};
use strum::FromRepr;

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

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, FromRepr)]
/// Exact meaning is unclear.
pub enum ContentsType {
    /// Only used in Colosseum stages.
    Colosseum = 0,
    /// Only used in 12th anniversary stages.
    Anni12 = 1,
}
impl From<ContentsSize> for ContentsType {
    fn from(value: ContentsSize) -> Self {
        ContentsType::from_repr(value)
            .unwrap_or_else(|| panic!("Unexpected SpecialRule ContentsType value: {value}."))
    }
}

/// Rarities in game.
const AMT_RARITIES: usize = 6;
/// Rule with single parameter.
type Single = [ParamSize; 1];
/// Rule with parameters for each rarity.
type Rarity = [ParamSize; AMT_RARITIES];

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
    /// Spawn extra cats automatically.
    ///
    /// Params are `[rarity_bitmask, extra_cats_spawned, spawn_delay_f]`.
    AwesomeCatSpawn([ParamSize; 3]),
    /// Increase Cat Cannon damage. Param is a percentage value.
    AwesomeCatCannon(Single),
    /// Normalise Cat and Enemy speed.
    ///
    /// 100% speculation, but is probably `[min_cat_speed, normalised_cat_speed,
    /// min_enemy_speed, normalised_enemy_speed]`.
    AwesomeUnitSpeed([ParamSize; 4]),
    /// Used so this program can still run when in the wrong update.
    Placeholder(u8),
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
            9 => Self::AwesomeCatCannon(Self::to_arr(params)),
            10 => Self::AwesomeUnitSpeed(Self::to_arr(params)),
            id => Self::Placeholder(id),
        }
        // unfortunately using a match is probably the only way to do this since
        // strum doesn't have the capability and even if it did I couldn't mark
        // the discriminants with explicit values
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Possible rule name label value.
pub enum RuleNameLabel {
    /// Trust Fund.
    TrustFund,
    /// Cooldown Equality.
    CooldownEquality,
    /// Only One Rarity.
    OnlyOneRarity,
    /// Cheap Labor.
    CheapLabor,
    /// Super Rare Sale.
    SuperRareSale,
    /// Deploy Limit.
    DeployLimit,
    /// Special Clearance.
    SpecialClearance,
    /// Plus One: Uber.
    PlusOneUber,
    /// Mega Cat Cannon.
    MegaCatCannon,
    /// 等速直線運動
    等速直線運動,
    /// プラスワン EX
    プラスワンEX,
}
impl<T: AsRef<str>> From<T> for RuleNameLabel {
    fn from(value: T) -> Self {
        match value.as_ref() {
            "SpecialRuleName000" => Self::TrustFund,
            "SpecialRuleName001" => Self::CooldownEquality,
            "SpecialRuleName002" => Self::OnlyOneRarity,
            "SpecialRuleName003" => Self::CheapLabor,
            "SpecialRuleName004" => Self::SuperRareSale,
            "SpecialRuleName005" => Self::DeployLimit,
            "SpecialRuleName006" => Self::SpecialClearance,
            "SpecialRuleName007" => Self::PlusOneUber,
            "SpecialRuleName008" => Self::MegaCatCannon,
            "SpecialRuleName009" => Self::等速直線運動,
            "SpecialRuleName010" => Self::プラスワンEX,
            label => panic!("Error: unknown special rule label {label:?}"),
        }
    }
}
impl RuleNameLabel {
    /// Get string representation of rule name label.
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleNameLabel::TrustFund => "Trust Fund",
            RuleNameLabel::CooldownEquality => "Cooldown Equality",
            RuleNameLabel::OnlyOneRarity => "Only One Rarity",
            RuleNameLabel::CheapLabor => "Cheap Labor",
            RuleNameLabel::SuperRareSale => "Super Rare Sale",
            RuleNameLabel::DeployLimit => "Deploy Limit",
            RuleNameLabel::SpecialClearance => "Special Clearance",
            RuleNameLabel::PlusOneUber => "Plus One: Uber",
            RuleNameLabel::MegaCatCannon => "Mega Cat Cannon",
            RuleNameLabel::等速直線運動 => "等速直線運動",
            RuleNameLabel::プラスワンEX => "プラスワン EX",
        }
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
    pub rule_name_label: Option<RuleNameLabel>,
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
            rule_name_label: value.rule_name_label.map(RuleNameLabel::from),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TEST_CONFIG;

    #[test]
    fn assert_no_placeholders() {
        let version = TEST_CONFIG.version.current_version();
        let rules = version.get_cached_file::<SpecialRules>();
        for rule in rules.map.iter() {
            for rtype in rule.1.rule_type.iter() {
                if let RuleType::Placeholder(n) = rtype {
                    panic!("Unknown SpecialRule id: {n}");
                }
            }
        }
    }
}
