//! Module that deals with parsing and storing metadata about stages.

/// Contains constant/static values to be used by the rest of the module.
pub mod consts {
    use regex::{Regex, RegexBuilder};
    use std::sync::LazyLock;

    #[derive(Debug, PartialEq)]
    /// Struct that contains information about each stage type.
    pub struct LegacyStageType {
        /// E.g. `"Stories of Legend"`.
        pub name: &'static str,
        /// Numerical value of stage type.
        pub number: u32,
        /// E.g. `"N"` for Stories of Legend.
        ///
        /// EX stages' map files are of the form `"MapStageDataRE"`, whereas
        /// their stage files are of the form `"stageEX"`, so their `code` is
        /// `"RE|EX"`.
        pub code: &'static str,
        /// Are files of the type `stageR{code}` or are they of the type
        /// `stage{code}`?
        pub has_r_prefix: bool,
        /// Enum variant of the stage type.
        pub type_enum: LegacyStageVariant,
    }

    const fn initialise_stage_type(
        name: &'static str,
        number: u32,
        code: &'static str,
        has_r_prefix: bool,
        type_enum: LegacyStageVariant,
    ) -> LegacyStageType {
        LegacyStageType {
            name,
            number,
            code,
            has_r_prefix,
            type_enum,
        }
    }

    #[derive(Debug)]
    /// Maps a [Regex] to a code from [STAGE_TYPES].
    pub struct LegacyStageTypeMap {
        /// Regex matching any valid pattern for the stage type.
        pub matcher: Regex,
        /// Code as in [STAGE_TYPES].
        pub stage_type: LegacyStageType,
    }

    fn initialise_type_map(
        pattern: &'static str,
        stage_type: LegacyStageVariant,
    ) -> LegacyStageTypeMap {
        let re = format!("^({pattern})$");
        let matcher = RegexBuilder::new(&re)
            .case_insensitive(true)
            .build()
            .unwrap();
        LegacyStageTypeMap {
            matcher,
            stage_type: get_stage_type_code(stage_type),
        }
    }

    /// Get the [StageType] that `stage_type` corresponds to from
    /// [STAGE_TYPES].
    fn get_stage_type_code(stage_type: LegacyStageVariant) -> LegacyStageType {
        for code in LEGACY_STAGE_TYPES {
            if stage_type == code.type_enum {
                return code;
            }
        }

        unreachable!("{stage_type:?} is an invalid stage type enum!");
    }

    /// Get [StageType] that `selector_type` refers to.
    pub fn get_selector_type(selector_type: &str) -> Option<&LegacyStageType> {
        for selector_map in STAGE_TYPE_MAP.iter() {
            if selector_map.matcher.is_match(selector_type) {
                return Some(&selector_map.stage_type);
            }
        }

        None
    }

    /// All possible types of stage.
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum LegacyStageVariant {
        SoL,
        Event,
        Collab,
        MainChapters,
        Extra,
        Dojo,
        RankingDojo,
        Tower,
        Challenge,

        UL,
        Catamin,
        // EocOutbreak,
        // ItfOutbreak,
        // CotcOutbreak,
        Outbreaks,
        Filibuster,
        Gauntlet,
        Enigma,
        CollabGauntlet,

        AkuRealms,
        Behemoth,
        Labyrinth,
        ZL,
        Colosseum,
        Championships,
    }
    use LegacyStageVariant as T;

