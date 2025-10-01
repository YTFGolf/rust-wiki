//! Cat talents

#![allow(non_snake_case, dead_code)]

use csv::ByteRecord;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Talents block.
#[derive(Debug, Default)] // default is only for now
pub struct TalentsFixed {
    ID: u8,
    typeID: u8,
}

/// Repeated group of talents.
#[derive(Debug, Default)] // default is only for now
pub struct TalentGroup {
    abilityID_X: u8,
    MAXLv_X: u8,
    min_X1: u8,
    max_X1: u8,
    min_X2: u8,
    max_X2: u8,
    min_X3: u8,
    max_X3: u8,
    min_X4: u8,
    max_X4: u8,
    textID_X: u8,
    LvID_X: u8,
    nameID_X: u8,
    limit_X: u8,
}
const AMT_GROUPS: usize = 8;

/// Container for a single line of talents.
#[derive(Debug)]
pub struct Talents {
    fixed: TalentsFixed,
    groups: [TalentGroup; AMT_GROUPS],
}

fn get_talents_file(path: &Path) -> Vec<Talents> {
    let reader = BufReader::new(File::open(path.join("DataLocal/SkillAcquisition.csv")).unwrap());

    reader
        .lines()
        .map(|line| {
            const FIXED_LEN: usize = 2;
            // fields in TalentsFixed
            const GROUP_LEN: usize = 14;
            // fields in TalentGroup

            let line = line.unwrap();
            let line = line.split(',').collect::<Vec<_>>();
            assert_eq!(line.len(), FIXED_LEN + AMT_GROUPS * GROUP_LEN);

            let fixed = TalentsFixed {
                ID: line[0].parse().unwrap(),
                typeID: line[1].parse().unwrap(),
            };

            let groups = (0..AMT_GROUPS)
                .map(|i| {
                    let first = i * AMT_GROUPS + FIXED_LEN;

                    TalentGroup {
                        abilityID_X: line[first + 0].parse().unwrap(),
                        MAXLv_X: line[first + 1].parse().unwrap(),
                        min_X1: line[first + 2].parse().unwrap(),
                        max_X1: line[first + 3].parse().unwrap(),
                        min_X2: line[first + 4].parse().unwrap(),
                        max_X2: line[first + 5].parse().unwrap(),
                        min_X3: line[first + 6].parse().unwrap(),
                        max_X3: line[first + 7].parse().unwrap(),
                        min_X4: line[first + 8].parse().unwrap(),
                        max_X4: line[first + 9].parse().unwrap(),
                        textID_X: line[first + 10].parse().unwrap(),
                        LvID_X: line[first + 11].parse().unwrap(),
                        nameID_X: line[first + 12].parse().unwrap(),
                        limit_X: line[first + 13].parse().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            Talents { fixed, groups }
        })
        .collect()
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
        todo!()
    }
}
