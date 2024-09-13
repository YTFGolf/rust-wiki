pub mod consts {
    use lazy_static::lazy_static;
    use regex::{Regex, RegexBuilder};

    #[derive(Debug, PartialEq)]
    pub struct StageCode {
        pub name: &'static str,
        pub number: usize,
        pub code: &'static str,
        pub has_r_prefix: bool,
    }

    const fn initialise_type_code(
        name: &'static str,
        number: usize,
        code: &'static str,
        has_r_prefix: bool,
    ) -> StageCode {
        StageCode {
            name,
            number,
            code,
            has_r_prefix,
        }
    }

    #[derive(Debug)]
    pub struct StageTypeMap {
        pub matcher: Regex,
        pub stage_type: &'static str,
    }

    fn initialise_type_map(pattern: &'static str, stage_type: &'static str) -> StageTypeMap {
        let re = format!("^({})$", pattern);
        let matcher = RegexBuilder::new(&re)
            .case_insensitive(true)
            .build()
            .unwrap();
        StageTypeMap {
            matcher,
            stage_type,
        }
    }

    #[rustfmt::skip]
    pub const STAGE_CODES: [StageCode; 18] = [
        initialise_type_code("Stories of Legend",    000, "N",     true),
        initialise_type_code("Event Stages",         001, "S",     true),
        initialise_type_code("Collaboration Stages", 002, "C",     true),
        initialise_type_code("Main Chapters",        003, "main",  false),
        initialise_type_code("Extra Stages",         004, "RE|EX", false),
        initialise_type_code("Catclaw Dojo",         006, "T",     true),
        initialise_type_code("Towers",               007, "V",     true),
        initialise_type_code("Ranking Dojo",         011, "R",     true),
        initialise_type_code("Challenge Battle",     012, "M",     true),
        initialise_type_code("Uncanny Legends",      013, "NA",    true),
        initialise_type_code("Catamin Stages",       014, "B",     true),
        initialise_type_code("Gauntlets",            024, "A",     true),
        initialise_type_code("Enigma Stages",        025, "H",     true),
        initialise_type_code("Collab Gauntlets",     027, "CA",    true),
        initialise_type_code("Behemoth Culling",     031, "Q",     true),
        initialise_type_code("Labyrinth",            033, "L",     false),
        initialise_type_code("Zero Legends",         034, "ND",    true),
        initialise_type_code("Colosseum",            036, "SR",    true),
    ];

    lazy_static! {
    #[rustfmt::skip]
    pub static ref STAGE_TYPE_MAP: [StageTypeMap; 19] = [
        initialise_type_map("SoL|0|N|RN",                               "N"),
        initialise_type_map("Event|Special|1|S|RS",                     "S"),
        initialise_type_map("Collab|2|C|RC",                            "C"),
        initialise_type_map("Extra|4|RE|EX",                            "RE|EX"),
        initialise_type_map("Dojo|6|T|RT",                              "T"),
        initialise_type_map("Tower|7|V|RV",                             "V"),
        initialise_type_map("Rank|11|R|RR",                             "R"),
        initialise_type_map("Challenge|12|M|RM",                        "M"),
        initialise_type_map("UL|13|NA|RNA",                             "NA"),
        initialise_type_map("Catamin|14|B|RB",                          "B"),
        initialise_type_map("LQ|16|D",                                  "Why would you want to do Legend Quest"),
        initialise_type_map("Gauntlet|Baron|24|A|RA",                   "A"),
        initialise_type_map("Enigma|25|H|RH",                           "H"),
        initialise_type_map("27|CA|RCA",                                "CA"),
        initialise_type_map("Behemoth|31|Q|RQ",                         "Q"),
        initialise_type_map("Labyrinth|33|L",                           "L"),
        initialise_type_map("ZL|34|ND|RND",                             "ND"),
        initialise_type_map("Colosseum|36|SR|RSR",                      "SR"),
        initialise_type_map("EoC|ItF|W|CotC|Space|Aku|DM|Z|Filibuster", "main")
    ];
    }
}
use consts::{StageCode, STAGE_CODES, STAGE_TYPE_MAP};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DB_REFERENCE_FULL: Regex =
        Regex::new(r"\*?https://battlecats-db.com/stage/(s[\d\-]+).html").unwrap();
    static ref DB_REFERENCE_STAGE: Regex = Regex::new(r"^s(\d{2})(\d{3})\-(\d{2})$").unwrap();
    static ref FILE_PATTERNS: FilePatterns = FilePatterns {
        eoc: Regex::new(r"^stage(\d{2})\.csv$").unwrap(),
        other_main: Regex::new(r"^stage(W|Space|DM|Z)\d\d.*\.csv$").unwrap(),
        default: Regex::new(r"^stage([\D]*)([\d]*)_([\d]*)\.csv$").unwrap(),
    };
}
struct FilePatterns {
    eoc: Regex,
    /// Main chapters that aren't EoC
    other_main: Regex,
    /// Every chapter that isn't EoC
    default: Regex,
}