    #[rustfmt::skip]
    #[allow(clippy::zero_prefixed_literal)]
    /// Collection of [StageTypes][StageType] covering all chapters in the game.
    pub const LEGACY_STAGE_TYPES: [LegacyStageType; 22] = [
        initialise_stage_type("Stories of Legend",            000, "N",     true,  T::SoL),
        initialise_stage_type("Event Stages",                 001, "S",     true,  T::Event),
        initialise_stage_type("Collaboration Stages",         002, "C",     true,  T::Collab),
        initialise_stage_type("Main Chapters",                003, "main",  false, T::MainChapters),
        initialise_stage_type("Extra Stages",                 004, "RE|EX", false, T::Extra),
        initialise_stage_type("Catclaw Dojo",                 006, "T",     true,  T::Dojo),
        initialise_stage_type("Towers",                       007, "V",     true,  T::Tower),
        initialise_stage_type("Ranking Dojo",                 011, "R",     true,  T::RankingDojo),
        initialise_stage_type("Challenge Battle",             012, "M",     true,  T::Challenge),

        initialise_stage_type("Uncanny Legends",              013, "NA",    true,  T::UL),
        initialise_stage_type("Catamin Stages",               014, "B",     true,  T::Catamin),
        // initialise_stage_type("Empire of Cats Outbreaks",     020, "",      false, T::EocOutbreak),
        // initialise_stage_type("Into the Future Outbreaks",    021, "",      false, T::ItfOutbreak),
        // initialise_stage_type("Cats of the Cosmos Outbreaks", 022, "",      false, T::CotcOutbreak),
        initialise_stage_type("Outbreaks",                    999, "main",  false, T::Outbreaks),

        initialise_stage_type("Filibuster Invasion",          023, "",      false, T::Filibuster),
        initialise_stage_type("Gauntlets",                    024, "A",     true,  T::Gauntlet),
        initialise_stage_type("Enigma Stages",                025, "H",     true,  T::Enigma),
        initialise_stage_type("Collab Gauntlets",             027, "CA",    true,  T::CollabGauntlet),

        initialise_stage_type("Aku Realms",                   030, "DM",    false, T::AkuRealms),
        initialise_stage_type("Behemoth Culling",             031, "Q",     true,  T::Behemoth),
        initialise_stage_type("Labyrinth",                    033, "L",     false, T::Labyrinth),
        initialise_stage_type("Zero Legends",                 034, "ND",    true,  T::ZL),
        initialise_stage_type("Colosseum",                    036, "SR",    true,  T::Colosseum),
        initialise_stage_type("Catclaw Championships",        037, "G",     false, T::Championships),
    ];

    #[rustfmt::skip]
    /// Map of regex matchers to code used in [STAGE_TYPES].
    ///
    /// Includes common name for type, type number, type prefix and type prefix
    /// with R if applicable.
    static STAGE_TYPE_MAP: LazyLock<[LegacyStageTypeMap; 22]> = LazyLock::new(|| {[
        initialise_type_map("SoL|0|N|RN",               T::SoL),
        initialise_type_map("Event|Special|1|S|RS",     T::Event),
        initialise_type_map("Collab|2|C|RC",            T::Collab),
        initialise_type_map("EoC|ItF|W|CotC|Space",     T::MainChapters),
        initialise_type_map("Extra|4|RE|EX",            T::Extra),
        initialise_type_map("Dojo|6|T|RT",              T::Dojo),
        initialise_type_map("Tower|7|V|RV",             T::Tower),
        initialise_type_map("Rank|11|R|RR",             T::RankingDojo),
        initialise_type_map("Challenge|12|M|RM",        T::Challenge),

        initialise_type_map("UL|13|NA|RNA",             T::UL),
        initialise_type_map("Catamin|14|B|RB",          T::Catamin),
        initialise_type_map("Z",                        T::Outbreaks),
        initialise_type_map("Filibuster|23",            T::Filibuster),
        // initialise_type_map("LQ|16|D",                  "Why would you want to do Legend Quest"),
        initialise_type_map("Gauntlet|Baron|24|A|RA",   T::Gauntlet),
        initialise_type_map("Enigma|25|H|RH",           T::Enigma),
        initialise_type_map("27|CA|RCA",                T::CollabGauntlet),
        initialise_type_map("Aku|30|DM",                T::AkuRealms),

        initialise_type_map("Behemoth|31|Q|RQ",         T::Behemoth),
        initialise_type_map("Labyrinth|33|L",           T::Labyrinth),
        initialise_type_map("ZL|34|ND|RND",             T::ZL),
        initialise_type_map("Colosseum|36|SR|RSR",      T::Colosseum),
        initialise_type_map("Championships|37|G",       T::Championships),
    ]});
    // There should probably be something that prints off these strings for users.
}
use consts::{get_selector_type, LegacyStageType, LegacyStageVariant};
use regex::Regex;
use std::sync::LazyLock;

/// Struct to contain [FILE_PATTERNS].
struct FilePatterns {
    /// Captures the stage number (e.g. `"40"` in `"stage40.csv"`).
    eoc: Regex,
    /// Main chapters that aren't EoC. Captures prefix (e.g. `"W"` in
    /// `"stageW04_33.csv"`).
    other_main: Regex,
    /// Every chapter that isn't EoC. Captures prefix, map number and stage
    /// number (e.g. `["RND", "106", "702"]` in `"stageRND106_702.csv"`).
    default: Regex,
}

