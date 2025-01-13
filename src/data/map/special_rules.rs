//! Deals with `SpecialRulesMap.json`.

use std::{any::Any, collections::HashMap, fs::File};
use crate::config::Config;
use serde::Deserialize;
use serde_json::Map;

#[derive(Debug,  Deserialize)]
struct RuleType {
    #[serde(rename = "Parameters")]
    parameters: Vec<u32>,
}

#[derive(Debug,  Deserialize)]
struct RuleData {
    #[serde(rename = "ContentsType")]
    contents_type: u32,
    #[serde(rename = "RuleType")]
    rule_type: HashMap<u32, RuleType>,
    #[serde(rename = "RuleNameLabel")]
    rule_name_label: Option<String>,
    #[serde(rename = "RuleExplanationLabel")]
    rule_explanation_label: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RulesMap {
    #[serde(rename = "MapID")]
    map_id: HashMap<u32, RuleData>,
}

pub fn do_thing(config: &Config) {
    let data = config
        .current_version
        .get_file_path("DataLocal/SpecialRulesMap.json");
    let data: RulesMap = serde_json::from_reader(File::open(data).unwrap()).unwrap();
    println!("{data:?}");

    panic!("End")
}
