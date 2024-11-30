//! Module that deals with parsing and storing metadata about stages.

/// Contains constant/static values to be used by the rest of the module.
pub mod consts {
    use regex::{Regex, RegexBuilder};
    use std::sync::LazyLock;

    #[derive(Debug, PartialEq)]
    /// Struct that contains information about each stage type.
    pub struct StageType {
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
        pub type_enum: StageTypeEnum,
    }

    const fn initialise_stage_type(
        name: &'static str,
        number: u32,
        code: &'static str,
        has_r_prefix: bool,
        type_enum: StageTypeEnum,
    ) -> StageType {
        StageType {
            name,
            number,
            code,
            has_r_prefix,
            type_enum,
        }
    }

    #[derive(Debug)]
    /// Maps a [Regex] to a code from [STAGE_TYPES].
    pub struct StageTypeMap {
        /// Regex matching any valid pattern for the stage type.
        pub matcher: Regex,
        /// Code as in [STAGE_TYPES].
        pub stage_type: StageType,
    }

    fn initialise_type_map(pattern: &'static str, stage_type: StageTypeEnum) -> StageTypeMap {
        let re = format!("^({})$", pattern);
        let matcher = RegexBuilder::new(&re)
            .case_insensitive(true)
            .build()
            .unwrap();
        StageTypeMap {
            matcher,
            stage_type: get_stage_type_code(stage_type),
        }
    }

    /// Get the [StageType] that `stage_type` corresponds to from
    /// [STAGE_TYPES].
    fn get_stage_type_code(stage_type: StageTypeEnum) -> StageType {
        for code in STAGE_TYPES {
            if stage_type == code.type_enum {
                return code;
            }
        }

        unreachable!("{stage_type:?} is an invalid stage type enum!");
    }

    /// Get [StageType] that `selector_type` refers to.
    pub fn get_selector_type(selector_type: &str) -> Option<&StageType> {
        for selector_map in STAGE_TYPE_MAP.iter() {
            if selector_map.matcher.is_match(selector_type) {
                return Some(&selector_map.stage_type);
            }
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_selector_type() {
            assert_eq!(get_selector_type("ITF").unwrap().type_enum, T::MainChapters);
            assert_eq!(get_selector_type("itf").unwrap().type_enum, T::MainChapters);
            assert_eq!(get_selector_type("itf2"), None);
        }

        #[test]
        fn test_get_stage_type_code() {
            assert_eq!(get_stage_type_code(T::MainChapters), STAGE_TYPES[3]);
        }
    }

    /// All possible type of stage.
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum StageTypeEnum {
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
        NewType,
    }
    use StageTypeEnum as T;

    #[rustfmt::skip]
    #[allow(clippy::zero_prefixed_literal)]
    /// Collection of [StageTypes][StageType] covering all chapters in the game.
    pub const STAGE_TYPES: [StageType; 22] = [
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
        initialise_stage_type("Outbreaks",                    003, "main",  false, T::Outbreaks),

        initialise_stage_type("Filibuster Invasion",          023, "",      false, T::Filibuster),
        initialise_stage_type("Gauntlets",                    024, "A",     true,  T::Gauntlet),
        initialise_stage_type("Enigma Stages",                025, "H",     true,  T::Enigma),
        initialise_stage_type("Collab Gauntlets",             027, "CA",    true,  T::CollabGauntlet),

        initialise_stage_type("Aku Realms",                   030, "DM",    false, T::AkuRealms),
        initialise_stage_type("Behemoth Culling",             031, "Q",     true,  T::Behemoth),
        initialise_stage_type("Labyrinth",                    033, "L",     false, T::Labyrinth),
        initialise_stage_type("Zero Legends",                 034, "ND",    true,  T::ZL),
        initialise_stage_type("Colosseum",                    036, "SR",    true,  T::Colosseum),
        initialise_stage_type("NewType",                      037, "G",     false, T::NewType),
    ];

    #[rustfmt::skip]
    /// Map of regex matchers to code used in [STAGE_TYPES].
    ///
    /// Includes common name for type, type number, type prefix and type prefix
    /// with R if applicable.
    ///
    /// ‎
    // Lines above are necessary otherwise rust-analyzer displays stuff as
    // headings
    static STAGE_TYPE_MAP: LazyLock<[StageTypeMap; 22]> = LazyLock::new(|| {[
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
        initialise_type_map("Filibuster|30",            T::Filibuster),
        // initialise_type_map("LQ|16|D",                  "Why would you want to do Legend Quest"),
        initialise_type_map("Gauntlet|Baron|24|A|RA",   T::Gauntlet),
        initialise_type_map("Enigma|25|H|RH",           T::Enigma),
        initialise_type_map("27|CA|RCA",                T::CollabGauntlet),
        initialise_type_map("Aku|30|DM",                T::AkuRealms),

        initialise_type_map("Behemoth|31|Q|RQ",         T::Behemoth),
        initialise_type_map("Labyrinth|33|L",           T::Labyrinth),
        initialise_type_map("ZL|34|ND|RND",             T::ZL),
        initialise_type_map("Colosseum|36|SR|RSR",      T::Colosseum),
        initialise_type_map("37|G",                     T::NewType),
    ]});
    // There should probably be something that prints off these strings for users.
}
use consts::{get_selector_type, StageType, StageTypeEnum};
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
/// Captures `"s00000-01"` in
/// `"*https://battlecats-db.com/stage/s00000-01.html"`.
static DB_REFERENCE_FULL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*?https://battlecats-db.com/stage/(s[\d\-]+).html").unwrap());
/// Captures `["01", "001", "999"]` in `"s01001-999"`.
static DB_REFERENCE_STAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^s(\d{2})(\d{3})\-(\d{2,})$").unwrap());

/// Static container for file-related regexes.
static FILE_PATTERNS: LazyLock<FilePatterns> = LazyLock::new(|| FilePatterns {
    eoc: Regex::new(r"^stage(\d{2})\.csv$").unwrap(),
    other_main: Regex::new(r"^stage(W|Space|DM|Z)\d\d.*\.csv$").unwrap(),
    default: Regex::new(r"^stage([\D]*)([\d]*)_([\d]*)\.csv$").unwrap(),
});

#[derive(Debug, PartialEq)]
/// Contains metadata about a given stage.
pub struct StageMeta {
    /// Long-form name of the stage type.
    pub type_name: &'static str,
    /// Short-form name of the stage type. All codes are given in [STAGE_TYPES].
    pub type_code: &'static str,
    /// Numerical value of the [StageType].
    pub type_num: u32,
    /// Enum variant of stage type.
    pub type_enum: StageTypeEnum,
    /// Map number of the stage.
    pub map_num: u32,
    /// Stage number of the stage.
    pub stage_num: u32,

