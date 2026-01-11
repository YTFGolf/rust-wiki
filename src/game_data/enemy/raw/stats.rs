use crate::game_data::{
    cat::raw::unitbuy::parse_unitbuy_error,
    version::{
        Version,
        version_data::{CacheableVersionData, CvdCreateError, CvdResult},
    },
};
use string_error::into_err;

/// Could reasonably either go above 65,535 or be multiplied to go above (e.g.
/// stats).
type Massive = u32;
/// Big number from 0 to 65,535.
type Big = u16;
/// 0-100.
type Percent = u8;
/// 0-256.
type Small = u8;
/// 0 or 1.
type Bool = u8;

#[derive(Debug, serde::Deserialize, Default)]
#[allow(missing_docs)]
pub struct EnemyCSV {
    pub hp: Massive,
    rest: Vec<i32>,
}

#[derive(Debug)]
pub struct TUnitContainer {
    units: Vec<EnemyCSV>,
}
impl CacheableVersionData for TUnitContainer {
    fn create(version: &Version) -> CvdResult<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(version.location().join("DataLocal/t_unit.csv"))
            .map_err(CvdCreateError::throw_from_err)?;

        let records: Result<Vec<EnemyCSV>, CvdCreateError<Self>> = rdr
            .byte_records()
            .map(|record| {
                let result = record.map_err(CvdCreateError::throw_from_err)?;
                let unit: EnemyCSV = match result.deserialize(None) {
                    Ok(u) => u,
                    Err(e) => {
                        let e2 = format!(
                            "Error when parsing record {result:?}: {e}. Item was {item:?}.",
                            item = parse_unitbuy_error(&e, &result)
                        );

                        return Err(CvdCreateError::throw(into_err(e2)));
                    }
                };

                Ok(unit)
            })
            .collect();

        Ok(Self { units: records? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();
        let t_unit = version.get_cached_file::<TUnitContainer>();
        let units = &t_unit.units;

        for unit in units {
            println!("{unit:?}");
            panic!()
        }
        panic!()
    }
}