/// Static container for file-related regexes.
static FILE_PATTERNS: LazyLock<FilePatterns> = LazyLock::new(|| FilePatterns {
    eoc: Regex::new(r"^stage(\d{2})\.csv$").unwrap(),
    other_main: Regex::new(r"^stage(W|Space|DM|Z)\d\d.*\.csv$").unwrap(),
    default: Regex::new(r"^stage([\D]*)([\d]*)_([\d]*)\.csv$").unwrap(),
});

// -----------------------------------------------------------------------------

// Temporary implementation for refactoring.

use crate::meta::stage::{
    stage_id::StageID,
    stage_types::{
        parse::{
            parse_stage::{
                parse_general_stage_id, parse_stage_file, parse_stage_ref, parse_stage_selector,
            },
            StageTypeParseError,
        },
        transform::transform_stage::stage_data_file,
    },
};

impl From<&StageID> for LegacyStageMeta {
    fn from(value: &StageID) -> Self {
        LegacyStageMeta::from_file(&stage_data_file(value)).unwrap()
    }
}
impl From<StageID> for LegacyStageMeta {
    fn from(value: StageID) -> Self {
        (&value).into()
    }
}

impl From<&LegacyStageMeta> for StageID {
    fn from(value: &LegacyStageMeta) -> Self {
        parse_stage_file(&value.stage_file_name).unwrap()
    }
}
impl From<LegacyStageMeta> for StageID {
    fn from(value: LegacyStageMeta) -> Self {
        (&value).into()
    }
}

// -----------------------------------------------------------------------------

// TODO split into type, map and stage
#[derive(Debug, PartialEq)]
// #[deprecated]
/// Contains metadata about a given stage.
pub struct LegacyStageMeta {
    /// Long-form name of the stage type.
    pub type_name: &'static str,
    /// [STAGE_TYPES]: consts::STAGE_TYPES
    /// Short-form name of the stage type. All codes are given in [STAGE_TYPES].
    pub type_code: &'static str,
    /// Numerical value of the [StageType].
    pub type_num: u32,
    /// Enum variant of stage type.
    pub type_enum: LegacyStageVariant,
    /// Map number of the stage.
    pub map_num: u32,
    /// Stage number of the stage.
    pub stage_num: u32,

    /// DataLocal file that contains information about the map the stage is in.
    map_file_name: String,
    /// DataLocal file that contains information about the stage.
    stage_file_name: String,
}

impl LegacyStageMeta {
    /// Stage's map data file name.
    pub fn map_file_name(&self) -> &str {
        &self.map_file_name
    }

    /// Stage's data file name.
    pub fn stage_file_name(&self) -> &str {
        &self.stage_file_name
    }
}

#[derive(Debug, PartialEq)]
/// Denotes an error when parsing [StageMeta].
pub enum StageMetaParseError {
    /// Not the correct function to use.
    Rejected,
    /// Either selector doesn't exist or numbers are not given.
    Invalid,
}

impl LegacyStageMeta {
    /// Catch-all method for parsing a selector.
    pub fn new(selector: &str) -> Option<LegacyStageMeta> {
        parse_general_stage_id(selector).map(Into::into)
    }

    fn is_main_chaps(m: LegacyStageVariant) -> bool {
        use LegacyStageVariant as T;
        matches!(
            m,
            T::MainChapters | T::Outbreaks | T::Filibuster | T::AkuRealms
        )
    }

    /// Parse space-delimited selector into [StageMeta] object.
    /// ```
    /// # use rust_wiki::data::stage::raw::stage_metadata::{StageMeta, consts::StageTypeEnum::SoL};
    /// let selector = "N 0 0";
    /// assert_eq!(StageMeta::from_selector(selector).unwrap(), StageMeta { type_name: "Stories of Legend", type_code: "N", type_num: 0, type_enum: SoL, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// ```
    pub fn from_selector(selector: &str) -> Result<LegacyStageMeta, StageTypeParseError> {
        parse_stage_selector(selector).map(Into::into)
    }