    /// DataLocal file that contains information about the map the stage is in.
    pub map_file_name: String,
    /// DataLocal file that contains information about the stage.
    pub stage_file_name: String,
}

#[derive(Debug, PartialEq)]
/// Denotes an error when parsing [StageMeta].
pub enum StageMetaParseError {
    /// Not the correct function to use.
    Rejected,
    /// Either selector doesn't exist or numbers are not given.
    Invalid,
}

impl StageMeta {
    /// Catch-all method for parsing a selector.
    pub fn new(selector: &str) -> Option<StageMeta> {
        // TODO optimise by checking selectors before running functions
        // TODO check the type: if rejected go to next function, if invalid
        // then return None.
        if let Ok(st) = Self::from_file(selector) {
            return Some(st);
        };
        if let Ok(st) = Self::from_selector(selector) {
            return Some(st);
        };
        if let Ok(st) = Self::from_ref(selector) {
            return Some(st);
        };

        None
    }

    /// Get `type_num` and `map_num`, normalised for inaccurate main chapter
    /// type numbers (temp function).
    pub fn type_map_num(&self) -> (u32, u32) {
        if true {
            return (self.type_num, self.map_num);
        }

        match self.type_enum {
            StageTypeEnum::MainChapters => (),
            _ => return (self.type_num, self.map_num),
        };

        const OUTBREAK_MAPS: [u32; 8] = [0, 1, 2, 10, 12, 13, 15, 16];
        match self.map_num {
            9 => (3, 0),
            // EoC. Not bothering with chaps 2 and 3
            3_u32..=5_u32 => (3, self.map_num),
            // ItF
            6_u32..=8_u32 => (3, self.map_num),
            // CotC
            0 | 1 | 2 | 10 | 12 | 13 | 15 | 16 => {
                let i = OUTBREAK_MAPS
                    .iter()
                    .position(|x| *x == self.map_num)
                    .unwrap();
                let i = i as u32;
                (20 + i / 3, i % 3)
            }
            // Outbreaks
            11 => (23, 0),
            // Filibuster
            14 => (30, 0),
            // Aku realms
            _ => unreachable!("{self:?}"),
        }
    }

