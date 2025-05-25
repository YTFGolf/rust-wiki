//! Module that deals with getting information about stages.
use super::stage_option::StageOptionCSV;
use crate::game_data::{
    map::{
        cached::{map_option::MapOptionCSV, special_rules::SpecialRule},
        raw::{csv_types::StageDataCSV, map_data::GameMapData},
    },
    meta::stage::{
        stage_id::StageID,
        stage_types::{
            parse::{parse_stage::parse_stage_file, parse_types::StageTypeParseError},
            transform::transform_stage::stage_data_file,
        },
    },
    version::Version,
};
use csv::{ByteRecord, StringRecord};
use csv_types::{HeaderCSV, Line2CSV, RawCSVData, StageEnemyCSV};
use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

/// Types to deserialise csv files.
pub mod csv_types {
    // TODO split this up
    #[derive(Debug, serde::Deserialize)]
    /// Data stored in the header of the csv file (minus most Main Chapters).
    pub struct HeaderCSV {
        /// ID of base used.
        pub base_id: i32,
        /// Is no continues? Boolean value.
        pub no_cont: u8,
        /// % chance of continuation.
        pub cont_chance: u32,
        /// `map_num` of continuation stages.
        pub cont_map_id: u32,
        /// Minimum `stage_num` of any continuation stage.
        pub cont_stage_id_min: u32,
        /// Maximum `stage_num` of any continuation stage.
        pub cont_stage_id_max: u32,
    }

    #[derive(Debug, serde::Deserialize)]
    /// Data stored in line 2 of the csv file (line 1 for most Main Chapter
    /// stages).
    pub struct Line2CSV {
        /// Stage width.
        pub width: u32,
        /// Base HP (ignore this if `animbase_id` is not 0).
        pub base_hp: u32,
        _生産最低f: u32,
        _生産最高f: u32,
        /// ID of stage background.
        pub background_id: u32,
        /// Max enemies in stage.
        pub max_enemies: u32,
        /// ID of animated base (if 0 then no base).
        pub anim_base_id: u32,
        /// Time limit (is this only used in Dojo stages?).
        pub time_limit: u32,
        /// Do you have the green barrier thing (boolean value).
        pub indestructible: u8,
        _unknown_3: Option<u32>,
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    /// CSV data for enemies. See [Stage Structure
    /// Page/Battlegrounds](https://battlecats.miraheze.org/wiki/The_Battle_Cats_Wiki:Stage_Structure_Page/Battlegrounds)
    /// for more complete documentation.
    pub struct StageEnemyCSV {
        /// battle-cats db id (i.e. Doge is 2).
        pub num: u32,
        /// Amount of enemy that spawns (0 = infinite).
        pub amt: u32,
        /// Start frame of enemies / 2. Ignored (unless `is_spawn_delay` is
        /// true) for enemies that spawn after base hit.
        pub start_frame: u32,
        /// Min respawn frame of enemies / 2.
        pub respawn_frame_min: u32,
        /// Max respawn frame of enemies / 2.
        pub respawn_frame_max: u32,
        /// At what percentage does the enemy spawn (absolute value for Dojo).
        pub base_hp: u32,
        /// Minimum layer.
        pub layer_min: u32,
        /// Maximum layer.
        pub layer_max: u32,
        /// 0 = none, 1 = boss, 2 = with screen shake.
        pub boss_type: u32,
        /// Enemy magnification.
        pub magnification: Option<u32>,

        #[serde(default)]
        _unknown_1: Option<u32>,
        #[serde(default)]
        /// If not 0 then enemy has different hp and ap mags. `magnification`
        /// should be taken as hp mag and this is ap mag.
        pub attack_magnification: Option<u32>,
        #[serde(default)]
        /// If base hp is <100 (unsure what the effect is in Dojos) and this is
        /// 1 then `start_frame` is not ignored (boolean value).
        pub is_spawn_delay: Option<u32>,
        #[serde(default)]
        /// How many cats need to die before enemy spawns.
        pub kill_count: Option<u32>,
    }