    /// Parse file name into [StageMeta] object.
    /// ```
    /// # use rust_wiki::data::stage::raw::stage_metadata::StageMeta;
    /// let file_name = "stageRN000_00.csv";
    /// assert_eq!(file_name, StageMeta::from_file(file_name).unwrap().stage_file_name);
    /// ```
    pub fn from_file(file_name: &str) -> Result<LegacyStageMeta, StageMetaParseError> {
        if file_name == "stageSpace09_Invasion_00.csv" {
            Self::from_selector_main("Filibuster", &[])
        } else if FILE_PATTERNS.eoc.is_match(file_name) {
            let chap_num = &FILE_PATTERNS.eoc.replace(file_name, "$1");
            Self::from_selector_main("eoc", &[chap_num.parse().unwrap()])
        } else if FILE_PATTERNS.other_main.is_match(file_name) {
            Self::from_file_other_main(file_name)
        } else if file_name.contains('_') {
            let caps = FILE_PATTERNS.default.captures(file_name).unwrap();
            let map_num: u32 = caps[2].parse::<u32>().unwrap();
            let stage_num: u32 = caps[3].parse::<u32>().unwrap();
            Self::from_split(&caps[1], map_num, stage_num)
        } else {
            Err(StageMetaParseError::Rejected)
        }
    }

    /// Parse file name if is main chapters but not eoc
    fn from_file_other_main(file_name: &str) -> Result<LegacyStageMeta, StageMetaParseError> {
        let caps = FILE_PATTERNS.default.captures(file_name).unwrap();
        let mut chap_num = caps[2].parse::<u32>().unwrap();
        if &caps[1] == "Z" && chap_num <= 3 {
            chap_num += 1;
        }

        let stage_num = caps[3].parse::<u32>().unwrap();
        let selector = match &caps[1] {
            "W" => (chap_num - 3, stage_num),
            "Space" => (chap_num - 6, stage_num),
            "DM" => (stage_num, stage_num),
            // sort of a workaround so this compiles
            "Z" => (chap_num, stage_num),
            _ => unreachable!(),
        };

        Self::from_selector_main(&caps[1], &[selector.0, selector.1])
    }

    /// Parse battle-cats.db reference into [StageMeta] object.
    ///
    /// `selector` can either be the full reference (with or without a leading
    /// `*`) or just the stage part.
    /// ```
    /// # use rust_wiki::data::stage::raw::stage_metadata::{StageMeta, consts::StageTypeEnum::SoL};
    /// let reference = "*https://battlecats-db.com/stage/s00000-01.html";
    /// assert_eq!(StageMeta::from_ref(reference).unwrap(), StageMeta { type_name: "Stories of Legend", type_code: "N", type_num: 0, type_enum: SoL, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// assert_eq!(StageMeta::from_ref(reference).unwrap(), StageMeta::from_ref("s00000-01").unwrap());
    /// ```
    pub fn from_ref(selector: &str) -> Result<LegacyStageMeta, StageTypeParseError> {
        parse_stage_ref(selector).map(Into::into)
    }

    /// Get meta from numbers.
    pub fn from_numbers(
        stage_type: u32,
        map_num: u32,
        stage_num: u32,
    ) -> Result<LegacyStageMeta, StageMetaParseError> {
        Self::from_split(&stage_type.to_string(), map_num, stage_num)
        // could potentially figure out stage type and use split_parsed
    }

    /// Get [StageMeta] from a selector split into variables.
    /// ```
    /// # use rust_wiki::data::stage::raw::stage_metadata::{StageMeta, consts::StageTypeEnum::SoL};
    /// let st = StageMeta::from_split("SoL", 0, 0);
    /// assert_eq!(st.unwrap(), StageMeta { type_name: "Stories of Legend", type_code: "N", type_num: 0, type_enum: SoL, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// ```
    pub fn from_split(
        type_str: &str,
        map_num: u32,
        stage_num: u32,
    ) -> Result<LegacyStageMeta, StageMetaParseError> {
        let Some(stage_type) = get_selector_type(type_str) else {
            return Err(StageMetaParseError::Invalid);
        };

        if Self::is_main_chaps(stage_type.type_enum) {
            Err(StageMetaParseError::Rejected)
        } else {
            Ok(Self::from_split_parsed(stage_type, map_num, stage_num))
        }
    }

