//! Cat talent costs.

use crate::game_data::version::{
    Version,
    version_data::{CacheableVersionData, CvdCreateError, CvdResult},
};
use std::{
    error::Error,
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
fn get_talent_costs_file(path: &Path) -> Result<Vec<TalentAcquisitionCost>, Box<dyn Error>> {
    let reader =
        BufReader::new(File::open(path.join("DataLocal/SkillLevel.csv")).map_err(Box::new)?);

    let mut costs_cont = vec![Default::default()];
    // dummy container so that `.from_cost_id(1)` is just `.get(1)`

    let parsed: Result<Vec<_>, Box<dyn Error>> = reader
        .lines()
        .skip(1)
        .map(|line| {
            let line: &str = &line.map_err(Box::new)?;
            let mut iter = line.split(',');

            let first = iter.next().expect("first shouldn't fail");
            let id = first.parse().map_err(Box::new)?;
            let mut costs = vec![];

            for rest in iter {
                if rest.is_empty() {
                    break;
                }
                costs.push(rest.parse().map_err(Box::new)?);
            }

            Ok(TalentAcquisitionCost { id, costs })
        })
        .collect();
    costs_cont.extend(parsed?);

    Ok(costs_cont)
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
    fn create(version: &Version) -> CvdResult<Self> {
        Ok(Self {
            costs_cont: get_talent_costs_file(version.location()).map_err(CvdCreateError::throw)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn check_talents() {
        for (i, line) in get_talent_costs_file(TEST_CONFIG.version.current_version().location())
            .unwrap()
            .iter()
            .enumerate()
        {
            assert_eq!(i, line.id);
        }
    }
}
