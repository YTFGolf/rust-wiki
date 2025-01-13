//! Deals with `SpecialRulesMap.json`.

use crate::config::Config;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

type ParamSize = u32;
type ContentsSize = u8;
type RuleKeySize = u8;
#[derive(Debug, Deserialize)]
struct RawRuleType {
    #[serde(rename = "Parameters")]
    parameters: Vec<ParamSize>,
}
#[derive(Debug, Deserialize)]
struct RawRuleData {
    #[serde(rename = "ContentsType")]
    contents_type: ContentsSize,
    #[serde(rename = "RuleType")]
    rule_type: HashMap<RuleKeySize, RawRuleType>,
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

#[derive(Debug, Clone)]
enum ContentsType {
    Colosseum = 0,
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

const AMT_RARITIES: usize = 6;
type Single = [ParamSize; 1];
type Rarity = [ParamSize; AMT_RARITIES];
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RuleType {
    TrustFund(Single),
    CooldownEquality(Single),
    RarityLimit(Rarity),
    CheapLabor(Single),
    RestrictPriceOrCd1(Rarity),
    RestrictPriceOrCd2(Rarity),
    DeployLimit(Single),
}
type RawRuleItem = (RuleKeySize, RawRuleType);
impl From<RawRuleItem> for RuleType {
    fn from(value: RawRuleItem) -> Self {
        let rule = match value.0 {
            0 => Self::TrustFund(Self::to_arr(value.1.parameters)),
            1 => Self::CooldownEquality(Self::to_arr(value.1.parameters)),
            //
            3 => Self::RarityLimit(Self::to_arr(value.1.parameters)),
            4 => Self::CheapLabor(Self::to_arr(value.1.parameters)),
            5 => Self::RestrictPriceOrCd1(Self::to_arr(value.1.parameters)),
            6 => Self::RestrictPriceOrCd2(Self::to_arr(value.1.parameters)),
            7 => Self::DeployLimit(Self::to_arr(value.1.parameters)),
            _ => unreachable!(),
        };

        // assert_eq!()
        // maybe figure out value of
        rule
    }
}
impl RuleType {
    fn to_arr<const N: usize>(params: Vec<ParamSize>) -> [ParamSize; N] {
        assert_eq!(params.len(), N);
        let mut arr = [0; N];
        for i in 0..N {
            arr[i] = params[i];
        }

        arr
    }

    // fn value
}
#[derive(Debug)]
pub struct SpecialRule {
    contents_type: ContentsType,
    rule_type: Vec<RuleType>,
    // rule_name_label: Option<String>,
    // rule_explanation_label: Option<String>,
}
impl From<RawRuleData> for SpecialRule {
    fn from(value: RawRuleData) -> Self {
        let contents_type = value.contents_type.into();
        let rule_type = value
            .rule_type
            .into_iter()
            .map(|crt| crt.into())
            .collect::<Vec<_>>();

        let mut rule_type = rule_type;
        rule_type.sort();
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
        let map = value
            .map_id
            .into_iter()
            .map(|(id, raw)| (id, raw.into()))
            .collect();
        Self { map }
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