    /// [STAGE_TYPES]: consts::STAGE_TYPES
    /// [from_split][StageMeta::from_split] but with `stage_type` being a code
    /// from [STAGE_TYPES].
    fn from_split_parsed(
        stage_type: &LegacyStageType,
        map_num: u32,
        stage_num: u32,
    ) -> LegacyStageMeta {
        let type_name = stage_type.name;
        let type_num = stage_type.number;
        let type_enum = stage_type.type_enum;

        let type_code;
        let map_file_name;
        let stage_file_name;
        if stage_type.code.contains('|') {
            // If more than RE|EX is needed this could completely break
            let map = &stage_type.code[..2];
            let stage = &stage_type.code[3..];
            type_code = stage;
            map_file_name = format!("MapStageData{map}_{map_num:03}.csv");
            stage_file_name = format!("stage{stage}{map_num:03}_{stage_num:02}.csv");
        } else {
            let stage_prefix = match stage_type.has_r_prefix {
                true => "R",
                false => "",
            };
            let code = stage_type.code;

            type_code = code;
            map_file_name = format!("MapStageData{code}_{map_num:03}.csv");
            stage_file_name = format!("stage{stage_prefix}{code}{map_num:03}_{stage_num:02}.csv");
        }
        // let type_code = code.code

        LegacyStageMeta {
            type_name,
            type_code,
            type_num,
            type_enum,
            map_num,
            stage_num,
            map_file_name,
            stage_file_name,
        }
    }

    /// Formats:
    /// - EoC: `("eoc", [0])` = Korea
    /// - ItF/W: `("itf", [1, 0])` = Japan Ch. 1
    /// - CotC/Space: `("cotc", [1, 0])` = Earth Ch. 1
    /// - Aku/DM: `("aku", [0])` = Korea
    /// - Filibuster: `("filibuster", [])`
    /// - Z: `("z", [1, 0])` = Korea
    pub fn from_selector_main(
        selector: &str,
        nums: &[u32],
    ) -> Result<LegacyStageMeta, StageMetaParseError> {
        let Some(code) = get_selector_type(selector) else {
            return Err(StageMetaParseError::Invalid);
        };

        let type_name = code.name;
        let type_code = code.code;
        let mut type_num = code.number;
        let type_enum = code.type_enum;

        let (map_num, stage_num, map_file_name, stage_file_name) =
            match selector.to_lowercase().as_str() {
                "eoc" => {
                    let stage_num: u32 = nums[0];
                    (
                        0_u32,
                        stage_num,
                        "stageNormal0.csv".to_string(),
                        format!("stage{stage_num:02}.csv"),
                    )
                }
                "itf" | "w" => {
                    let map_num: u32 = nums[0] + 2;
                    assert!((3..=5).contains(&map_num));
                    let stage_num: u32 = nums[1];

                    let map_file = format!("stageNormal1_{}.csv", map_num - 3);
                    let stage_file = format!("stageW{:02}_{stage_num:02}.csv", map_num + 1);

                    (map_num, stage_num, map_file, stage_file)
                }
                "cotc" | "space" => {
                    let map_num: u32 = nums[0] + 5;
                    assert!((6..=8).contains(&map_num));
                    let stage_num: u32 = nums[1];

                    let map_file = format!("stageNormal2_{}.csv", map_num - 6);
                    let stage_file = format!("stageSpace{:02}_{stage_num:02}.csv", map_num + 1);

                    (map_num, stage_num, map_file, stage_file)
                }
                "aku" | "dm" | "30" => {
                    let stage_num: u32 = nums[0];
                    (
                        0_u32,
                        stage_num,
                        "MapStageDataDM_000.csv".to_string(),
                        format!("stageDM000_{stage_num:02}.csv"),
                    )
                }
                "filibuster" => (
                    0_u32,
                    0_u32,
                    "stageNormal2_2_Invasion.csv".to_string(),
                    "stageSpace09_Invasion_00.csv".to_string(),
                ),
                "z" => {
                    let mut chap_num: u32 = nums[0];
                    assert!((1..=9).contains(&chap_num));

                    type_num = 20 + ((chap_num - 1) / 3);
                    let map_num = (chap_num - 1) % 3;
                    let mut stage_num = nums[1];

                    let map_file = format!(
                        "stageNormal{}_{}_Z.csv",
                        (chap_num - 1) / 3,
                        (chap_num - 1) % 3
                    );

                    if chap_num <= 3 {
                        chap_num -= 1;
                    }
                    let stage_file = format!("stageZ{chap_num:02}_{stage_num:02}.csv");

                    if type_num == 20 && stage_num > 47 {
                        stage_num = 47;
                    }
                    // makes other calculations easier

                    (map_num, stage_num, map_file, stage_file)
                }
                _ => return Err(StageMetaParseError::Invalid),
            };

        Ok(LegacyStageMeta {
            type_name,
            type_code,
            type_num,
            type_enum,
            map_num,
            stage_num,
            map_file_name,
            stage_file_name,
        })
    }
}
