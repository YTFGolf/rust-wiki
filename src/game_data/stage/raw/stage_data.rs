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
use std::{fmt::Display, fs::File, io::BufReader, path::PathBuf};

/// Types to deserialise csv files.
pub mod csv_types {
    // TODO split this up
    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
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

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
    /// Data stored in line 2 of the csv file (line 1 for most Main Chapter
    /// stages).
    pub struct Line2CSV {
        /// Stage width.
        pub width: u32,
        /// Base HP (ignore this if `animbase_id` is not 0).
        pub base_hp: u32,
        pub(super) _生産最低f: u32,
        pub(super) _生産最高f: u32,
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
        pub(super) _unknown_3: Option<u32>,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
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
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, thiserror::Error)]
/// Error when getting data from file name.
pub enum FromFileError {
    /// File selector doesn't work.
    #[error(transparent)]
    InvalidSelector(#[from] StageTypeParseError),
    /// Error creating data object.
    #[error(transparent)]
    DataParseError(#[from] StageDataError),
}

#[derive(Debug, thiserror::Error)]
/// Error when creating [StageData].
pub enum StageDataError {
    /// Error opening file given.
    #[error("Couldn't open file {file_name}. {source}")]
    FileOpenError {
        /// File that couldn't be opened.
        file_name: String,
        /// What went wrong when trying to open.
        source: std::io::Error,
    },
    /// Error when parsing CSV.
    #[error("couldn't parse CSV")]
    ParseError(#[from] CSVParseError),
}

#[derive(Debug, thiserror::Error)]
/// Type of error encountered on certain line.
pub enum CSVParseErrorKind {
    /// CSV file doesn't have enough lines.
    #[error("tried to get a line that didn't exist")]
    NotEnoughLines,
    /// Error when converting to CSV Record.
    #[error("Couldn't convert to CSV Record. (TODO: fix this message) {0}")]
    CSVRecordError(csv::Error),
    /// Error when deserialising CSV Record.
    #[error(transparent)]
    DeserialiseError(csv::Error),
}
type CSVParseErrorLine = (CSVParseErrorKind, usize);

#[derive(Debug)]
/// Error when parsing stage data CSV files.
pub struct CSVParseError {
    kind: CSVParseErrorKind,
    file_name: String,
    line: usize,
}
impl Display for CSVParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error on line {line} in file {file_name:?}: {kind}",
            line = self.line,
            file_name = self.file_name,
            kind = self.kind
        )
    }
}
impl std::error::Error for CSVParseError {}

