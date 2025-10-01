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

fn parse_talents_error(e: &csv::Error, result: &ByteRecord) -> impl Debug {
    let index = match e.kind() {
        csv::ErrorKind::Deserialize { pos: _, err } => err.field().unwrap(),
        _ => unimplemented!(),
    };

    String::from_utf8(result[index as usize].into()).unwrap()
}

fn get_talents_file(path: &Path) -> Vec<Talents> {
    let reader = BufReader::new(File::open(path.join("resLocal/SkillAcquisition.csv")).unwrap());

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            println!("{line:?}");
            Talents {
                fixed: Default::default(),
                groups: Default::default(),
            }
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
