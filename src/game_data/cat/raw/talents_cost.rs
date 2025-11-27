//! Cat talent costs.

use crate::game_data::version::version_data::CacheableVersionData;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Default, Clone)]
/// Cost of acquiring talent with this cost id.
pub struct TalentAcquisitionCost {
    /// Cost id for internal use.
    pub id: usize,
    /// Costs for each level.
    pub costs: Vec<u16>,
}

/// Get all data from the talents file.
fn get_talent_costs_file(path: &Path) -> Vec<TalentAcquisitionCost> {
    let reader = BufReader::new(File::open(path.join("DataLocal/SkillLevel.csv")).unwrap());

    let mut costs_cont = vec![Default::default()];
    costs_cont.extend(reader.lines().skip(1).map(|line| {
        let line: &str = &line.unwrap();
        let mut iter = line.split(',');

        let first = iter.next().expect("first shouldn't fail");
        let id = first.parse().unwrap();
        let mut costs = vec![];

        for rest in iter {
            if rest.is_empty() {
                break;
            }
            costs.push(rest.parse().unwrap());
        }

        TalentAcquisitionCost { id, costs }
    }));

    costs_cont
}

#[derive(Debug)]
/// Container for talent costs.
pub struct TalentsCostContainer {
    costs_cont: Vec<TalentAcquisitionCost>,
}
impl TalentsCostContainer {
    /// Get costs with this id.
    pub fn from_cost_id(&self, id: usize) -> Option<&TalentAcquisitionCost> {
        self.costs_cont.get(id)
    }
}
impl CacheableVersionData for TalentsCostContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            costs_cont: get_talent_costs_file(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn check_talents() {
        for (i, line) in get_talent_costs_file(TEST_CONFIG.version.current_version().location())
            .iter()
            .enumerate()
        {
            assert_eq!(i, line.id);
        }
    }
}