    /// Parse space-delimited selector into [StageMeta] object.
    /// ```
    /// # use rust_wiki::data::stage::raw::stage_metadata::{StageMeta, consts::StageTypeEnum::SoL};
    /// let selector = "N 0 0";
    /// assert_eq!(StageMeta::from_selector(selector).unwrap(), StageMeta { type_name: "Stories of Legend", type_code: "N", type_num: 0, type_enum: SoL, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// ```
    pub fn from_selector(selector: &str) -> Result<StageMeta, StageMetaParseError> {
        let selector: Vec<&str> = selector.split(" ").collect();

        let Some(stage_type) =
            get_selector_type(selector.first().expect("Selector should have content!"))
        else {
            return Err(StageMetaParseError::Invalid);
        };

        use StageTypeEnum as T;
        match stage_type.type_enum {
            T::MainChapters | T::Outbreaks | T::Filibuster | T::AkuRealms => {
                Self::from_selector_main(&selector)
            }
            _ => {
                // let chapter: u32 = stage_type.parse().unwrap();
                let submap: u32 = selector[1].parse().unwrap();
                let stage: u32 = selector[2].parse::<u32>().unwrap();
                Self::from_split_parsed(stage_type, submap, stage)
            }
        }
    }

    /// Parse file name into [StageMeta] object.
    /// ```
    /// # use rust_wiki::data::stage::raw::stage_metadata::StageMeta;
    /// let file_name = "stageRN000_00.csv";
    /// assert_eq!(file_name, StageMeta::from_file(file_name).unwrap().stage_file_name);
    /// ```
    pub fn from_file(file_name: &str) -> Result<StageMeta, StageMetaParseError> {
        if file_name == "stageSpace09_Invasion_00.csv" {
            Self::from_selector_main(&["Filibuster"])
        } else if FILE_PATTERNS.eoc.is_match(file_name) {
            Self::from_selector_main(&["eoc", &FILE_PATTERNS.eoc.replace(file_name, "$1")])
        } else if FILE_PATTERNS.other_main.is_match(file_name) {
            Self::from_file_other_main(file_name)
        } else if file_name.contains("_") {
            let caps = FILE_PATTERNS.default.captures(file_name).unwrap();
            let map_num: u32 = caps[2].parse::<u32>().unwrap();
            let stage_num: u32 = caps[3].parse::<u32>().unwrap();
            Self::from_split(&caps[1], map_num, stage_num)
        } else {
            Err(StageMetaParseError::Rejected)
        }
    }

