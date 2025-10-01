//! Cat talents

#![allow(non_snake_case, dead_code)]

use crate::game_data::version::version_data::CacheableVersionData;
use std::{
    any::type_name,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

/// Talents block.
#[derive(Debug)]
pub struct TalentsFixed {
    /// ID of cat unit.
    pub id: u16,
    /// Enemies that are newly targeted by name_id talents.
    pub type_id: u16,
}

/// Repeated group of talents.
#[derive(Debug)]
pub struct TalentGroup {
    /// ID of ability affected by talent.
    pub abilityID_X: u8,
    pub MAXLv_X: u8,
    pub min_X1: u16,
    pub max_X1: u16,
    pub min_X2: u16,
    pub max_X2: u16,
    pub min_X3: u16,
    pub max_X3: u16,
    pub min_X4: u16,
    pub max_X4: u16,
    /// ID of talent description (SkillDescriptions.csv).
    pub textID_X: u8,
    pub LvID_X: u8,
    /// Something to do with abilities that also add a target.
    pub nameID_X: i16,
    /// 0 for normal, 1 for ultra.
    pub limit_X: u8,
}
const AMT_GROUPS: usize = 8;

/// Container for a single line of talents.
#[derive(Debug)]
pub struct TalentLine {
    /// Fixed data for each line.
    pub fixed: TalentsFixed,
    /// Groups of talents.
    pub groups: [TalentGroup; AMT_GROUPS],
}

fn parse_talents_line(line: &str) -> TalentLine {
    const FIXED_LEN: usize = 2;
    // fields in TalentsFixed
    const GROUP_LEN: usize = 14;
    // fields in TalentGroup
    const TOTAL_AMT_FIELDS: usize = FIXED_LEN + AMT_GROUPS * GROUP_LEN;

    let line = line.split(',').collect::<Vec<_>>();
    assert_eq!(line.len(), TOTAL_AMT_FIELDS);
    let line: [&str; TOTAL_AMT_FIELDS] = line.try_into().unwrap();

    fn parse_index<T: FromStr>(line: &[&str; TOTAL_AMT_FIELDS], i: usize) -> T {
        line[i].parse().unwrap_or_else(|_| {
            if i < FIXED_LEN {
                panic!(
                    "error when attempting to parse fixed index {i} into {tname}: {field:?}",
                    tname = type_name::<T>(),
                    field = line[i]
                );
            }

            let j = (i - FIXED_LEN) / GROUP_LEN;
            let k = (i - FIXED_LEN) % GROUP_LEN;
            panic!(
                "error when attempting to parse index {i}/{j}.{k} into {tname}: {field:?}",
                tname = type_name::<T>(),
                field = line[i]
            )
        })
    }

    let fixed = TalentsFixed {
        id: parse_index(&line, 0),
        type_id: parse_index(&line, 1),
    };

    let groups = (0..AMT_GROUPS)
        .map(|i| {
            let first = i * GROUP_LEN + FIXED_LEN;

            TalentGroup {
                abilityID_X: parse_index(&line, first + 0),
                MAXLv_X: parse_index(&line, first + 1),
                min_X1: parse_index(&line, first + 2),
                max_X1: parse_index(&line, first + 3),
                min_X2: parse_index(&line, first + 4),
                max_X2: parse_index(&line, first + 5),
                min_X3: parse_index(&line, first + 6),
                max_X3: parse_index(&line, first + 7),
                min_X4: parse_index(&line, first + 8),
                max_X4: parse_index(&line, first + 9),
                textID_X: parse_index(&line, first + 10),
                LvID_X: parse_index(&line, first + 11),
                nameID_X: parse_index(&line, first + 12),
                limit_X: parse_index(&line, first + 13),
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    TalentLine { fixed, groups }
}

/// Get all data from the talents file.
fn get_talents_file(path: &Path) -> Vec<TalentLine> {
    let reader = BufReader::new(File::open(path.join("DataLocal/SkillAcquisition.csv")).unwrap());

    reader
        .lines()
        .skip(1)
        .map(|line| parse_talents_line(&line.unwrap()))
        .collect()
}

#[derive(Debug)]
/// Container for talents.
pub struct TalentsContainer {
    talents: Vec<TalentLine>,
}
impl TalentsContainer {
    /// Get unit's talents from their id.
    pub fn from_id(&self, id: u16) -> Option<&TalentLine> {
        self.talents.iter().find(|t| t.fixed.id == id)
    }

    /// Iterate through all talents.
    pub fn iter(&self) -> impl Iterator<Item = &TalentLine> {
        self.talents.iter()
    }
}
impl CacheableVersionData for TalentsContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            talents: get_talents_file(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn check_talents() {
        for line in get_talents_file(TEST_CONFIG.version.current_version().location()) {
            println!("{line:?}")
        }
        // todo!("What did I even need to test here")
    }
}
