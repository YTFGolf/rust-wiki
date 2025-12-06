//! Cat talents

use crate::game_data::version::{
    Version,
    version_data::{CacheableVersionData, CvdCreateError, CvdCreateHandler, CvdResult},
};
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
    pub ability_id_x: u8,
    /// Max level talent can be upgraded to.
    pub maxlv_x: u8,
    /// Min value of parameter 1.
    pub min_x1: u16,
    /// Max value of parameter 1.
    pub max_x1: u16,
    /// Min value of parameter 2.
    pub min_x2: u16,
    /// Max value of parameter 2.
    pub max_x2: u16,
    /// Min value of parameter 3.
    pub min_x3: u16,
    /// Max value of parameter 3.
    pub max_x3: u16,
    /// Min value of parameter 4.
    pub min_x4: u16,
    /// Max value of parameter 4.
    pub max_x4: u16,
    /// ID of talent description (SkillDescriptions.csv).
    pub text_id_x: u8,
    /// ID of costs (SkillLevel.csv).
    pub lv_id_x: u8,
    /// Something to do with abilities that also add a target.
    pub name_id_x: i16,
    /// 0 for normal, 1 for ultra.
    pub limit_x: u8,
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

type E = String;
fn parse_talents_line(line: &str) -> Result<TalentLine, E> {
    const FIXED_LEN: usize = 2;
    // fields in TalentsFixed
    const GROUP_LEN: usize = 14;
    // fields in TalentGroup
    const TOTAL_AMT_FIELDS: usize = FIXED_LEN + AMT_GROUPS * GROUP_LEN;

    let line = line.split(',').collect::<Vec<_>>();
    assert_eq!(line.len(), TOTAL_AMT_FIELDS);
    let line: [&str; TOTAL_AMT_FIELDS] = line
        .try_into()
        .expect("is a `Vec<&str>` of length `TOTAL_AMT_FIELDS`");

    fn parse_index<T: FromStr>(line: &[&str; TOTAL_AMT_FIELDS], i: usize) -> Result<T, E> {
        line[i].parse().map_err(|_| {
            assert!(
                i >= FIXED_LEN,
                "error when attempting to parse fixed index {i} into {tname}: {field:?}",
                tname = type_name::<T>(),
                field = line[i]
            );

            let j = (i - FIXED_LEN) / GROUP_LEN;
            let k = (i - FIXED_LEN) % GROUP_LEN;
            format!(
                "error when attempting to parse index {i}/{j}.{k} into {tname}: {field:?}",
                tname = type_name::<T>(),
                field = line[i]
            )
        })
    }

    let fixed = TalentsFixed {
        id: parse_index(&line, 0)?,
        type_id: parse_index(&line, 1)?,
    };

    let groups = (0..AMT_GROUPS)
        .map(|i| {
            let first = i * GROUP_LEN + FIXED_LEN;

            Ok(TalentGroup {
                ability_id_x: parse_index(&line, first + 0)?,
                maxlv_x: parse_index(&line, first + 1)?,
                min_x1: parse_index(&line, first + 2)?,
                max_x1: parse_index(&line, first + 3)?,
                min_x2: parse_index(&line, first + 4)?,
                max_x2: parse_index(&line, first + 5)?,
                min_x3: parse_index(&line, first + 6)?,
                max_x3: parse_index(&line, first + 7)?,
                min_x4: parse_index(&line, first + 8)?,
                max_x4: parse_index(&line, first + 9)?,
                text_id_x: parse_index(&line, first + 10)?,
                lv_id_x: parse_index(&line, first + 11)?,
                name_id_x: parse_index(&line, first + 12)?,
                limit_x: parse_index(&line, first + 13)?,
            })
        })
        .collect::<Result<Vec<_>, E>>()?
        .try_into()
        .expect("both this function and the return type use the `AMT_GROUPS` constant");

    Ok(TalentLine { fixed, groups })
}

/// Get all data from the talents file.
fn get_talents_file(path: &Path) -> Result<Vec<TalentLine>, E> {
    let reader = BufReader::new(File::open(path.join("DataLocal/SkillAcquisition.csv")).unwrap());

    reader
        .lines()
        .skip(1)
        .map(|line| parse_talents_line(&line.unwrap()))
        .collect()
}

#[derive(Debug, Default)]
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
    fn create(version: &Version) -> CvdResult<Self> {
        let talents = get_talents_file(version.location()).map_err(|e| CvdCreateError {
            handler: CvdCreateHandler::Default(Self::default()),
            err: Box::new(e),
        })?;
        Ok(Self { talents })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn check_talents() {
        for line in get_talents_file(TEST_CONFIG.version.current_version().location()).unwrap() {
            println!("{line:?}");
        }
        // todo!("What did I even need to test here")
        // was probably nothing, just that it parses correctly
    }
}