#[derive(Debug, PartialEq)]
pub struct StageType {
    pub type_name: &'static str,
    pub type_code: &'static str,
    pub type_num: usize,
    pub map_num: usize,
    pub stage_num: usize,

    pub map_file_name: String,
    pub stage_file_name: String,
}

#[derive(Debug, PartialEq)]
pub enum StageTypeError {
    /// Not the correct function to use.
    Rejected,
    /// Either selector doesn't exist or numbers are not given.
    Invalid,
}

impl StageType {
    /// Catch-all method for parsing a selector.
    pub fn new(selector: &str) -> Result<StageType, StageTypeError> {
        if let Ok(st) = Self::from_selector(selector) {
            return Ok(st);
        };
        if let Ok(st) = Self::from_file(selector) {
            return Ok(st);
        };
        if let Ok(st) = Self::from_ref(selector) {
            return Ok(st);
        };

        Err(StageTypeError::Invalid)
    }

    /// Parse space-delimited selector into StageType.
    /// ```
    /// # use rust_wiki::stage::stage_type::StageType;
    /// let selector = "N 0 0";
    /// assert_eq!(StageType::from_selector(selector).unwrap(), StageType { type_name: "Stories of Legend", type_code: "N", type_num: 0, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// ```
    pub fn from_selector(selector: &str) -> Result<StageType, StageTypeError> {
        let selector: Vec<&str> = selector.split(" ").collect();
        let stage_type =
            Self::get_selector_type(selector.get(0).expect("Selector should have content!"))?;

        match stage_type {
            "main" => Self::from_selector_main(selector),
            _ => {
                // let chapter: usize = stage_type.parse().unwrap();
                let submap: usize = (&selector[1]).parse().unwrap();
                let stage: usize = (&selector[2]).parse::<usize>().unwrap();
                return Self::from_split_parsed(stage_type, submap, stage);
            }
        }
    }

    /// Parse file name into stage type.
    /// ```
    /// # use rust_wiki::stage::stage_type::StageType;
    /// let file_name = "stageRN000_00.csv";
    /// assert_eq!(file_name, StageType::from_file(file_name).unwrap().stage_file_name);
    /// ```
    pub fn from_file(file_name: &str) -> Result<StageType, StageTypeError> {
        if file_name == "stageSpace09_Invasion_00.csv" {
            return Self::from_selector_main(vec!["Filibuster"]);
        } else if FILE_PATTERNS.eoc.is_match(file_name) {
            return Self::from_selector_main(vec![
                "eoc",
                &FILE_PATTERNS.eoc.replace(file_name, "$1"),
            ]);
        } else if FILE_PATTERNS.other_main.is_match(file_name) {
            ()
            // will deal with this later
        } else if file_name.contains("_") {
            let caps = FILE_PATTERNS.default.captures(file_name).unwrap();
            let map_num: usize = (&caps[2]).parse::<usize>().unwrap();
            let stage_num: usize = (&caps[3]).parse::<usize>().unwrap();
            return Self::from_split(&caps[1], map_num, stage_num);
        } else {
            return Err(StageTypeError::Rejected);
        }

        // Rest is for main chapters minus EoC
        let caps = FILE_PATTERNS.default.captures(file_name).unwrap();
        let mut chap_num = caps[2].parse::<usize>().unwrap();
        if &caps[1] == "Z" && chap_num <= 3 {
            chap_num += 1;
        }

        let stage_num = caps[3].parse::<usize>().unwrap();
        let selector = match &caps[1] {
            "W" => (chap_num - 3, stage_num),
            "Space" => (chap_num - 6, stage_num),
            "DM" => (stage_num, stage_num),
            // sort of a workaround so this compiles
            "Z" => (chap_num, stage_num),
            _ => unreachable!(),
        };
        Self::from_selector_main(vec![
            &caps[1],
            &selector.0.to_string(),
            &selector.1.to_string(),
        ])
    }

