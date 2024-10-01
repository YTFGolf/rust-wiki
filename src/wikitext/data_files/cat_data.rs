//! Module that gets information about cat names

use crate::file_handler::{get_file_location, FileLocation};
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CatName {
    #[serde(rename = "Number")]
    _id: u32,
    #[serde(rename = "First")]
    pub normal: String,
    pub evolved: Option<String>,
    pub r#true: Option<String>,
    pub ultra: Option<String>,
    #[serde(rename = "PageName")]
    pub page: String,
    pub rarity: String,
}
pub struct CatDataContainer {
    names: LazyLock<Vec<CatName>>,
}
impl CatDataContainer {
    pub fn get_cat(&self, id: u32) -> &CatName {
        return &self.names[id as usize];
    }

    pub fn get_cat_link(&self, id: u32) -> String {
        let cat = self.get_cat(id);
        format!(
            "[[{link}|{name}]]",
            link = Self::get_cat_wiki_name(&cat.normal),
            name = cat.normal
        )
    }

    pub fn get_cat_wiki_name(name: &str) -> &str {
        match name {
            "C&D Swordsman" => "Swordsman",
            "Cat Bros EX" => "Cat Bros",
            "Cat Bros R" => "Cat Bros",
            "Kitaro Cat & Nezumi-Otoko Cat 2" => "Kitaro Cat & Nezumi-Otoko Cat",
            "Cat Bros Sw" => "Cat Bros",
            "Kabuto Cat Sw" => "Kabuto Cat",
            "Kuwagata Cat Sw" => "Kuwagata Cat",
            other => other,
        }
    }
}
pub static CAT_DATA: CatDataContainer = CatDataContainer {
    names: LazyLock::new(get_cat_names),
};

fn get_cat_names() -> Vec<CatName> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(get_file_location(FileLocation::WikiData).join("UnitNames.csv"));

    rdr.unwrap()
        .deserialize::<CatName>()
        .map(|r| r.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_equals_index() {
        for (i, cat) in CAT_DATA.names.iter().enumerate() {
            assert_eq!(cat._id as usize, i);
        }
    }
}
