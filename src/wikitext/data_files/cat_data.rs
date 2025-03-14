//! Module that gets information about cat names.G

use crate::file_handler::{FileLocation, get_file_location};
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Name and data for each cat.
pub struct CatName {
    #[serde(rename = "Number")]
    _id: u32,
    #[serde(rename = "First")]
    /// Normal form name.
    pub normal: String,
    /// Evolved form name.
    pub evolved: Option<String>,
    #[serde(rename = "True")]
    /// True form name.
    pub true_form: Option<String>,
    /// Ultra form name.
    pub ultra: Option<String>,
    #[serde(rename = "PageName")]
    /// Wiki page name.
    pub page: String,
    /// Short rarity code.
    pub rarity: String,
}
/// Container for cat data.
pub struct CatDataContainer {
    names: LazyLock<Vec<CatName>>,
}
impl CatDataContainer {
    /// Get cat data from wiki ID.
    pub fn get_cat(&self, id: u32) -> &CatName {
        &self.names[id as usize]
    }

    /// Get cat link from wiki ID.
    pub fn get_cat_link(&self, id: u32) -> String {
        let cat = self.get_cat(id);
        format!(
            "[[{link}|{name}]]",
            link = cat.page,
            name = Self::clean_cat_name(&cat.normal),
        )
    }

    /// Convert unique cat name into actual cat name.
    fn clean_cat_name(name: &str) -> &str {
        match name {
            "C&D Swordsman" => "Swordsman",
            "Cat Bros EX" | "Cat Bros R" | "Cat Bros Sw" => "Cat Bros",
            "Kitaro Cat & Nezumi-Otoko Cat 2" => "Kitaro Cat & Nezumi-Otoko Cat",
            "Kabuto Cat Sw" => "Kabuto Cat",
            "Kuwagata Cat Sw" => "Kuwagata Cat",
            other => other,
        }
    }
}

/// Contains data about cats.
pub static CAT_DATA: CatDataContainer = CatDataContainer {
    names: LazyLock::new(get_cat_names),
};

fn get_cat_names() -> Vec<CatName> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(get_file_location(&FileLocation::WikiData).join("UnitNames.csv"));

    rdr.unwrap()
        .deserialize::<CatName>()
        .map(|r| r.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::used_underscore_binding)]
    fn test_id_equals_index() {
        for (i, cat) in CAT_DATA.names.iter().enumerate() {
            assert_eq!(cat._id as usize, i);
        }
    }
}
