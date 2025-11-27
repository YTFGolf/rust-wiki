//! Get information about stage rewards.

use crate::wiki_data::file_handler::get_wiki_data_location;
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize, Default)]
/// Entry in the TalentNames.csv file.
pub struct TalentEntry {
    _id: usize,
    /// Name of talent.
    pub name: String,
}

/// Container for [`TALENT_DATA`].
pub struct TalentMap {
    map: LazyLock<Vec<TalentEntry>>,
}
impl TalentMap {
    fn get_talent(&self, id: usize) -> &TalentEntry {
        self.map
            .get(id)
            .unwrap_or_else(|| panic!("talent id not found: {id}."))
    }
    /// Get the name of the talent.
    pub fn get_talent_name(&self, id: usize) -> &str {
        &self.get_talent(id).name
    }
}

/// Contains data about talents.
pub static TALENT_DATA: TalentMap = TalentMap {
    map: LazyLock::new(get_talent_data),
};

fn get_talent_data() -> Vec<TalentEntry> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(get_wiki_data_location().join("TalentNames.csv"));

    let mut data = vec![TalentEntry::default()];
    // first entry has id 1 so this is necessary to ensure indices line up.
    data.extend(rdr.unwrap().byte_records().map(|result| {
        let talent = result.unwrap().deserialize::<TalentEntry>(None).unwrap();
        talent
    }));

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::used_underscore_binding)]
    fn ids_are_same_as_index() {
        for (i, talent) in TALENT_DATA.map.iter().enumerate() {
            assert_eq!(i, talent._id, "talent {talent:?} has incorrect id");
        }
    }
}