    /// Raw data from the stage csv file.
    #[derive(Debug)]
    pub struct RawCSVData {
        /// Header row.
        pub header: HeaderCSV,
        /// Line 2.
        pub line2: Line2CSV,
        /// Enemies.
        pub enemies: Vec<StageEnemyCSV>,
    }
}

/// Stores information about a stage.
#[derive(Debug)]
pub struct StageData<'a> {
    /// Stage's metadata.
    pub id: StageID,
    /// Data stored in the stage's CSV file.
    pub stage_csv_data: RawCSVData,

    version: &'a Version,
}

#[derive(Debug)]
/// Error when creating [StageData].
pub enum StageDataError {
    /// Error opening file given.
    IOError(io::Error),
    /// Selector doesn't work.
    InvalidSelector(StageTypeParseError),
}

#[derive(Debug)]
/// Type of error encountered on certain line.
pub enum CSVParseErrorKind {
    /// CSV file doesn't have enough lines.
    NotEnoughLines,
    /// Error when converting to ByteRecord.
    ByteRecordError(csv::Error),
    /// Error when deserialising ByteRecord.
    DeserialiseError(csv::Error),
}
type CSVParseErrorLine = (CSVParseErrorKind, u8);

#[derive(Debug)]
/// Error when parsing stage data CSV files.
pub struct CSVParseError {
    kind: CSVParseErrorKind,
    file_name: String,
    line: u8,
}

impl<'a> StageData<'_> {
    /// Create new StageData object from file name.
    pub fn from_file_name(
        selector: &str,
        version: &'a Version,
    ) -> Result<StageData<'a>, StageDataError> {
        match parse_stage_file(selector) {
            Ok(id) => Self::from_id(id, version),
            Err(e) => Err(StageDataError::InvalidSelector(e)),
        }
    }

    /// Get stage data from [`StageID`].
    pub fn from_id(id: StageID, version: &'a Version) -> Result<StageData<'a>, StageDataError> {
        let stage_file = PathBuf::from("DataLocal").join(stage_data_file(&id));
        let reader = BufReader::new(
            File::open(version.get_file_path(&stage_file)).map_err(StageDataError::IOError)?,
        );

        let stage_file_reader = reader;
        let stage_csv_data = Self::read_stage_csv(stage_file_reader).unwrap();

        Ok(StageData {
            id,
            stage_csv_data,
            version,
        })
    }

    /// Read a stage's csv file and obtain the data from it.
    pub fn read_stage_csv<R: std::io::Read>(reader: R) -> Result<RawCSVData, CSVParseErrorLine> {
        // TODO really needs proper error handling
        // type E = CSVParseErrorKind;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(reader);

        let mut records = rdr.byte_records();
        let (header, line2) = read_header_lines(&mut records)?;

        let mut enemies = vec![];
        for result in rdr.records() {
            let record = result.unwrap();
            if record[0].contains('/') {
                continue;
            }
            let Some(enemy) = deserialise_single_enemy(record) else {
                continue;
            };

            if enemy.num == 0 {
                break;
            }
            enemies.push(enemy);
        }

        Ok(RawCSVData {
            header,
            line2,
            enemies,
        })
    }

    /// Get MapStageData data if it exists.
    pub fn get_map_stage_data(&self) -> Option<StageDataCSV> {
        GameMapData::get_stage_data(&self.id, self.version)
    }

    /// Get Map_option data if it exists.
    pub fn get_map_option_data(&self) -> Option<MapOptionCSV> {
        GameMapData::get_map_option_data(self.id.map(), self.version)
    }

    /// Get Stage_option data if it exists.
    pub fn get_stage_option_data(&self) -> Option<Vec<&StageOptionCSV>> {
        GameMapData::stage_stage_option_data(&self.id, self.version)
    }

    /// Get Map_option data if it exists.
    pub fn get_ex_option_data(&self) -> Option<u32> {
        GameMapData::get_ex_option_data(self.id.map(), self.version)
    }

    /// Get SpecialRulesMap data if it exists.
    pub fn get_special_rules_data(&self) -> Option<&SpecialRule> {
        GameMapData::get_special_rules_data(self.id.map(), self.version)
    }

    /// Get the data object's version.
    pub fn version(&self) -> &Version {
        self.version
    }
}