    /// Parse file name if is main chapters but not eoc
    pub fn from_file_other_main(file_name: &str) -> Result<StageMeta, StageMetaParseError> {
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

        Self::from_selector_main(&[&caps[1], &selector.0.to_string(), &selector.1.to_string()])
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
    pub fn from_ref(selector: &str) -> Result<StageMeta, StageMetaParseError> {
        let reference = DB_REFERENCE_FULL.replace(selector, "$1");

        match DB_REFERENCE_STAGE.captures(&reference) {
            Some(caps) => {
                let chapter: u32 = caps[1].parse().unwrap();
                // necessary since can contain leading 0s
                // TODO probably easier to just remove leading 0s
                let submap: u32 = caps[2].parse().unwrap();
                let stage: u32 = caps[3].parse::<u32>().unwrap() - 1;
                Self::from_numbers(chapter, submap, stage)
            }
            None => Err(StageMetaParseError::Rejected),
        }
    }

    /// Is this even necessary?
    fn from_numbers(
        stage_type: u32,
        map_num: u32,
        stage_num: u32,
    ) -> Result<StageMeta, StageMetaParseError> {
        Self::from_split(&stage_type.to_string(), map_num, stage_num)
    }

    /// Get [StageMeta] from a selector split into variables.
    /// ```
    /// # use rust_wiki::data::stage::raw::stage_metadata::{StageMeta, consts::StageTypeEnum::SoL};
    /// let st = StageMeta::from_split("SoL", 0, 0);
    /// assert_eq!(st.unwrap(), StageMeta { type_name: "Stories of Legend", type_code: "N", type_num: 0, type_enum: SoL, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// ```
    pub fn from_split(
        stage_type: &str,
        map_num: u32,
        stage_num: u32,
    ) -> Result<StageMeta, StageMetaParseError> {
        let Some(stage_type) = get_selector_type(stage_type) else {
            return Err(StageMetaParseError::Invalid);
        };
        Self::from_split_parsed(stage_type, map_num, stage_num)
    }

    /// [from_split][StageMeta::from_split] but with `stage_type` being a code
    /// from [STAGE_TYPES].
    fn from_split_parsed(
        stage_type: &StageType,
        map_num: u32,
        stage_num: u32,
    ) -> Result<StageMeta, StageMetaParseError> {
        let type_name = stage_type.name;
        let type_num = stage_type.number;
        let type_enum = stage_type.type_enum;

        let type_code;
        let map_file_name;
        let stage_file_name;
        if stage_type.code.contains("|") {
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

        Ok(StageMeta {
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

    /// Formats:
    /// - EoC: `["eoc", "0"]` = Korea
    /// - ItF/W: `["itf", "1", "0"]` = Japan Ch. 1
    /// - CotC/Space: `["cotc", "1", "0"]` = Earth Ch. 1
    /// - Aku/DM: `["aku", "0"]` = Korea
    /// - Filibuster: `["filibuster"]`
    /// - Z: `["z", "1", "0"]` = Korea
    // TODO change selector to be something else probably so you don't need to
    // convert to string only to have numbers be parsed again. Or make new
    // internal function for that (does this one even need to be public?).
    pub fn from_selector_main(selector: &[&str]) -> Result<StageMeta, StageMetaParseError> {
        let Some(code) = get_selector_type(selector[0]) else {
            return Err(StageMetaParseError::Invalid);
        };

        let type_name = code.name;
        let type_code = code.code;
        let mut type_num = code.number;
        let type_enum = code.type_enum;

        let (map_num, stage_num, map_file_name, stage_file_name) =
            match selector[0].to_lowercase().as_str() {
                "eoc" => {
                    let stage_num: u32 = selector[1].parse::<u32>().unwrap();
                    (
                        0_u32,
                        stage_num,
                        "stageNormal0.csv".to_string(),
                        format!("stage{stage_num:02}.csv"),
                    )
                }
                "itf" | "w" => {
                    let map_num: u32 = selector[1].parse::<u32>().unwrap() + 2;
                    assert!(3 <= map_num && map_num <= 5);
                    let stage_num: u32 = selector[2].parse::<u32>().unwrap();

                    let map_file = format!("stageNormal1_{}.csv", map_num - 3);
                    let stage_file = format!("stageW{:02}_{stage_num:02}.csv", map_num + 1);

                    (map_num, stage_num, map_file, stage_file)
                }
                "cotc" | "space" => {
                    let map_num: u32 = selector[1].parse::<u32>().unwrap() + 5;
                    assert!(6 <= map_num && map_num <= 8);
                    let stage_num: u32 = selector[2].parse::<u32>().unwrap();

                    let map_file = format!("stageNormal2_{}.csv", map_num - 6);
                    let stage_file = format!("stageSpace{:02}_{stage_num:02}.csv", map_num + 1);

                    (map_num, stage_num, map_file, stage_file)
                }
                "aku" | "dm" => {
                    let stage_num: u32 = selector[1].parse::<u32>().unwrap();
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
                    let mut chap_num: u32 = selector[1].parse().unwrap();
                    assert!(1 <= chap_num && chap_num <= 9);

                    type_num = 20 + ((chap_num - 1) / 3);
                    let map_num = (chap_num - 1) % 3;
                    let stage_num = selector[2].parse::<u32>().unwrap();

                    let map_file = format!(
                        "stageNormal{}_{}_Z.csv",
                        (chap_num - 1) / 3,
                        (chap_num - 1) % 3
                    );

                    if chap_num <= 3 {
                        chap_num -= 1;
                    }
                    let stage_file = format!("stageZ{chap_num:02}_{stage_num:02}.csv");

                    (map_num, stage_num, map_file, stage_file)
                }
                _ => return Err(StageMetaParseError::Invalid),
            };

        Ok(StageMeta {
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

#[cfg(test)]
mod tests {
    use super::*;
    use consts::STAGE_TYPES;
    use rand::random;
    use StageTypeEnum as T;

    #[test]
    fn test_from_split_sol() {
        let answer = StageMeta {
            type_name: "Stories of Legend",
            type_code: "N",
            type_num: 0,
            type_enum: T::SoL,
            map_num: 0,
            stage_num: 0,
            map_file_name: "MapStageDataN_000.csv".to_string(),
            stage_file_name: "stageRN000_00.csv".to_string(),
        };

        let st = StageMeta::from_split("SoL", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_split("sol", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_split("n", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_split("rn", 0, 0).unwrap();
        assert_eq!(st, answer);

        assert_eq!(answer.type_map_num(), (0, 0));
    }

    #[test]
    fn test_from_split_ex() {
        let answer = StageMeta {
            type_name: "Extra Stages",
            type_code: "EX",
            type_num: 4,
            type_enum: T::Extra,
            map_num: 0,
            stage_num: 0,
            map_file_name: "MapStageDataRE_000.csv".to_string(),
            stage_file_name: "stageEX000_00.csv".to_string(),
        };

        let st = StageMeta::from_split("eXTRA", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_split("extra", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_split("4", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_split("RE", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_split("EX", 0, 0).unwrap();
        assert_eq!(st, answer);

        assert_eq!(answer.type_map_num(), (4, 0));
    }

    #[test]
    fn test_from_selector_main() {
        let st = StageMeta::from_selector_main(&["eoc", "0"]).unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 0,
                stage_num: 0,
                map_file_name: "stageNormal0.csv".to_string(),
                stage_file_name: "stage00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (3, 0));

        let st = StageMeta::from_selector_main(&["itf", "1", "0"]).unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 3,
                stage_num: 0,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (3, 3));

        let st = StageMeta::from_selector_main(&["cotc", "1", "0"]).unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 6,
                stage_num: 0,
                map_file_name: "stageNormal2_0.csv".to_string(),
                stage_file_name: "stageSpace07_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (3, 6));

        let st = StageMeta::from_selector_main(&["aku", "0"]).unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Aku Realms",
                type_code: "DM",
                type_num: 30,
                type_enum: T::AkuRealms,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataDM_000.csv".to_string(),
                stage_file_name: "stageDM000_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (30, 0));

        let st = StageMeta::from_selector_main(&["filibuster"]).unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Filibuster Invasion",
                type_code: "",
                type_num: 23,
                type_enum: T::Filibuster,
                map_num: 0,
                stage_num: 0,
                map_file_name: "stageNormal2_2_Invasion.csv".to_string(),
                stage_file_name: "stageSpace09_Invasion_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (23, 0));

        let st = StageMeta::from_selector_main(&["z", "7", "0"]).unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Outbreaks",
                type_code: "main",
                type_num: 22,
                type_enum: T::Outbreaks,
                map_num: 0,
                stage_num: 0,
                map_file_name: "stageNormal2_0_Z.csv".to_string(),
                stage_file_name: "stageZ07_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (22, 0));
    }

    #[test]
    fn test_from_split_fail() {
        let st = StageMeta::from_split("doesn't exist", 0, 0);
        assert_eq!(st, Err(StageMetaParseError::Invalid));
    }

    #[test]
    fn test_from_selector() {
        let st = StageMeta::from_selector("N 0 0").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                type_enum: T::SoL,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (0, 0));

        let st = StageMeta::from_selector("sol 0 0").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                type_enum: T::SoL,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (0, 0));

        let st = StageMeta::from_selector("T 0 0").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Catclaw Dojo",
                type_code: "T",
                type_num: 6,
                type_enum: T::Dojo,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataT_000.csv".to_string(),
                stage_file_name: "stageRT000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (6, 0));

        let st = StageMeta::from_selector("EX 0 0").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Extra Stages",
                type_code: "EX",
                type_num: 4,
                type_enum: T::Extra,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataRE_000.csv".to_string(),
                stage_file_name: "stageEX000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (4, 0));

        let st = StageMeta::from_selector("COTC 1 0").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 6,
                stage_num: 0,
                map_file_name: "stageNormal2_0.csv".to_string(),
                stage_file_name: "stageSpace07_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (3, 6));
    }

    #[test]
    fn test_from_file() {
        let st = StageMeta::from_file("stageRN000_00.csv").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                type_enum: T::SoL,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (0, 0));

        let st = StageMeta::from_file("stageRT000_00.csv").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Catclaw Dojo",
                type_code: "T",
                type_num: 6,
                type_enum: T::Dojo,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataT_000.csv".to_string(),
                stage_file_name: "stageRT000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (6, 0));

        let st = StageMeta::from_file("stageL000_00.csv").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Labyrinth",
                type_code: "L",
                type_num: 33,
                type_enum: T::Labyrinth,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataL_000.csv".to_string(),
                stage_file_name: "stageL000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (33, 0));

        let st = StageMeta::from_file("stageEX000_00.csv").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Extra Stages",
                type_code: "EX",
                type_num: 4,
                type_enum: T::Extra,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataRE_000.csv".to_string(),
                stage_file_name: "stageEX000_00.csv".to_string(),
            }
        );
        assert_eq!(st.type_map_num(), (4, 0));
    }

    #[test]
    fn test_from_file_main() {
        let st = StageMeta::from_file("stageSpace07_00.csv").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 6,
                stage_num: 0,
                map_file_name: "stageNormal2_0.csv".to_string(),
                stage_file_name: "stageSpace07_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (3, 6));

        let st = StageMeta::from_file("stageZ00_00.csv").unwrap();
        assert_eq!(
            st,
            StageMeta {
                type_name: "Outbreaks",
                type_code: "main",
                type_num: 20,
                type_enum: T::Outbreaks,
                map_num: 0,
                stage_num: 0,
                map_file_name: "stageNormal0_0_Z.csv".to_string(),
                stage_file_name: "stageZ00_00.csv".to_string()
            }
        );
        assert_eq!(st.type_map_num(), (20, 0));
    }

    #[test]
    fn test_from_ref() {
        let answer = StageMeta {
            type_name: "Stories of Legend",
            type_code: "N",
            type_num: 0,
            type_enum: T::SoL,
            map_num: 0,
            stage_num: 0,
            map_file_name: "MapStageDataN_000.csv".to_string(),
            stage_file_name: "stageRN000_00.csv".to_string(),
        };
        assert_eq!(answer.type_map_num(), (0, 0));

        let st = StageMeta::from_ref("*https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_ref("https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = StageMeta::from_ref("s00000-01").unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_new() {
        let selector = "*https://battlecats-db.com/stage/s01382-03.html";
        assert_eq!(
            StageMeta::from_ref(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::new(selector).unwrap(),
            StageMeta {
                type_name: "Event Stages",
                type_code: "S",
                type_num: 1,
                type_enum: T::Event,
                map_num: 382,
                stage_num: 2,
                map_file_name: "MapStageDataS_382.csv".to_string(),
                stage_file_name: "stageRS382_02.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (1, 382));

        let selector = "ItF 1 48";
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 3,
                stage_num: 48,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_48.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (3, 3));

        let selector = "DM 0";
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta {
                type_name: "Aku Realms",
                type_code: "DM",
                type_num: 30,
                type_enum: T::AkuRealms,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataDM_000.csv".to_string(),
                stage_file_name: "stageDM000_00.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (30, 0));

        let selector = "Filibuster";
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta {
                type_name: "Filibuster Invasion",
                type_code: "",
                type_num: 23,
                type_enum: T::Filibuster,
                map_num: 0,
                stage_num: 0,
                map_file_name: "stageNormal2_2_Invasion.csv".to_string(),
                stage_file_name: "stageSpace09_Invasion_00.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (23, 0));

        let selector = "z 5 0";
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::from_selector(selector).unwrap(),
            StageMeta {
                type_name: "Outbreaks",
                type_code: "main",
                type_num: 21,
                type_enum: T::Outbreaks,
                map_num: 1,
                stage_num: 0,
                map_file_name: "stageNormal1_1_Z.csv".to_string(),
                stage_file_name: "stageZ05_00.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (21, 1));

        let selector = "stageRN013_05.csv";
        assert_eq!(
            StageMeta::from_file(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::from_file(selector).unwrap(),
            StageMeta {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                type_enum: T::SoL,
                map_num: 13,
                stage_num: 5,
                map_file_name: "MapStageDataN_013.csv".to_string(),
                stage_file_name: "stageRN013_05.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (0, 13));

        let selector = "stageRN000_00.csv";
        assert_eq!(
            StageMeta::from_file(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::from_file(selector).unwrap(),
            StageMeta {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                type_enum: T::SoL,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (0, 0));

        let selector = "stageW04_05.csv";
        assert_eq!(
            StageMeta::from_file(selector).unwrap(),
            StageMeta::new(selector).unwrap()
        );
        assert_eq!(
            StageMeta::from_file(selector).unwrap(),
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 3,
                stage_num: 5,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_05.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (3, 3));

        let selector = "stageW04_05.csv";
        assert_eq!(
            StageMeta::new(&String::from(selector)),
            StageMeta::new(selector)
        );
        assert_eq!(
            StageMeta::new(&String::from(selector)).unwrap(),
            StageMeta {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                type_enum: T::MainChapters,
                map_num: 3,
                stage_num: 5,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_05.csv".to_string()
            }
        );
        assert_eq!(StageMeta::new(selector).unwrap().type_map_num(), (3, 3));
    }

    #[test]
    fn test_stage_type_error() {
        assert_eq!(StageMeta::new("unknown 0"), None);
        assert_eq!(
            StageMeta::from_file("file no exist"),
            Err(StageMetaParseError::Rejected)
        );
        assert_eq!(
            StageMeta::from_ref("not a reference"),
            Err(StageMetaParseError::Rejected)
        );
        assert_eq!(
            StageMeta::from_selector_main(&["none"]),
            Err(StageMetaParseError::Invalid)
        );
    }

    #[test]
    #[should_panic]
    fn test_negative_selector() {
        let _ = StageMeta::from_selector("Q 2 -1");
    }

    #[test]
    #[should_panic]
    fn test_non_numeric_selector() {
        let _ = StageMeta::from_selector("Labyrinth two three");
    }

    #[test]
    #[should_panic]
    fn test_not_enough_args() {
        let _ = StageMeta::from_selector_main(&["itf"]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_number_low_itf() {
        let _ = StageMeta::from_selector_main(&["itf", "0", "0"]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_number_low_cotc() {
        let _ = StageMeta::from_selector_main(&["cotc", "0", "0"]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_number_high() {
        let _ = StageMeta::from_selector_main(&["z", "10", "0"]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_number_neg() {
        let _ = StageMeta::from_selector_main(&["itf", "1", "-1"]);
    }

    #[test]
    fn test_random_properties() {
        const NUM_ITERATIONS: usize = 20;
        for code in STAGE_TYPES {
            if matches!(
                code.type_enum,
                T::MainChapters | T::Outbreaks | T::Filibuster | T::AkuRealms
            ) {
                continue;
            }

            for _ in 0..NUM_ITERATIONS {
                let (map, stage) = (random::<u32>() % 1000, random::<u32>() % 1000);
                let st = StageMeta::from_split_parsed(&code, map, stage).unwrap();
                let file_name = &st.stage_file_name;
                assert_eq!(
                    file_name,
                    &StageMeta::from_file(file_name).unwrap().stage_file_name
                );
                let type_code = {
                    if code.code == "RE|EX" {
                        "EX"
                    } else {
                        code.code
                    }
                };
                assert_eq!(
                    st,
                    StageMeta {
                        type_name: code.name,
                        type_code,
                        type_num: code.number,
                        type_enum: code.type_enum,
                        map_num: map,
                        stage_num: stage,

                        map_file_name: st.map_file_name.to_string(),
                        stage_file_name: st.stage_file_name.to_string(),
                        // these 2 are more difficult to test properly
                    }
                );
                assert_eq!(st.type_map_num(), (st.type_num, st.map_num));
                assert_eq!(
                    st,
                    StageMeta::new(&format!("{} {map} {stage}", code.number)).unwrap()
                );
                assert_eq!(
                    st,
                    StageMeta::new(&format!("s{:02}{:03}-{:02}", code.number, map, stage + 1))
                        .unwrap()
                );
            }
        }
    }

    #[test]
    fn test_random_properties_main() {
        const NUM_ITERATIONS: usize = 20;

        let code: &StageType = &STAGE_TYPES[3];
        let selector = "eoc";
        for _ in 0..NUM_ITERATIONS {
            let stage = random::<u32>() % 100;
            // EoC only supports 2 digits
            let st = StageMeta::from_selector_main(&[selector, &stage.to_string()]).unwrap();
            let file_name = &st.stage_file_name;
            assert_eq!(
                file_name,
                &StageMeta::from_file(file_name).unwrap().stage_file_name
            );
            assert_eq!(
                st,
                StageMeta {
                    type_name: code.name,
                    type_code: code.code,
                    type_num: code.number,
                    type_enum: code.type_enum,
                    map_num: 0,
                    stage_num: stage,
                    map_file_name: st.map_file_name.to_string(),
                    stage_file_name: st.stage_file_name.to_string(),
                }
            );
            assert_eq!(st.type_map_num(), (3, 0));
            assert_eq!(st, StageMeta::new(&format!("{selector} {stage}")).unwrap());
        }

        let code: &StageType = &STAGE_TYPES[3];
        let selector = "itf";
        for _ in 0..NUM_ITERATIONS {
            let (map, stage) = (random::<u32>() % 3 + 1, random::<u32>() % 1000);
            // itf is 1-based so need +1
            let st =
                StageMeta::from_selector_main(&[selector, &map.to_string(), &stage.to_string()])
                    .unwrap();
            let file_name = &st.stage_file_name;
            assert_eq!(
                file_name,
                &StageMeta::from_file(file_name).unwrap().stage_file_name
            );
            assert_eq!(
                st,
                StageMeta {
                    type_name: code.name,
                    type_code: code.code,
                    type_num: code.number,
                    type_enum: code.type_enum,
                    map_num: map + 2,
                    // 3, 4, 5
                    stage_num: stage,

                    map_file_name: st.map_file_name.to_string(),
                    stage_file_name: st.stage_file_name.to_string(),
                }
            );
            assert_eq!(st.type_map_num(), (3, map + 2));
            assert_eq!(
                st,
                StageMeta::new(&format!("{selector} {map} {stage}")).unwrap()
            );
        }

        let code: &StageType = &STAGE_TYPES[3];
        let selector = "cotc";
        for _ in 0..NUM_ITERATIONS {
            let (map, stage) = (random::<u32>() % 3 + 1, random::<u32>() % 1000);
            // cotc is 1-based so need +1
            let st =
                StageMeta::from_selector_main(&[selector, &map.to_string(), &stage.to_string()])
                    .unwrap();
            let file_name = &st.stage_file_name;
            assert_eq!(
                file_name,
                &StageMeta::from_file(file_name).unwrap().stage_file_name
            );
            assert_eq!(
                st,
                StageMeta {
                    type_name: code.name,
                    type_code: code.code,
                    type_num: code.number,
                    type_enum: code.type_enum,
                    map_num: map + 5,
                    // 6, 7, 8
                    stage_num: stage,

                    map_file_name: st.map_file_name.to_string(),
                    stage_file_name: st.stage_file_name.to_string(),
                }
            );
            assert_eq!(st.type_map_num(), (3, map + 5));
            assert_eq!(
                st,
                StageMeta::new(&format!("{selector} {map} {stage}")).unwrap()
            );
        }

        let code: &StageType = &STAGE_TYPES[16];
        let selector = "aku";
        for _ in 0..NUM_ITERATIONS {
            let stage = random::<u32>() % 1000;
            let st = StageMeta::from_selector_main(&[selector, &stage.to_string()]).unwrap();
            let file_name = &st.stage_file_name;
            assert_eq!(
                file_name,
                &StageMeta::from_file(file_name).unwrap().stage_file_name
            );
            assert_eq!(
                st,
                StageMeta {
                    type_name: code.name,
                    type_code: code.code,
                    type_num: code.number,
                    type_enum: code.type_enum,
                    map_num: 0,
                    stage_num: stage,

                    map_file_name: st.map_file_name.to_string(),
                    stage_file_name: st.stage_file_name.to_string(),
                }
            );
            assert_eq!(st.type_map_num(), (30, 0));
            assert_eq!(st, StageMeta::new(&format!("{selector} {stage}")).unwrap());
        }

        let code: &StageType = &STAGE_TYPES[11];
        let selector = "Z";
        for _ in 0..NUM_ITERATIONS {
            let (map, stage) = (random::<usize>() % 8 + 1, random::<u32>() % 1000);
            // Currently 8 chapters exist
            let st =
                StageMeta::from_selector_main(&[selector, &map.to_string(), &stage.to_string()])
                    .unwrap();
            let file_name = &st.stage_file_name;
            assert_eq!(
                file_name,
                &StageMeta::from_file(file_name).unwrap().stage_file_name
            );

            let mapind = map as u32 - 1;
            let type_num = 20 + mapind / 3;
            let map_num = mapind % 3;
            assert_eq!(st.type_map_num(), (type_num, map_num));

            assert_eq!(
                st,
                StageMeta {
                    type_name: code.name,
                    type_code: code.code,
                    type_num,
                    type_enum: code.type_enum,
                    map_num,
                    stage_num: stage,

                    map_file_name: st.map_file_name.to_string(),
                    stage_file_name: st.stage_file_name.to_string(),
                }
            );
            assert_eq!(
                st,
                StageMeta::new(&format!("{selector} {map} {stage}")).unwrap()
            );
        }
    }
}
