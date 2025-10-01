//! Cat talents

#![allow(non_snake_case, dead_code)]

use csv::ByteRecord;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
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
    LvID_X: i8,
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
        .skip(1)
        .map(|line| {
            const FIXED_LEN: usize = 2;
            // fields in TalentsFixed
            const GROUP_LEN: usize = 14;
            // fields in TalentGroup
            const TOTAL_AMT_FIELDS: usize = FIXED_LEN + AMT_GROUPS * GROUP_LEN;

            let line = line.unwrap();
            let line = line.split(',').collect::<Vec<_>>();
            assert_eq!(line.len(), TOTAL_AMT_FIELDS);
            let line: [&str; TOTAL_AMT_FIELDS] = line.try_into().unwrap();

            fn parse_index<T: FromStr>(line: &[&str; TOTAL_AMT_FIELDS], i: usize) -> T {
                line[i].parse().unwrap_or_else(|_| {
                    if i < FIXED_LEN {
                        panic!(
                            "error when attempting to parse index {i} into {tname}: {field:?}",
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
                ID: parse_index(&line, 0),
                typeID: parse_index(&line, 1),
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
