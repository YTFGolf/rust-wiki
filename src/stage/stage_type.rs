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
use consts::{STAGE_CODES, STAGE_TYPE_MAP};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DB_REFERENCE_FULL: Regex =
        Regex::new(r"\*?https://battlecats-db.com/stage/(s[\d\-]+).html").unwrap();
    static ref DB_REFERENCE_STAGE: Regex = Regex::new(r"^s(\d{2})(\d{3})\-(\d{2})$").unwrap();
}

#[derive(Debug)]
struct StageType {
    pub stage_type: &'static str,
    pub type_code: &'static str,
    pub type_num: i32,
    pub map_num: i32,
    pub stage_num: i32,

    pub map_file_name: String,
    pub stage_file_name: String,
}

#[derive(Debug)]
enum StageTypeError {
    Rejected,
}

impl StageType {
    pub fn from_ref(selector: &str) -> Result<StageType, StageTypeError> {
        let reference = DB_REFERENCE_FULL.replace(selector, "$1");

        if let Some(caps) = DB_REFERENCE_STAGE.captures(&reference) {
            // let chapter: i32 = caps[1].parse().unwrap();
            let submap: i32 = caps[2].parse().unwrap();
            let stage: i32 = caps[3].parse::<i32>().unwrap() - 1;

            return Self::from_split(&caps[1], submap, stage);
        }

        return Err(StageTypeError::Rejected);
    }

    pub fn from_numbers(stage_type: i32, map_num: i32, stage_num: i32)-> Result<StageType, StageTypeError> {
        return Self::from_split(&stage_type.to_string(), map_num, stage_num)
    }

    pub fn from_split(stage_type: &str, map_num: i32, stage_num: i32) -> Result<StageType, StageTypeError> {
        Ok(StageType {
            stage_type: "the",
            type_code: "the",
            type_num: 0,
            map_num: 0,
            stage_num: 0,
            map_file_name: "".to_string(),
            stage_file_name: "".to_string(),
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
    selector
}