impl<'a> StageData<'_> {
    /// Create new StageData object from file name.
    pub fn from_file_name(
        selector: &str,
        version: &'a Version,
    ) -> Result<StageData<'a>, FromFileError> {
        match parse_stage_file(selector) {
            Ok(id) => Self::from_id(id, version).map_err(FromFileError::DataParseError),
            Err(e) => Err(FromFileError::InvalidSelector(e)),
        }
    }

    /// Get stage data from [`StageID`].
    pub fn from_id(id: StageID, version: &'a Version) -> Result<StageData<'a>, StageDataError> {
        let file_name = stage_data_file(&id);
        let path = PathBuf::from("DataLocal").join(&file_name);

        let file = match File::open(version.get_file_path(&path)) {
            Ok(file) => file,
            Err(source) => return Err(StageDataError::FileOpenError { file_name, source }),
            // `map_err` can't be used because it moves `file_name` and compiler
            // can't determine that that's completely safe to do so
        };

        let reader = BufReader::new(file);
        let stage_csv_data =
            Self::read_stage_csv(reader).map_err(|(kind, line)| CSVParseError {
                kind,
                file_name,
                line,
            })?;

        Ok(StageData {
            id,
            stage_csv_data,
            version,
        })
    }

    /// Read a stage's csv file and obtain the data from it.
    pub fn read_stage_csv<R: std::io::Read>(reader: R) -> Result<RawCSVData, CSVParseErrorLine> {
        type E = CSVParseErrorKind;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(reader);

        let mut records = rdr.byte_records();
        let (header, line2) = read_header_lines(&mut records)?;

        let mut enemies = vec![];
        for (i, result) in rdr.records().enumerate() {
            const OFFSET: usize = 2;
            let record = result.map_err(|e| (E::CSVRecordError(e), OFFSET + i))?;

            assert!(!record.is_empty());
            // below would panic anyway without this assert
            if record[0].contains('/') {
                continue;
            }
            // TODO real error handling in the function here
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
        .map_err(|e| (E::CSVRecordError(e), 1))?;
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
            .map_err(|e| (E::CSVRecordError(e), 2))?;
        line_1_or_2 = remove_comment_ind(line_1_or_2, 9);

        head
        // if (cas == -1)
        //     cas = CH_CASTLES[id.id];
    } else {
        line_1_or_2 = remove_comment_ind(line_1_or_2, 6);
        HeaderCSV::default()
        // castle = Identifier.parseInt(sm.cast * 1000 + CH_CASTLES[id.id], CastleImg.class);
    };

    let line_2 = line_1_or_2;
    let csv_line_2: Line2CSV = line_2
        .deserialize(None)
        .map_err(|e| (E::DeserialiseError(e), 2))?;

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

#[allow(clippy::unwrap_used)]
// I really can't be bothered to make this look nice rn
// FIXME this
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
    use std::vec;

    /// Get CSV reader from lines.
    fn get_reader(lines: &str) -> csv::Reader<&[u8]> {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(lines.as_bytes())
    }

    #[test]
    fn test_whitespace() {
        let aven_jazz_cafe_sirrel = "414,5 ,1430,10,15,75,6,9,0,150,";
        let mut rdr = get_reader(aven_jazz_cafe_sirrel);

        let line = rdr.records().next().unwrap().unwrap();
        let enemy = line.deserialize::<StageEnemyCSV>(None);

        assert!(enemy.is_ok());
        assert_eq!(enemy.ok(), deserialise_single_enemy(line));
    }

    #[test]
    fn test_invalid_char() {
        let tragedy_in_red_those_guys = "4,0,20,50,150,100,0,8,0,2400,.";
        let mut rdr = get_reader(tragedy_in_red_those_guys);

        let mut line = rdr.records().next().unwrap().unwrap();
        line.truncate(10);
        let enemy = line.deserialize::<StageEnemyCSV>(None);

        assert!(enemy.is_ok());
        assert_eq!(enemy.ok(), deserialise_single_enemy(line));
    }

    #[test]
    fn test_simple_parse() {
        let those_guys = "4,0,20,50,150,100,0,8,0,2400";
        let mut rdr = get_reader(those_guys);

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
    // really should not panic but idc
    fn test_parse_error_panics() {
        let unparsable = "4,0,20,50,.,100,0,8,0,2400";
        let mut rdr = get_reader(unparsable);

        let line = rdr.records().next().unwrap().unwrap();
        deserialise_single_enemy(line);
    }

    #[test]
    fn test_invalid_line_fails() {
        let invalid = ",,,,,,,,,,";
        let mut rdr = get_reader(invalid);

        let line = rdr.records().next().unwrap().unwrap();
        assert_eq!(deserialise_single_enemy(line), None);
    }

    #[test]
    fn test_file_no_exist() {
        let version = TEST_CONFIG.version.current_version();
        let no_file = StageData::from_file_name("stage84.csv", version).unwrap_err();
        // I could use the actual numbers but idc
        assert!(matches!(
            no_file,
            FromFileError::DataParseError(StageDataError::FileOpenError { .. })
        ));

        assert!(
            no_file
                .to_string()
                .starts_with("Couldn't open file stage84.csv.") // ensure proper error bubbling. Testing IOError is not
                                                                // necessary and may change between platforms.
        );
    }

    #[test]
    fn header_parse() {
        let header = "0,0,0,0,0,0,\n4200,60000,1,60,0,7,0,0,0,0,";
        let data = StageData::read_stage_csv(header.as_bytes()).unwrap();

        const LINE_2: Line2CSV = Line2CSV {
            width: 4_200,
            base_hp: 60_000,
            _生産最低f: 1,
            _生産最高f: 60,
            background_id: 0,
            max_enemies: 7,
            anim_base_id: 0,
            time_limit: 0,
            indestructible: 0,
            _unknown_3: Some(0),
        };
        assert_eq!(
            data,
            RawCSVData {
                header: Default::default(),
                line2: LINE_2,
                enemies: Default::default(),
            }
        );
    }

    #[test]
    fn header_parse_fail() {
        let header = "0,-1,0,0,0,0,\n4200,60000,1,60,0,7,0,0,0,0,";
        let error = StageData::read_stage_csv(header.as_bytes()).unwrap_err();

        assert!(matches!(
            error,
            (CSVParseErrorKind::DeserialiseError(_), 1),
            //
        ));

        assert_eq!(
            error.0.to_string(),
            "CSV deserialize error: record 0 (line: 1, byte: 0): field 1: invalid digit found in string",
            "{}",
            error.0
        );
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
