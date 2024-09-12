mod consts {
    use lazy_static::lazy_static;
    use regex::{Regex, RegexBuilder};

    #[derive(Debug)]
    pub struct StageCode {
        pub name: &'static str,
        pub number: i32,
        pub code: &'static str,
        pub has_r_prefix: bool,
    }

    const fn initialise_code(
        name: &'static str,
        number: i32,
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

    #[rustfmt::skip]
    pub const STAGE_CODES: [StageCode; 18] = [
        initialise_code("Stories of Legend",    000, "N",     true),
        initialise_code("Event Stages",         001, "S",     true),
        initialise_code("Collaboration Stages", 002, "C",     true),
        initialise_code("Main Chapters",        003, "main",  false),
        initialise_code("Extra Stages",         004, "RE|EX", false),
        initialise_code("Catclaw Dojo",         006, "T",     true),
        initialise_code("Towers",               007, "V",     true),
        initialise_code("Ranking Dojo",         011, "R",     true),
        initialise_code("Challenge Battle",     012, "M",     true),
        initialise_code("Uncanny Legends",      013, "NA",    true),
        initialise_code("Catamin Stages",       014, "B",     true),
        initialise_code("Gauntlets",            024, "A",     true),
        initialise_code("Enigma Stages",        025, "H",     true),
        initialise_code("Collab Gauntlets",     027, "CA",    true),
        initialise_code("Behemoth Culling",     031, "Q",     true),
        initialise_code("Labyrinth",            033, "L",     false),
        initialise_code("Zero Legends",         034, "ND",    true),
        initialise_code("Colosseum",            036, "SR",    true),
    ];

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

#[derive(Debug)]
#[allow(dead_code)]
pub struct StageType {
    pub type_name: &'static str,
    pub type_code: &'static str,
    pub type_num: i32,
    pub map_num: i32,
    pub stage_num: i32,

    pub map_file_name: String,
    pub stage_file_name: String,
}

#[derive(Debug)]
pub enum StageTypeError {
    /// Not the correct function to use.
    Rejected,
    /// Either selector doesn't exist or numbers are not given.
    Invalid,
}

impl StageType {
    pub fn new(selector: &str) -> Result<StageType, StageTypeError> {
        todo!()
    }

    pub fn from_selector(selector: &str) -> Result<StageType, StageTypeError> {
        let selector: Vec<&str> = selector.split(" ").collect();
        let stage_type =
            Self::get_selector_type(selector.get(0).expect("Selector should have content!"))?;

        match stage_type {
            "main" => Self::from_selector_main(selector),
            _ => {
                // let chapter: i32 = stage_type.parse().unwrap();
                let submap: i32 = (&selector[1]).parse().unwrap();
                let stage: i32 = (&selector[2]).parse::<i32>().unwrap();
                return Self::from_new(stage_type, submap, stage);
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
            // will deal with this later
            ()
        } else if file_name.contains("_") {
            let caps = FILE_PATTERNS.default.captures(file_name).unwrap();
            let map_num: i32 = (&caps[2]).parse::<i32>().unwrap();
            let stage_num: i32 = (&caps[3]).parse::<i32>().unwrap();
            return Self::from_split(&caps[1], map_num, stage_num);
        } else {
            return Err(StageTypeError::Rejected);
        }

        let caps = FILE_PATTERNS.default.captures(file_name).unwrap();
        let mut chap_num = caps[2].parse::<i32>().unwrap();
        if &caps[1] == "Z" && chap_num <= 3 {
            chap_num += 1;
        }

        let stage_num = caps[3].parse::<i32>().unwrap();
        let selector = match &caps[1] {
            "W" => (chap_num - 3, stage_num),
            "Space" => (chap_num - 6, stage_num),
            "DM" => (stage_num, stage_num),
            // sort of a workaround
            "Z" => (stage_num, chap_num),
            _ => unreachable!(),
        };
        Self::from_selector_main(vec![
            &caps[1],
            &selector.0.to_string(),
            &selector.1.to_string(),
        ])
    }

    fn get_selector_type(selector_type: &str) -> Result<&'static str, StageTypeError> {
        for selector_map in STAGE_TYPE_MAP.iter() {
            if selector_map.matcher.is_match(selector_type) {
                return Ok(selector_map.stage_type);
            }
        }

        Err(StageTypeError::Invalid)
    }

    pub fn from_ref(selector: &str) -> Result<StageType, StageTypeError> {
        let reference = DB_REFERENCE_FULL.replace(selector, "$1");

        match DB_REFERENCE_STAGE.captures(&reference) {
            Some(caps) => {
                let chapter: i32 = (&caps[1]).parse().unwrap();
                // necessary since can contain leading 0s
                let submap: i32 = (&caps[2]).parse().unwrap();
                let stage: i32 = (&caps[3]).parse::<i32>().unwrap() - 1;
                return Self::from_numbers(chapter, submap, stage);
            }
            None => Err(StageTypeError::Rejected),
        }
    }

    fn from_numbers(
        stage_type: i32,
        map_num: i32,
        stage_num: i32,
    ) -> Result<StageType, StageTypeError> {
        return Self::from_split(&stage_type.to_string(), map_num, stage_num);
    }

    fn get_stage_code(stage_type: &str) -> StageCode {
        for code in STAGE_CODES {
            if stage_type == code.code {
                return code;
            }
        }

        unreachable!();
    }

    pub fn from_split(
        stage_type: &str,
        map_num: i32,
        stage_num: i32,
    ) -> Result<StageType, StageTypeError> {
        Self::from_new(Self::get_selector_type(stage_type)?, map_num, stage_num)
    }

    /// IDK this naming convention any more
    fn from_new(
        stage_type: &str,
        map_num: i32,
        stage_num: i32,
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

    fn from_selector_main(selector: Vec<&str>) -> Result<StageType, StageTypeError> {
        let code = &STAGE_CODES[3];
        let type_name = code.name;
        let type_code = code.code;
        let type_num = code.number;

        let (map_num, stage_num, map_file_name, stage_file_name) =
            match selector[0].to_lowercase().as_str() {
                "eoc" => {
                    let stage_num: i32 = selector[1].parse::<i32>().unwrap();
                    (
                        9_i32,
                        stage_num,
                        "stageNormal0.csv".to_string(),
                        format!("stage{stage_num:02}.csv"),
                    )
                }
                "itf" | "w" => {
                    let map_num: i32 = selector[1].parse::<i32>().unwrap() + 2;
                    let stage_num: i32 = selector[2].parse::<i32>().unwrap();
                    let map_file = format!("stageNormal1_{}.csv", map_num - 3);
                    let stage_file = format!("stageW{:02}_{stage_num:02}.csv", map_num + 1);
                    (map_num, stage_num, map_file, stage_file)
                }
                "cotc" | "space" => {
                    let map_num: i32 = selector[1].parse::<i32>().unwrap() + 5;
                    let stage_num: i32 = selector[2].parse::<i32>().unwrap();
                    let map_file = format!("stageNormal2_{}.csv", map_num - 6);
                    let stage_file = format!("stageSpace{:02}_{stage_num:02}.csv", map_num + 1);
                    (map_num, stage_num, map_file, stage_file)
                }
                "aku" | "dm" => {
                    let stage_num: i32 = selector[1].parse::<i32>().unwrap();
                    (
                        14_i32,
                        stage_num,
                        "MapStageDataDM_000.csv".to_string(),
                        format!("stageDM000_{stage_num:02}.csv"),
                    )
                }
                "filibuster" => (
                    11_i32,
                    0_i32,
                    "stageNormal2_2_Invasion.csv".to_string(),
                    "stageSpace09_Invasion_00.csv".to_string(),
                ),
                "z" => {
                    let mut chap_num: usize = selector[1].parse().unwrap();

                    let map_num = [0, 1, 2, 10, 12, 13, 15, 16][chap_num - 1];
                    let stage_num = selector[2].parse::<i32>().unwrap();
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
                _ => unreachable!(),
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
    println!("{:?}", STAGE_CODES);
    println!("{:?}", STAGE_TYPE_MAP.iter().collect::<Vec<_>>());
    println!("{:?}", StageType::from_split("0", 0, 0));
    println!(
        "{:?}",
        StageType::from_ref("*https://battlecats-db.com/stage/s01382-03.html")
    );
    println!("{:?}", StageType::from_selector(selector));
    println!("{:?}", StageType::from_selector("ItF 1 48"));
    println!("{:?}", StageType::from_selector("DM 0"));
    println!("{:?}", StageType::from_selector("Filibuster"));
    println!("{:?}", StageType::from_selector("z 5 0"));
    println!("{:?}", StageType::from_file("stageRN013_05.csv"));
    println!("{:?}", StageType::from_file("stageRN000_00.csv"));
    println!("{:?}", StageType::from_file("stageW04_05.csv"));
    // println!("{:?}", StageType::new(&String::from("stageW04_05.csv")));
    selector
}
// from split, from file, from ref
