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
    contents_type: u32,
    #[serde(rename = "RuleType")]
    rule_type: HashMap<u32, RawRuleType>,
    #[serde(rename = "RuleNameLabel")]
    rule_name_label: Option<String>,
    #[serde(rename = "RuleExplanationLabel")]
    rule_explanation_label: Option<String>,
}
#[derive(Debug, Deserialize)]
struct RulesMap {
    #[serde(rename = "MapID")]
    map_id: HashMap<u32, RawRuleData>,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct SpecialRule<'a> {
    contents_type: u32,
    rule_type: HashMap<u32, &'a [u32]>,
    rule_name_label: Option<String>,
    rule_explanation_label: Option<String>,
}

#[derive(Debug)]
pub struct SpecialRules<'a> {
    map: HashMap<u32, SpecialRule<'a>>,
}
impl SpecialRules<'_> {
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