fn read_header_lines<R: std::io::Read>(
    records: &mut csv::ByteRecordsIter<'_, R>,
) -> Result<(HeaderCSV, Line2CSV), CSVParseErrorLine> {
    type E = CSVParseErrorKind;
    let mut line_1_or_2 = records
        .next()
        .ok_or((E::NotEnoughLines, 1))?
        .map_err(|e| (E::ByteRecordError(e), 1))?;
    // some stages (e.g. EoC) don't have the header line, and in those stages
    // the header line is the struct referred to as `Line2`.

    let is_ignorable = |entry: &[u8]| entry.is_empty() || entry.contains(&b'/');
    let has_header =
        line_1_or_2.len() <= 7 || is_ignorable(&line_1_or_2[6]) || is_ignorable(&line_1_or_2[7]);
    // does this specific file have the proper header? If not then the first
    // line will be what is called line2
    let csv_head: HeaderCSV = if has_header {
        let tmp = line_1_or_2;
        let head = tmp
            .deserialize(None)
            .map_err(|e| (E::DeserialiseError(e), 1))?;

        line_1_or_2 = records
            .next()
            .ok_or((E::NotEnoughLines, 2))?
            .map_err(|e| (E::ByteRecordError(e), 2))?;
        line_1_or_2 = remove_comment_ind(line_1_or_2, 9);

        head
        // if (cas == -1)
        //     cas = CH_CASTLES[id.id];
    } else {
        line_1_or_2 = remove_comment_ind(line_1_or_2, 6);

        // In EoC
        HeaderCSV {
            base_id: 0,
            no_cont: 0,
            cont_chance: 0,
            cont_map_id: 0,
            cont_stage_id_min: 0,
            cont_stage_id_max: 0,
        }
        // castle = Identifier.parseInt(sm.cast * 1000 + CH_CASTLES[id.id], CastleImg.class);
    };

    let line_2 = line_1_or_2;
    let csv_line_2: Line2CSV = line_2.deserialize(None).unwrap();

    Ok((csv_head, csv_line_2))
}

/// If `record[index]` is a comment, then truncate record.
fn remove_comment_ind(mut record: ByteRecord, index: usize) -> ByteRecord {
    if record[index].contains(&b'/') {
        record.truncate(index);
        record.push_field(b"");
    }

    record
}

