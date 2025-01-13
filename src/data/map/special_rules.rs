//! Deals with `SpecialRulesMap.json`.

use crate::config::Config;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, mem};

#[derive(Debug, Deserialize)]
struct RawRuleType {
    #[serde(rename = "Parameters")]
    parameters: Vec<u32>,
}
#[derive(Debug, Deserialize)]
struct RawRuleData {
    #[serde(rename = "ContentsType")]
    contents_type: u8,
    #[serde(rename = "RuleType")]
    rule_type: HashMap<u32, RawRuleType>,
    #[serde(rename = "RuleNameLabel")]
    _rule_name_label: Option<String>,
    #[serde(rename = "RuleExplanationLabel")]
    _rule_explanation_label: Option<String>,
}
#[derive(Debug, Deserialize)]
struct RulesMap {
    #[serde(rename = "MapID")]
    map_id: HashMap<u32, RawRuleData>,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum ContentsType {
    Colosseum = 0,
    Anni12 = 1,
}
#[derive(Debug, Clone)]
enum RuleType {
    TrustFund = 0,
    CooldownEquality = 1,
    RarityLimit = 3,
    CheapLabor = 4,
    RestrictPriceOrCd1 = 5,
    RestrictPriceOrCd2 = 6,
    DeployLimit = 7,
}
impl From<u8> for RuleType {
    fn from(value: u8) -> Self {
        let rule = match value {
            0 => Self::TrustFund,
            1 => Self::CooldownEquality,
            3 => Self::RarityLimit,
            4 => Self::CheapLabor,
            5 => Self::RestrictPriceOrCd1,
            6 => Self::RestrictPriceOrCd2,
            7 => Self::DeployLimit,
            _ => unreachable!(),
        };

        assert_eq!(rule.clone() as u8, value);
        rule
    }
}
const AMT_RARITIES: usize = 6;
#[derive(Debug)]
enum RuleArray<T> {
    Single([T; 1]),
    Rarity([T; AMT_RARITIES]),
}
impl RuleType {
    pub fn get_rule_array(&self, params: Vec<u32>) -> RuleArray<u32> {
        match self {
            Self::TrustFund | Self::CooldownEquality | Self::CheapLabor | Self::DeployLimit => {
                const LEN: usize = 1;
                assert_eq!(params.len(), LEN);

                let mut arr = [0; LEN];
                for i in 0..LEN {
                    arr[i] = params[i];
                }

                RuleArray::Single(arr)
            }
            Self::RarityLimit | Self::RestrictPriceOrCd1 | Self::RestrictPriceOrCd2 => {
                const LEN: usize = AMT_RARITIES;
                assert_eq!(params.len(), LEN);

                let mut arr = [0; LEN];
                for i in 0..LEN {
                    arr[i] = params[i];
                }

                RuleArray::Rarity(arr)
            }
        }
    }
}
#[derive(Debug)]
pub struct SpecialRule {
    contents_type: ContentsType,
    rule_type: Vec<(RuleType, RuleArray<u32>)>,
    // rule_name_label: Option<String>,
    // rule_explanation_label: Option<String>,
}
impl From<RawRuleData> for SpecialRule {
    fn from(value: RawRuleData) -> Self {
        let contents_type = todo!();
        let rule_type = todo!();
        Self {
            contents_type,
            rule_type,
        }
    }
}

#[derive(Debug)]
pub struct SpecialRules {
    map: HashMap<u32, SpecialRule>,
}
impl SpecialRules {
    /// Get the map data that `map_id` corresponds to.
    pub fn get_map(&self, map_id: u32) -> Option<&SpecialRule> {
        self.map.get(&map_id)
    }
}
impl From<RulesMap> for SpecialRules {
    fn from(value: RulesMap) -> Self {
        todo!()
    }
}

pub fn do_thing(config: &Config) {
    let data = config
        .current_version
        .get_file_path("DataLocal/SpecialRulesMap.json");
    let data: RulesMap = serde_json::from_reader(File::open(data).unwrap()).unwrap();

    println!("{:?}", SpecialRules::from(data));
    // println!("{:?}", data.get_map(1385));

    panic!("End")
}
