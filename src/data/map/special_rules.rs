//! Deals with `SpecialRulesMap.json`.

use crate::config::Config;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

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
#[derive(Debug)]
enum RuleType {
    TrustFund = 0,
    CooldownEquality = 1,
    RarityLimit = 3,
    CheapLabor = 4,
    RestrictPriceOrCd1 = 5,
    RestrictPriceOrCd2 = 6,
    DeployLimit = 7,
}
impl RuleType {
    pub fn len(&self) -> usize {
        const AMT_RARITIES: usize = 6;
        match self {
            Self::TrustFund | Self::CooldownEquality | Self::CheapLabor | Self::DeployLimit => 1,
            Self::RarityLimit | Self::RestrictPriceOrCd1 | Self::RestrictPriceOrCd2 => AMT_RARITIES,
        }
    }
}
#[derive(Debug)]
pub struct SpecialRule {
    contents_type: ContentsType,
    rule_type: Vec<(RuleType, Vec<u32>)>,
    // rule_name_label: Option<String>,
    // rule_explanation_label: Option<String>,
}
impl From<RawRuleData> for SpecialRule {
    fn from(value: RawRuleData) -> Self {
        todo!()
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

pub fn do_thing(config: &Config) {
    let data = config
        .current_version
        .get_file_path("DataLocal/SpecialRulesMap.json");
    let data: RulesMap = serde_json::from_reader(File::open(data).unwrap()).unwrap();

    // println!("{:?}", data);
    // println!("{:?}", data.get_map(1385));

    panic!("End")
}
