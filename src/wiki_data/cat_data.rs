//! Module that gets information about cat names.G

use crate::wiki_data::file_handler::get_wiki_data_location;
use serde::Deserialize;
use std::{collections::BTreeMap, fs::File, sync::LazyLock};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Name and data for each cat.
pub struct CatName {
    #[serde(rename = "Number")]
    id: u32,
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
    replacements: LazyLock<BTreeMap<String, String>>,
}
impl CatDataContainer {
    /// Get cat replacement name (e.g. `Cat Bros EX` -> `Cat Bros`).
    pub fn get_cat_replacement(&self, name: &str) -> Option<&str> {
        self.replacements.get(name).map(|x| x.as_str())
    }

    /// Get cat replacement name (e.g. `Cat Bros EX` -> `Cat Bros`).
    pub fn replace_name<'a>(&'a self, name: &str) -> Cow<'a, str> {
        // I have no idea what to do with this
        match self.get_cat_replacement(name) {
            Some(n) => Cow::Owned(n.into()),
            None => Cow::Owned(name.into()),
        }
    }
}
impl CatDataContainer {
    /// Try to get cat data from wiki ID.
    pub fn try_get_cat(&self, id: usize) -> Option<&CatName> {
        self.names.get(id)
    }

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
            name = self.replace_name(&cat.normal),
        )
    }

    /// Get unit's name from id. Case-insensitive. Works for all 4 forms.
    pub fn get_id_from_name(&self, name: &str) -> Option<u32> {
        let name = name.to_lowercase();
        for cat in self.names.iter() {
            for form in [
                Some(&cat.normal),
                cat.evolved.as_ref(),
                cat.true_form.as_ref(),
                cat.ultra.as_ref(),
            ]
            .iter()
            .flatten()
            {
                if form.to_lowercase() == name {
                    return Some(cat.id);
                }
            }
        }
        // self.reverse_id_map.get(&name.to_lowercase())
        None
    }
}

/// Contains data about cats.
pub static CAT_DATA: CatDataContainer = CatDataContainer {
    names: LazyLock::new(get_cat_names),
    replacements: LazyLock::new(get_cat_replacements),
};

fn get_cat_names() -> Vec<CatName> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(get_wiki_data_location().join("UnitNames.csv"));

    rdr.unwrap()
        .deserialize::<CatName>()
        .map(|r| r.unwrap())
        .collect()
}

fn get_cat_replacements() -> BTreeMap<String, String> {
    let f = File::open(get_wiki_data_location().join("UnitReplacements.json")).unwrap();
    serde_json::from_reader(f).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::used_underscore_binding)]
    fn test_id_equals_index() {
        for (i, cat) in CAT_DATA.names.iter().enumerate() {
            assert_eq!(cat.id as usize, i);
        }
    }
}