    /// Get `StageCode.code` from `selector_type`.
    fn get_selector_type(selector_type: &str) -> Result<&'static str, StageTypeError> {
        for selector_map in STAGE_TYPE_MAP.iter() {
            if selector_map.matcher.is_match(selector_type) {
                return Ok(selector_map.stage_type);
            }
        }

        Err(StageTypeError::Invalid)
    }

    /// Parse battle-cats.db reference into StageType.
    /// ```
    /// # use rust_wiki::stage::stage_type::StageType;
    /// let reference = "*https://battlecats-db.com/stage/s00000-01.html";
    /// assert_eq!(StageType::from_ref(reference).unwrap(), StageType { type_name: "Stories of Legend", type_code: "N", type_num: 0, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// ```
    pub fn from_ref(selector: &str) -> Result<StageType, StageTypeError> {
        let reference = DB_REFERENCE_FULL.replace(selector, "$1");

        match DB_REFERENCE_STAGE.captures(&reference) {
            Some(caps) => {
                let chapter: usize = (&caps[1]).parse().unwrap();
                // necessary since can contain leading 0s
                let submap: usize = (&caps[2]).parse().unwrap();
                let stage: usize = (&caps[3]).parse::<usize>().unwrap() - 1;
                return Self::from_numbers(chapter, submap, stage);
            }
            None => Err(StageTypeError::Rejected),
        }
    }

    /// Is this even necessary?
    fn from_numbers(
        stage_type: usize,
        map_num: usize,
        stage_num: usize,
    ) -> Result<StageType, StageTypeError> {
        return Self::from_split(&stage_type.to_string(), map_num, stage_num);
    }

    /// Get the StageCode from `STAGE_CODES`.
    fn get_stage_code(stage_type: &str) -> StageCode {
        for code in STAGE_CODES {
            if stage_type == code.code {
                return code;
            }
        }

        panic!("You shouldn't be able to get to this line.");
    }

    /// Get StageType from selectors split into variables.
    /// ```
    /// # use rust_wiki::stage::stage_type::StageType;
    /// let st = StageType::from_split("SoL", 0, 0);
    /// assert_eq!(st.unwrap(), StageType { type_name: "Stories of Legend", type_code: "N", type_num: 0, map_num: 0, stage_num: 0, map_file_name: "MapStageDataN_000.csv".to_string(), stage_file_name: "stageRN000_00.csv".to_string() });
    /// ```
    pub fn from_split(
        stage_type: &str,
        map_num: usize,
        stage_num: usize,
    ) -> Result<StageType, StageTypeError> {
        Self::from_split_parsed(Self::get_selector_type(stage_type)?, map_num, stage_num)
    }

    /// `from_split` but with `stage_type` being a code from `STAGE_CODES`.
    fn from_split_parsed(
        stage_type: &str,
        map_num: usize,
        stage_num: usize,
    ) -> Result<StageType, StageTypeError> {
        let code = Self::get_stage_code(stage_type);

        let type_name = code.name;
        let type_num = code.number;

        let type_code;
        let map_file_name;
        let stage_file_name;
        if code.code.contains("|") {
            // If more than RE|EX is needed this could completely break
            let map = &code.code[..2];
            let stage = &code.code[3..];
            type_code = stage;
            map_file_name = format!("MapStageData{map}_{map_num:03}.csv");
            stage_file_name = format!("stage{stage}{map_num:03}_{stage_num:02}.csv");
        } else {
            let stage_prefix = match code.has_r_prefix {
                true => "R",
                false => "",
            };
            let code = code.code;

            type_code = code;
            map_file_name = format!("MapStageData{code}_{map_num:03}.csv");
            stage_file_name = format!("stage{stage_prefix}{code}{map_num:03}_{stage_num:02}.csv");
        }
        // let type_code = code.code

        Ok(StageType {
            type_name,
            type_code,
            type_num,
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
    pub fn from_selector_main(selector: Vec<&str>) -> Result<StageType, StageTypeError> {
        let code = &STAGE_CODES[3];
        let type_name = code.name;
        let type_code = code.code;
        let type_num = code.number;

        let (map_num, stage_num, map_file_name, stage_file_name) =
            match selector[0].to_lowercase().as_str() {
                "eoc" => {
                    let stage_num: usize = selector[1].parse::<usize>().unwrap();
                    (
                        9_usize,
                        stage_num,
                        "stageNormal0.csv".to_string(),
                        format!("stage{stage_num:02}.csv"),
                    )
                }
                "itf" | "w" => {
                    let map_num: usize = selector[1].parse::<usize>().unwrap() + 2;
                    let stage_num: usize = selector[2].parse::<usize>().unwrap();
                    let map_file = format!("stageNormal1_{}.csv", map_num - 3);
                    let stage_file = format!("stageW{:02}_{stage_num:02}.csv", map_num + 1);
                    (map_num, stage_num, map_file, stage_file)
                }
                "cotc" | "space" => {
                    let map_num: usize = selector[1].parse::<usize>().unwrap() + 5;
                    let stage_num: usize = selector[2].parse::<usize>().unwrap();
                    let map_file = format!("stageNormal2_{}.csv", map_num - 6);
                    let stage_file = format!("stageSpace{:02}_{stage_num:02}.csv", map_num + 1);
                    (map_num, stage_num, map_file, stage_file)
                }
                "aku" | "dm" => {
                    let stage_num: usize = selector[1].parse::<usize>().unwrap();
                    (
                        14_usize,
                        stage_num,
                        "MapStageDataDM_000.csv".to_string(),
                        format!("stageDM000_{stage_num:02}.csv"),
                    )
                }
                "filibuster" => (
                    11_usize,
                    0_usize,
                    "stageNormal2_2_Invasion.csv".to_string(),
                    "stageSpace09_Invasion_00.csv".to_string(),
                ),
                "z" => {
                    let mut chap_num: usize = selector[1].parse().unwrap();

                    let map_num = [0, 1, 2, 10, 12, 13, 15, 16][chap_num - 1];
                    let stage_num = selector[2].parse::<usize>().unwrap();
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
                _ => return Err(StageTypeError::Invalid),
            };

        Ok(StageType {
            type_name,
            type_code,
            type_num,
            map_num,
            stage_num,
            map_file_name,
            stage_file_name,
        })
    }
}

pub fn get_st_obj(selector: &str) -> &str {
    selector
}

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;

    #[test]
    fn test_from_split_sol() {
        let answer = StageType {
            type_name: "Stories of Legend",
            type_code: "N",
            type_num: 0,
            map_num: 0,
            stage_num: 0,
            map_file_name: "MapStageDataN_000.csv".to_string(),
            stage_file_name: "stageRN000_00.csv".to_string(),
        };

        let st = StageType::from_split("SoL", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_split("sol", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_split("n", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_split("rn", 0, 0).unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_from_split_ex() {
        let answer = StageType {
            type_name: "Extra Stages",
            type_code: "EX",
            type_num: 4,
            map_num: 0,
            stage_num: 0,
            map_file_name: "MapStageDataRE_000.csv".to_string(),
            stage_file_name: "stageEX000_00.csv".to_string(),
        };

        let st = StageType::from_split("eXTRA", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_split("extra", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_split("4", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_split("RE", 0, 0).unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_split("EX", 0, 0).unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_from_selector_main() {
        let st = StageType::from_selector_main(vec!["eoc", "0"]).unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 9,
                stage_num: 0,
                map_file_name: "stageNormal0.csv".to_string(),
                stage_file_name: "stage00.csv".to_string()
            }
        );

        let st = StageType::from_selector_main(vec!["itf", "1", "0"]).unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 3,
                stage_num: 0,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_00.csv".to_string()
            }
        );

        let st = StageType::from_selector_main(vec!["cotc", "1", "0"]).unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 6,
                stage_num: 0,
                map_file_name: "stageNormal2_0.csv".to_string(),
                stage_file_name: "stageSpace07_00.csv".to_string()
            }
        );

        let st = StageType::from_selector_main(vec!["aku", "0"]).unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 14,
                stage_num: 0,
                map_file_name: "MapStageDataDM_000.csv".to_string(),
                stage_file_name: "stageDM000_00.csv".to_string()
            }
        );

        let st = StageType::from_selector_main(vec!["filibuster"]).unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 11,
                stage_num: 0,
                map_file_name: "stageNormal2_2_Invasion.csv".to_string(),
                stage_file_name: "stageSpace09_Invasion_00.csv".to_string()
            }
        );

        let st = StageType::from_selector_main(vec!["z", "7", "0"]).unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 15,
                stage_num: 0,
                map_file_name: "stageNormal2_0_Z.csv".to_string(),
                stage_file_name: "stageZ07_00.csv".to_string()
            }
        );
    }

    #[test]
    fn test_from_split_fail() {
        let st = StageType::from_split("doesn't exist", 0, 0);
        assert_eq!(st, Err(StageTypeError::Invalid));
    }

    #[test]
    fn test_from_selector() {
        let st = StageType::from_selector("N 0 0").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string(),
            }
        );

        let st = StageType::from_selector("sol 0 0").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string(),
            }
        );

        let st = StageType::from_selector("T 0 0").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Catclaw Dojo",
                type_code: "T",
                type_num: 6,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataT_000.csv".to_string(),
                stage_file_name: "stageRT000_00.csv".to_string(),
            }
        );

        let st = StageType::from_selector("EX 0 0").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Extra Stages",
                type_code: "EX",
                type_num: 4,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataRE_000.csv".to_string(),
                stage_file_name: "stageEX000_00.csv".to_string(),
            }
        );

        let st = StageType::from_selector("COTC 1 0").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 6,
                stage_num: 0,
                map_file_name: "stageNormal2_0.csv".to_string(),
                stage_file_name: "stageSpace07_00.csv".to_string()
            }
        );
    }

    #[test]
    fn test_from_file() {
        let st = StageType::from_file("stageRN000_00.csv").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string(),
            }
        );

        let st = StageType::from_file("stageRT000_00.csv").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Catclaw Dojo",
                type_code: "T",
                type_num: 6,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataT_000.csv".to_string(),
                stage_file_name: "stageRT000_00.csv".to_string(),
            }
        );

        let st = StageType::from_file("stageL000_00.csv").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Labyrinth",
                type_code: "L",
                type_num: 33,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataL_000.csv".to_string(),
                stage_file_name: "stageL000_00.csv".to_string(),
            }
        );

        let st = StageType::from_file("stageEX000_00.csv").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Extra Stages",
                type_code: "EX",
                type_num: 4,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataRE_000.csv".to_string(),
                stage_file_name: "stageEX000_00.csv".to_string(),
            }
        );
    }

    #[test]
    fn test_from_file_main() {
        let st = StageType::from_file("stageSpace07_00.csv").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 6,
                stage_num: 0,
                map_file_name: "stageNormal2_0.csv".to_string(),
                stage_file_name: "stageSpace07_00.csv".to_string()
            }
        );

        let st = StageType::from_file("stageZ00_00.csv").unwrap();
        assert_eq!(
            st,
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 0,
                stage_num: 0,
                map_file_name: "stageNormal0_0_Z.csv".to_string(),
                stage_file_name: "stageZ00_00.csv".to_string()
            }
        );
    }

    #[test]
    fn test_from_ref() {
        let answer = StageType {
            type_name: "Stories of Legend",
            type_code: "N",
            type_num: 0,
            map_num: 0,
            stage_num: 0,
            map_file_name: "MapStageDataN_000.csv".to_string(),
            stage_file_name: "stageRN000_00.csv".to_string(),
        };

        let st = StageType::from_ref("*https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_ref("https://battlecats-db.com/stage/s00000-01.html").unwrap();
        assert_eq!(st, answer);
        let st = StageType::from_ref("s00000-01").unwrap();
        assert_eq!(st, answer);
    }

    #[test]
    fn test_new() {
        let selector = "*https://battlecats-db.com/stage/s01382-03.html";
        assert_eq!(StageType::from_ref(selector), StageType::new(selector));
        assert_eq!(
            StageType::new(selector).unwrap(),
            StageType {
                type_name: "Event Stages",
                type_code: "S",
                type_num: 1,
                map_num: 382,
                stage_num: 2,
                map_file_name: "MapStageDataS_382.csv".to_string(),
                stage_file_name: "stageRS382_02.csv".to_string()
            }
        );

        let selector = "ItF 1 48";
        assert_eq!(StageType::from_selector(selector), StageType::new(selector));
        assert_eq!(
            StageType::from_selector(selector).unwrap(),
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 3,
                stage_num: 48,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_48.csv".to_string()
            }
        );

        let selector = "DM 0";
        assert_eq!(StageType::from_selector(selector), StageType::new(selector));
        assert_eq!(
            StageType::from_selector(selector).unwrap(),
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 14,
                stage_num: 0,
                map_file_name: "MapStageDataDM_000.csv".to_string(),
                stage_file_name: "stageDM000_00.csv".to_string()
            }
        );

        let selector = "Filibuster";
        assert_eq!(StageType::from_selector(selector), StageType::new(selector));
        assert_eq!(
            StageType::from_selector(selector).unwrap(),
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 11,
                stage_num: 0,
                map_file_name: "stageNormal2_2_Invasion.csv".to_string(),
                stage_file_name: "stageSpace09_Invasion_00.csv".to_string()
            }
        );

        let selector = "z 5 0";
        assert_eq!(StageType::from_selector(selector), StageType::new(selector));
        assert_eq!(
            StageType::from_selector(selector).unwrap(),
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 12,
                stage_num: 0,
                map_file_name: "stageNormal1_1_Z.csv".to_string(),
                stage_file_name: "stageZ05_00.csv".to_string()
            }
        );

        let selector = "stageRN013_05.csv";
        assert_eq!(StageType::from_file(selector), StageType::new(selector));
        assert_eq!(
            StageType::from_file(selector).unwrap(),
            StageType {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                map_num: 13,
                stage_num: 5,
                map_file_name: "MapStageDataN_013.csv".to_string(),
                stage_file_name: "stageRN013_05.csv".to_string()
            }
        );

        let selector = "stageRN000_00.csv";
        assert_eq!(StageType::from_file(selector), StageType::new(selector));
        assert_eq!(
            StageType::from_file(selector).unwrap(),
            StageType {
                type_name: "Stories of Legend",
                type_code: "N",
                type_num: 0,
                map_num: 0,
                stage_num: 0,
                map_file_name: "MapStageDataN_000.csv".to_string(),
                stage_file_name: "stageRN000_00.csv".to_string()
            }
        );

        let selector = "stageW04_05.csv";
        assert_eq!(StageType::from_file(selector), StageType::new(selector));
        assert_eq!(
            StageType::from_file(selector).unwrap(),
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 3,
                stage_num: 5,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_05.csv".to_string()
            }
        );

        let selector = "stageW04_05.csv";
        assert_eq!(
            StageType::new(&String::from(selector)),
            StageType::new(selector)
        );
        assert_eq!(
            StageType::new(&String::from(selector)).unwrap(),
            StageType {
                type_name: "Main Chapters",
                type_code: "main",
                type_num: 3,
                map_num: 3,
                stage_num: 5,
                map_file_name: "stageNormal1_0.csv".to_string(),
                stage_file_name: "stageW04_05.csv".to_string()
            }
        );
    }

    #[test]
    fn test_stage_type_error() {
        assert_eq!(StageType::new("unknown 0"), Err(StageTypeError::Invalid));
        assert_eq!(
            StageType::from_file("file no exist"),
            Err(StageTypeError::Rejected)
        );
        assert_eq!(
            StageType::from_ref("not a reference"),
            Err(StageTypeError::Rejected)
        );
        assert_eq!(
            StageType::from_selector_main(vec!["none"]),
            Err(StageTypeError::Invalid)
        );
    }

    #[test]
    fn test_get_selector_type() {
        assert_eq!(StageType::get_selector_type("itf").unwrap(), "main");
        assert_eq!(
            StageType::get_selector_type("itf2"),
            Err(StageTypeError::Invalid)
        );
    }

    #[test]
    fn test_get_stage_code() {
        assert_eq!(StageType::get_stage_code("main"), STAGE_CODES[3]);
    }

    #[test]
    fn test_random_properties(){
        const NUM_ITERATIONS: usize = 50;
        for code in STAGE_CODES{
            if code.code == "main"{continue}
            for _ in 0..NUM_ITERATIONS{
                let (map, stage) = (random::<usize>() % 1000, random::<usize>() % 100);
                let st = StageType::from_split_parsed(code.code, map, stage).unwrap();
                let file_name = st.stage_file_name;
                assert_eq!(file_name, StageType::from_file(&file_name).unwrap().stage_file_name);
            }
        }
    }

    #[test]
    fn test_random_properties_main(){
    }

    // normal, ex, then main, then fail
    // ref do *htt, htt, s0

    // [x] split
    // [x] selector
    // [x] file
    // [x] ref
    // [x] new
    // [x] failing
    // [x] internals
    // [ ] property stuff

    // #[test]
    // fn test_from_file_property(){
    //     // let selector =
    // }

    // property
    // selector = stage_file_name
    // selector = "{type_num} {map_num} {stage_num}"
    // selector = "s{type_num:02}{map_num:03}-{stage_num:03}"
}