fn deserialise_single_enemy(result: StringRecord) -> Option<StageEnemyCSV> {
    let record: StageEnemyCSV = match result.deserialize(None) {
        Ok(r) => r,
        Err(x) => match x.kind() {
            // possibly could return errors instead of an option
            csv::ErrorKind::Deserialize { pos: _, err } => match err.kind() {
                csv::DeserializeErrorKind::ParseInt(parse_int_error) => {
                    match parse_int_error.kind() {
                        std::num::IntErrorKind::Empty => return None,
                        std::num::IntErrorKind::InvalidDigit => {
                            let index: usize = err.field().unwrap().try_into().unwrap();
                            let field = result.get(index).unwrap();

                            if matches!(field, ".") {
                                assert_eq!(index + 1, result.len());
                                let mut r2 = result;
                                r2.truncate(index);
                                return deserialise_single_enemy(r2);
                            }
                            if field.starts_with("//") {
                                let mut r2 = result;
                                r2.truncate(index);
                                return deserialise_single_enemy(r2);
                            }

                            println!("{field:?}");
                            println!("{result:?}");
                            panic!("{err:?}")
                        }
                        // std::num::IntErrorKind::PosOverflow => unimplemented!(),
                        // std::num::IntErrorKind::NegOverflow => unimplemented!(),
                        // std::num::IntErrorKind::Zero => unimplemented!(),
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(), // csv::DeserializeErrorKind::Message(_) => unimplemented!(),
                                       // csv::DeserializeErrorKind::Unsupported(_) => unimplemented!(),
                                       // csv::DeserializeErrorKind::UnexpectedEndOfRow => unimplemented!(),
                                       // csv::DeserializeErrorKind::InvalidUtf8(utf8_error) => unimplemented!(),
                                       // csv::DeserializeErrorKind::ParseBool(parse_bool_error) => unimplemented!(),
                                       // csv::DeserializeErrorKind::ParseFloat(parse_float_error) => unimplemented!(),
            },
            // csv::ErrorKind::Io(error) => unimplemented!(),
            // csv::ErrorKind::Utf8 { pos, err } => unimplemented!(),
            // csv::ErrorKind::UnequalLengths { pos, expected_len, len } => unimplemented!(),
            // csv::ErrorKind::Seek => unimplemented!(),
            // csv::ErrorKind::Serialize(_) => unimplemented!(),
            _ => unimplemented!(),
        },
    };
    Some(record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        TEST_CONFIG,
        game_data::{
            map::raw::csv_types::{ScoreRewardsCSV, TreasureCSV, TreasureType},
            meta::stage::variant::StageVariantID as T,
        },
    };
    use std::{io::Cursor, vec};

    #[test]
    fn test_whitespace() {
        let aven_jazz_cafe_sirrel = Cursor::new("414,5 ,1430,10,15,75,6,9,0,150,");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(aven_jazz_cafe_sirrel);
        let line = rdr.records().next().unwrap().unwrap();
        let enemy = line.deserialize::<StageEnemyCSV>(None);
        assert!(enemy.is_ok());
        assert_eq!(enemy.ok(), deserialise_single_enemy(line));
    }

    #[test]
    fn test_invalid_char() {
        let tragedy_in_red_those_guys = Cursor::new("4,0,20,50,150,100,0,8,0,2400,.");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(tragedy_in_red_those_guys);
        let mut line = rdr.records().next().unwrap().unwrap();
        line.truncate(10);
        let enemy = line.deserialize::<StageEnemyCSV>(None);
        assert!(enemy.is_ok());
        assert_eq!(enemy.ok(), deserialise_single_enemy(line));
    }

    #[test]
    fn test_simple_parse() {
        let those_guys = Cursor::new("4,0,20,50,150,100,0,8,0,2400");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(those_guys);
        let line = rdr.records().next().unwrap().unwrap();
        assert_eq!(
            deserialise_single_enemy(line)
                .unwrap()
                .magnification
                .unwrap(),
            2400
        );
    }

    #[test]
    #[should_panic = "left: 5\n right: 10"]
    // . is not at end of line so panics
    fn test_parse_error_panics() {
        let unparsable = Cursor::new("4,0,20,50,.,100,0,8,0,2400");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(unparsable);
        let line = rdr.records().next().unwrap().unwrap();
        deserialise_single_enemy(line);
    }

    #[test]
    fn test_invalid_line_fails() {
        let invalid = Cursor::new(",,,,,,,,,,");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(invalid);
        let line = rdr.records().next().unwrap().unwrap();
        assert_eq!(deserialise_single_enemy(line), None);
    }

    #[test]
    fn test_basic() {
        let earthshaker = StageData::from_id(
            StageID::from_components(T::SoL, 0, 0),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        let doge = &earthshaker.stage_csv_data.enemies[0];
        assert_eq!(doge.amt, 50);
        assert_eq!(doge.respawn_frame_min, 30);
        assert_eq!(doge.base_hp, 100);
        assert_eq!(doge.magnification, Some(200));
    }

    #[test]
    fn test_once_then_unlimited_treasure() {
        let whole_new_world = StageData::from_id(
            StageID::from_components(T::ZL, 0, 0),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        let mdata = whole_new_world.get_map_stage_data().unwrap();
        assert_eq!(
            mdata.treasure_drop,
            vec![
                TreasureCSV {
                    item_chance: 1,
                    item_id: 6,
                    item_amt: 10_000_000
                },
                TreasureCSV {
                    item_chance: 1,
                    item_id: 1,
                    item_amt: 1
                }
            ]
        );
        assert_eq!(mdata.treasure_type, TreasureType::OnceThenUnlimited);
    }

    #[test]
    fn test_guaranteed_once_treasure() {
        let it_floor_20 = StageData::from_id(
            StageID::from_components(T::Tower, 6, 19),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        let mdata = it_floor_20.get_map_stage_data().unwrap();
        assert_eq!(
            mdata.treasure_drop,
            vec![
                TreasureCSV {
                    item_chance: 33,
                    item_id: 41,
                    item_amt: 1
                },
                TreasureCSV {
                    item_chance: 34,
                    item_id: 160,
                    item_amt: 1
                },
                TreasureCSV {
                    item_chance: 33,
                    item_id: 43,
                    item_amt: 1
                }
            ]
        );
        assert_eq!(mdata.treasure_type, TreasureType::GuaranteedOnce);
    }

    #[test]
    fn test_killcount() {
        let dja10 = StageData::from_id(
            StageID::from_components(T::Behemoth, 0, 9),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        assert_eq!(dja10.stage_csv_data.enemies[5].kill_count, Some(60));
        assert_eq!(dja10.stage_csv_data.enemies[6].kill_count, Some(120));
    }

    #[test]
    fn test_equal_chance() {
        let spring_popstar = StageData::from_id(
            StageID::from_components(T::Collab, 128, 0),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        let mdata = spring_popstar.get_map_stage_data().unwrap();
        assert_eq!(
            mdata.treasure_drop,
            vec![
                TreasureCSV {
                    item_chance: 1,
                    item_id: 0,
                    item_amt: 1
                },
                TreasureCSV {
                    item_chance: 1,
                    item_id: 3,
                    item_amt: 1
                },
                TreasureCSV {
                    item_chance: 1,
                    item_id: 4,
                    item_amt: 1
                },
                TreasureCSV {
                    item_chance: 1,
                    item_id: 5,
                    item_amt: 1
                }
            ]
        );
        assert_eq!(mdata.treasure_type, TreasureType::GuaranteedUnlimited);
    }

    #[test]
    fn test_continue_multiple() {
        let proving_grounds = StageData::from_id(
            StageID::from_components(T::Event, 250, 0),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        assert_eq!(proving_grounds.stage_csv_data.header.no_cont, 1);
        assert_eq!(proving_grounds.stage_csv_data.header.cont_chance, 100);
        assert_eq!(proving_grounds.stage_csv_data.header.cont_stage_id_min, 0);
        assert_eq!(proving_grounds.stage_csv_data.header.cont_stage_id_max, 1);
        assert_eq!(proving_grounds.stage_csv_data.header.cont_map_id, 27);
    }

    #[test]
    fn test_once_then_unlimited_treasure_2() {
        let taste_of_success = StageData::from_id(
            StageID::from_components(T::Event, 155, 0),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        let mdata = taste_of_success.get_map_stage_data().unwrap();
        assert_eq!(
            mdata.treasure_drop,
            vec![
                TreasureCSV {
                    item_chance: 10,
                    item_id: 6,
                    item_amt: 2_030_000
                },
                TreasureCSV {
                    item_chance: 30,
                    item_id: 6,
                    item_amt: 1_020_000
                },
                TreasureCSV {
                    item_chance: 100,
                    item_id: 6,
                    item_amt: 510_000
                }
            ]
        );
        assert_eq!(mdata.treasure_type, TreasureType::OnceThenUnlimited);
    }

    #[test]
    fn test_all_unlimited() {
        let jubilee_green_night = StageData::from_id(
            StageID::from_components(T::Extra, 0, 0),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        let mdata = jubilee_green_night.get_map_stage_data().unwrap();
        assert_eq!(
            mdata.treasure_drop,
            vec![
                TreasureCSV {
                    item_chance: 70,
                    item_id: 40,
                    item_amt: 1
                },
                TreasureCSV {
                    item_chance: 85,
                    item_id: 38,
                    item_amt: 1
                },
                TreasureCSV {
                    item_chance: 100,
                    item_id: 33,
                    item_amt: 1
                }
            ]
        );
        assert_eq!(mdata.treasure_type, TreasureType::AllUnlimited);
    }

    #[test]
    fn test_timed_scores() {
        let germany_itf_1 = StageData::from_id(
            StageID::from_components(T::MainChapters, 3, 8),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
        let mdata = germany_itf_1.get_map_stage_data().unwrap();
        assert_eq!(
            mdata.score_rewards,
            vec![
                ScoreRewardsCSV {
                    score: 8500,
                    item_id: 13,
                    item_amt: 10
                },
                ScoreRewardsCSV {
                    score: 5000,
                    item_id: 6,
                    item_amt: 45000
                }
            ]
        );
    }

    #[test]
    fn test_massive_header() {
        let _ = StageData::from_id(
            StageID::from_components(T::Collab, 224, 2),
            TEST_CONFIG.version.current_version(),
        )
        .unwrap();
    }
}
